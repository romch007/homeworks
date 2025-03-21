use axum::{
    extract::{Path, Query, State},
    Json,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    errors::{not_found, AppResult},
    models, AppState,
};

const TAG: &str = "Subjects";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_subjects, create_subject))
        .routes(routes!(find_subject, update_subject, delete_subject))
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
struct ListSubjectsParams {
    search: Option<String>,
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
    Query(params): Query<ListSubjectsParams>,
) -> AppResult<Json<Vec<models::Subject>>> {
    use crate::schema::subjects::dsl::*;

    let mut query = subjects.into_boxed();

    if let Some(search) = params.search {
        let q = format!("%{search}%");

        query = query.filter(name.ilike(q));
    }

    let mut conn = state.pool.get().await?;

    let results = query.load::<models::Subject>(&mut conn).await?;

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
) -> AppResult<Json<models::Subject>> {
    use crate::schema::subjects::dsl::*;

    let mut conn = state.pool.get().await?;

    let subject = subjects
        .find(target_id as i32)
        .first::<models::Subject>(&mut conn)
        .await?;

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
) -> AppResult<Json<models::Subject>> {
    use crate::schema::subjects;

    let mut conn = state.pool.get().await?;

    let new_subject = diesel::insert_into(subjects::table)
        .values(&payload)
        .returning(models::Subject::as_returning())
        .get_result(&mut conn)
        .await?;

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
) -> AppResult<Json<models::Subject>> {
    use crate::schema::subjects;
    use crate::schema::subjects::dsl::*;

    let mut conn = state.pool.get().await?;

    let updated_subject = diesel::update(subjects::table)
        .filter(id.eq(target_id as i32))
        .set(&payload)
        .returning(models::Subject::as_returning())
        .get_result(&mut conn)
        .await?;

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
) -> AppResult<()> {
    use crate::schema::subjects::dsl::*;

    let mut conn = state.pool.get().await?;

    let deleted_rows = diesel::delete(subjects.filter(id.eq(target_id as i32)))
        .execute(&mut conn)
        .await?;

    if deleted_rows == 0 {
        return Err(not_found());
    }

    Ok(())
}
