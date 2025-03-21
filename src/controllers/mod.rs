mod homeworks;
mod subjects;

use axum::extract::State;
use diesel_async::RunQueryDsl;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{errors::AppResult, AppState};

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = OK, description = "App is healthy"),
    )
)]
async fn health(State(state): State<AppState>) -> AppResult<()> {
    let mut conn = state.pool.get().await?;

    diesel::sql_query("SELECT 1").execute(&mut conn).await?;

    Ok(())
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/homeworks", homeworks::router())
        .nest("/subjects", subjects::router())
        .routes(routes!(health))
}
