use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    errors::{InternalErrExt, NotFoundExt},
    models, AppState,
};

const TAG: &str = "Subjects";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_subjects, create_subject))
        .routes(routes!(find_subject, update_subject, delete_subject))
}

/// Retrieves all the subjects
#[utoipa::path(
    get,
    path = "/",
    tag = TAG,
    responses(
        (status = OK, body = [models::Subject])
    )
)]
async fn list_subjects(
    State(state): State<AppState>,
) -> Result<Json<Vec<models::Subject>>, StatusCode> {
    use crate::schema::subjects::dsl::*;

    let mut query = subjects.into_boxed();

    let mut conn = state.pool.get().await.map_internal_err()?;

    let results = query
        .load::<models::Subject>(&mut conn)
        .await
        .map_internal_err()?;

    Ok(Json(results))
}

/// Retrieves a specific subject
#[utoipa::path(
    get,
    path = "/{id}",
    tag = TAG,
    responses(
        (status = OK, body = models::Subject),
        (status = NOT_FOUND, description = "The subject does not exist")
    ),
    params(
        ("id", description = "Id of the subject"),
    )
)]
async fn find_subject(
    State(state): State<AppState>,
    Path(target_id): Path<u32>,
) -> Result<Json<models::Subject>, StatusCode> {
    use crate::schema::subjects::dsl::*;

    let mut conn = state.pool.get().await.map_internal_err()?;

    let subject = subjects
        .find(target_id as i32)
        .first::<models::Subject>(&mut conn)
        .await
        .optional()
        .map_internal_err()?
        .map_not_found()?;

    Ok(Json(subject))
}

/// Creates a new subject
#[utoipa::path(
    post,
    path = "/",
    tag = TAG,
    responses(
        (status = OK, body = models::Subject)
    )
)]
async fn create_subject(
    State(state): State<AppState>,
    Json(payload): Json<models::NewSubject>,
) -> Result<Json<models::Subject>, StatusCode> {
    use crate::schema::subjects;

    let mut conn = state.pool.get().await.map_internal_err()?;

    let new_subject = diesel::insert_into(subjects::table)
        .values(&payload)
        .returning(models::Subject::as_returning())
        .get_result(&mut conn)
        .await
        .map_internal_err()?;

    Ok(Json(new_subject))
}

/// Updates a subject
#[utoipa::path(
    put,
    path = "/{id}",
    tag = TAG,
    responses(
        (status = OK, body = models::Subject),
        (status = NOT_FOUND, description = "The subject does not exist")
    ),
    params(
        ("id", description = "Id of the subject"),
    )
)]
async fn update_subject(
    State(state): State<AppState>,
    Path(target_id): Path<u32>,
    Json(payload): Json<models::UpdatedSubject>,
) -> Result<Json<models::Subject>, StatusCode> {
    use crate::schema::subjects;
    use crate::schema::subjects::dsl::*;

    let mut conn = state.pool.get().await.map_internal_err()?;

    let updated_subject = diesel::update(subjects::table)
        .filter(id.eq(target_id as i32))
        .set(&payload)
        .returning(models::Subject::as_returning())
        .get_result(&mut conn)
        .await
        .optional()
        .map_internal_err()?
        .map_not_found()?;

    Ok(Json(updated_subject))
}

/// Deletes a subject
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = TAG,
    responses(
        (status = OK),
        (status = NOT_FOUND, description = "The subject does not exist")
    ),
    params(
        ("id", description = "Id of the subject"),
    )
)]
async fn delete_subject(
    State(state): State<AppState>,
    Path(target_id): Path<u32>,
) -> Result<(), StatusCode> {
    use crate::schema::subjects::dsl::*;

    let mut conn = state.pool.get().await.map_internal_err()?;

    let deleted_rows = diesel::delete(subjects.filter(id.eq(target_id as i32)))
        .execute(&mut conn)
        .await
        .map_internal_err()?;

    if deleted_rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(())
}
