mod controllers;
mod db;
mod errors;
mod models;
mod schema;
mod utils;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{http::StatusCode, routing::get_service};
use color_eyre::eyre::Context;
use serde::Deserialize;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::EnvFilter;
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
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("homeworks=info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let config = envy::from_env::<Config>().wrap_err("cannot deserialize env")?;

    let sockaddr = SocketAddr::new(
        config.addr.unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
        config.port.unwrap_or(8080),
    );

    db::run_migrations(&config.database_url)
        .await
        .wrap_err("cannot run migrations")?;

    let pool = db::create_database(&config.database_url)
        .await
        .wrap_err("cannot create db pool")?;

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
        .wrap_err("cannot bind socket address")?;

    tracing::info!("listening on {sockaddr}");

    axum::serve(listener, router)
        .await
        .wrap_err("cannot serve http")?;

    Ok(())
}
