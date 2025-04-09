mod controllers;
mod db;
mod errors;
mod models;
mod schema;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{http::StatusCode, routing::get_service};
use serde::Deserialize;
use tower_http::services::{ServeDir, ServeFile};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Deserialize)]
struct Config {
    database_url: String,

    addr: Option<IpAddr>,
    port: Option<u16>,
}

#[derive(Debug, Clone)]
struct AppState {
    pool: db::Pool,
}

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().init();

    let config = envy::from_env::<Config>().expect("cannot deserialize env");

    let sockaddr = SocketAddr::new(
        config.addr.unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
        config.port.unwrap_or(8080),
    );

    db::run_migrations(&config.database_url).await;

    let pool = db::create_database(&config.database_url).await;

    let state = AppState { pool };

    let handle_svc_error =
        |_| async move { (StatusCode::INTERNAL_SERVER_ERROR, "internal server error") };

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", controllers::router())
        .nest_service(
            "/assets",
            get_service(ServeDir::new("./dist/assets")).handle_error(handle_svc_error),
        )
        .fallback_service(
            get_service(ServeFile::new("./dist/index.html")).handle_error(handle_svc_error),
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
