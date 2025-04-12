use std::str::FromStr;

use color_eyre::eyre::Context;
use diesel::ConnectionResult;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::bb8;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::ManagerConfig;
use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use tokio::task::spawn_blocking;

pub type Pool = bb8::Pool<AsyncPgConnection>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub async fn create_database(database_url: &str) -> color_eyre::Result<Pool> {
    let mut config = ManagerConfig::default();
    config.custom_setup = Box::new(establish_tls_connection);

    let mgr =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(database_url, config);

    let pool = bb8::Pool::builder()
        .test_on_check_out(true)
        .build(mgr)
        .await
        .wrap_err("cannot create postgres connection pool")?;

    Ok(pool)
}

pub fn establish_tls_connection(
    database_url: &str,
) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
    use rustls_platform_verifier::ConfigVerifierExt;

    let fut = async {
        // We first set up the way we want rustls to work.
        let rustls_config = rustls::ClientConfig::with_platform_verifier();
        let tls_config = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let config = tokio_postgres::Config::from_str(database_url).expect("invalid postgres url");

        let (client, conn) = config
            .connect(tls_config)
            .await
            .map_err(|e| diesel::ConnectionError::BadConnection(e.to_string()))?;

        AsyncPgConnection::try_from_client_and_connection(client, conn).await
    };
    fut.boxed()
}

pub async fn run_migrations(database_url: &str) -> color_eyre::Result<()> {
    tracing::info!("running pending migrations");

    let conn = establish_tls_connection(database_url)
        .await
        .wrap_err("cannot establish connection to postgres")?;

    let mut conn = AsyncConnectionWrapper::<AsyncPgConnection>::from(conn);

    spawn_blocking(move || {
        conn.run_pending_migrations(MIGRATIONS)
            .expect("cannot run pending migrations");
    })
    .await?;

    tracing::info!("pending migrations done");

    Ok(())
}
