mod homeworks;
mod subjects;

use axum::{extract::State, http::StatusCode};
use diesel_async::RunQueryDsl;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{errors::InternalErrExt, AppState};

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = OK, description = "App is healthy"),
    )
)]
async fn health(State(state): State<AppState>) -> Result<(), StatusCode> {
    let mut conn = state.pool.get().await.map_internal_err()?;

    diesel::sql_query("SELECT 1")
        .execute(&mut conn)
        .await
        .map_internal_err()?;

    state
        .s3
        .head_bucket()
        .bucket(&state.config.s3_bucket)
        .send()
        .await
        .map_internal_err()?;

    Ok(())
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/homeworks", homeworks::router())
        .nest("/subjects", subjects::router())
        .routes(routes!(health))
}
