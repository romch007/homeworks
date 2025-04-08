use std::str::FromStr;

use diesel::ConnectionResult;
use diesel_async::pooled_connection::bb8;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::ManagerConfig;
use diesel_async::AsyncPgConnection;
use futures_util::future::BoxFuture;
use futures_util::FutureExt;

pub type Pool = bb8::Pool<AsyncPgConnection>;

pub async fn create_database(database_url: &str, tls: bool) -> Pool {
    let mgr = if tls {
        let mut config = ManagerConfig::default();
        config.custom_setup = Box::new(establish_tls_connection);

        AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(database_url, config)
    } else {
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url)
    };

    bb8::Pool::builder()
        .test_on_check_out(true)
        .build(mgr)
        .await
        .expect("cannot create postgres connection pool")
}

fn establish_tls_connection(database_url: &str) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
    use rustls_platform_verifier::ConfigVerifierExt;

    let fut = async {
        // We first set up the way we want rustls to work.
        let rustls_config = rustls::ClientConfig::with_platform_verifier();
        let tls_config = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let mut config =
            tokio_postgres::Config::from_str(database_url).expect("invalid postgres url");
        config.ssl_mode(tokio_postgres::config::SslMode::Require);

        let (client, conn) = config
            .connect(tls_config)
            .await
            .map_err(|e| diesel::ConnectionError::BadConnection(e.to_string()))?;

        AsyncPgConnection::try_from_client_and_connection(client, conn).await
    };
    fut.boxed()
}
