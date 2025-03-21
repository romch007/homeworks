mod controllers;
mod db;
mod errors;
mod models;
mod s3;
mod schema;

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{http::StatusCode, routing::get_service};
use serde::Deserialize;
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Deserialize)]
struct Config {
    database_url: String,

    s3_access_key: String,
    s3_secret_key: String,
    s3_endpoint_url: String,
    s3_bucket: String,
    s3_region: String,

    addr: Option<IpAddr>,
    port: Option<u16>,
}

#[derive(Debug, Clone)]
struct AppState {
    pool: db::Pool,
    s3: aws_sdk_s3::Client,
    config: Arc<Config>,
}

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().init();

    let config = Arc::new(envy::from_env::<Config>().expect("cannot deserialize env"));

    let sockaddr = SocketAddr::new(
        config.addr.unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
        config.port.unwrap_or(8080),
    );

    let pool = db::create_database(&config.database_url).await;

    let s3 = s3::create_client(
        &config.s3_access_key,
        &config.s3_secret_key,
        &config.s3_endpoint_url,
        &config.s3_region,
    )
    .await;

    let state = AppState { pool, s3, config };

    let handle_svc_error =
        |_| async move { (StatusCode::INTERNAL_SERVER_ERROR, "internal server error") };

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", controllers::router())
        .nest_service(
            "/assets",
            get_service(ServeDir::new("./dist/assets")).handle_error(handle_svc_error),
        )
        .fallback_service(
            get_service(ServeDir::new("./dist/index.html")).handle_error(handle_svc_error),
        )
        .with_state(state)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

    let listener = tokio::net::TcpListener::bind(sockaddr)
        .await
        .expect("cannot bind socket address");

    tracing::info!("listening on {sockaddr}");

    axum::serve(listener, router)
        .await
        .expect("cannot serve http");
}
