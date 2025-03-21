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

const TAG: &str = "Homeworks";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_homeworks, create_homework))
        .routes(routes!(find_homework, update_homework, delete_homework))
}

/// Retrieves all the homeworks
#[utoipa::path(
    get,
    path = "/",
    tag = TAG,
    responses(
        (status = OK, body = [models::HomeworkWithSubject])
    )
)]
async fn list_homeworks(
    State(state): State<AppState>,
) -> Result<Json<Vec<models::HomeworkWithSubject>>, StatusCode> {
    use crate::schema::homeworks::dsl::*;
    use crate::schema::subjects;

    let mut query = homeworks.left_join(subjects::table).into_boxed();

    let mut conn = state.pool.get().await.map_internal_err()?;

    let results = query
        .load::<(models::Homework, Option<models::Subject>)>(&mut conn)
        .await
        .map_internal_err()?;

    let results = results
        .into_iter()
        .map(|(homework, subject)| models::HomeworkWithSubject { homework, subject })
        .collect::<Vec<_>>();

    Ok(Json(results))
}

/// Retrieves a specific homework
#[utoipa::path(
    get,
    path = "/{id}",
    tag = TAG,
    responses(
        (status = OK, body = models::HomeworkWithSubject),
        (status = NOT_FOUND, description = "The homework does not exist")
    ),
    params(
        ("id", description = "Id of the homework"),
    )
)]
async fn find_homework(
    State(state): State<AppState>,
    Path(target_id): Path<u32>,
) -> Result<Json<models::HomeworkWithSubject>, StatusCode> {
    use crate::schema::homeworks::dsl::*;
    use crate::schema::subjects;

    let mut conn = state.pool.get().await.map_internal_err()?;

    let (homework, subject) = homeworks
        .find(target_id as i32)
        .left_join(subjects::table)
        .first::<(models::Homework, Option<models::Subject>)>(&mut conn)
        .await
        .optional()
        .map_internal_err()?
        .map_not_found()?;

    Ok(Json(models::HomeworkWithSubject { homework, subject }))
}

/// Creates a new homework
#[utoipa::path(
    post,
    path = "/",
    tag = TAG,
    responses(
        (status = OK, body = models::Homework)
    )
)]
async fn create_homework(
    State(state): State<AppState>,
    Json(payload): Json<models::NewHomework>,
) -> Result<Json<models::Homework>, StatusCode> {
    use crate::schema::homeworks;

    let mut conn = state.pool.get().await.map_internal_err()?;

    let new_homework = diesel::insert_into(homeworks::table)
        .values(&payload)
        .returning(models::Homework::as_returning())
        .get_result(&mut conn)
        .await
        .map_internal_err()?;

    Ok(Json(new_homework))
}

/// Updates a homework
#[utoipa::path(
    put,
    path = "/{id}",
    tag = TAG,
    responses(
        (status = OK, body = models::Homework),
        (status = NOT_FOUND, description = "The homework does not exist")
    ),
    params(
        ("id", description = "Id of the homework"),
    )
)]
async fn update_homework(
    State(state): State<AppState>,
    Path(target_id): Path<u32>,
    Json(payload): Json<models::UpdatedHomework>,
) -> Result<Json<models::Homework>, StatusCode> {
    use crate::schema::homeworks;
    use crate::schema::homeworks::dsl::*;

    let mut conn = state.pool.get().await.map_internal_err()?;

    let updated_homework = diesel::update(homeworks::table)
        .filter(id.eq(target_id as i32))
        .set(&payload)
        .returning(models::Homework::as_returning())
        .get_result(&mut conn)
        .await
        .optional()
        .map_internal_err()?
        .map_not_found()?;

    Ok(Json(updated_homework))
}

/// Deletes a homework
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = TAG,
    responses(
        (status = OK),
        (status = NOT_FOUND, description = "The homework does not exist")
    ),
    params(
        ("id", description = "Id of the homework"),
    )
)]
async fn delete_homework(
    State(state): State<AppState>,
    Path(target_id): Path<u32>,
) -> Result<(), StatusCode> {
    use crate::schema::homeworks::dsl::*;

    let mut conn = state.pool.get().await.map_internal_err()?;

    let deleted_rows = diesel::delete(homeworks.filter(id.eq(target_id as i32)))
        .execute(&mut conn)
        .await
        .map_internal_err()?;

    if deleted_rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(())
}
