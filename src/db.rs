use diesel_async::pooled_connection::bb8;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type Pool = bb8::Pool<AsyncPgConnection>;

pub async fn create_database(database_url: &str) -> Pool {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    let pool = bb8::Pool::builder()
        .test_on_check_out(true)
        .build(config)
        .await
        .expect("cannot create postgres connection pool");

    pool
}
