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

const TAG: &str = "Homeworks";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_homeworks, create_homework))
        .routes(routes!(get_homework, update_homework, delete_homework))
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
struct ListHomeworksParams {
    /// Search query
    search: Option<String>,
}

/// Retrieves all the homeworks
#[utoipa::path(
    get,
    path = "/",
    tag = TAG,
    params(
        ListHomeworksParams
    ),
    responses(
        (status = OK, body = [models::HomeworkWithSubject])
    )
)]
async fn list_homeworks(
    State(state): State<AppState>,
    Query(params): Query<ListHomeworksParams>,
) -> AppResult<Json<Vec<models::HomeworkWithSubject>>> {
    use crate::schema::homeworks::dsl::*;
    use crate::schema::subjects;

    let mut query = homeworks.left_join(subjects::table).into_boxed();

    if let Some(search) = params.search {
        let q = format!("%{search}%");

        query = query.filter(title.ilike(q.clone()).or(description.ilike(q)));
    }

    let mut conn = state.pool.get().await?;

    let results = query
        .load::<(models::Homework, Option<models::Subject>)>(&mut conn)
        .await?;

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
async fn get_homework(
    State(state): State<AppState>,
    Path(target_id): Path<u32>,
) -> AppResult<Json<models::HomeworkWithSubject>> {
    use crate::schema::homeworks::dsl::*;
    use crate::schema::subjects;

    let mut conn = state.pool.get().await?;

    let (homework, subject) = homeworks
        .find(target_id as i32)
        .left_join(subjects::table)
        .first::<(models::Homework, Option<models::Subject>)>(&mut conn)
        .await?;

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
) -> AppResult<Json<models::Homework>> {
    use crate::schema::homeworks;

    let mut conn = state.pool.get().await?;

    let new_homework = diesel::insert_into(homeworks::table)
        .values(&payload)
        .returning(models::Homework::as_returning())
        .get_result(&mut conn)
        .await?;

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
) -> AppResult<Json<models::Homework>> {
    use crate::schema::homeworks;
    use crate::schema::homeworks::dsl::*;

    let mut conn = state.pool.get().await?;

    let updated_homework = diesel::update(homeworks::table)
        .filter(id.eq(target_id as i32))
        .set(&payload)
        .returning(models::Homework::as_returning())
        .get_result(&mut conn)
        .await?;

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
) -> AppResult<()> {
    use crate::schema::homeworks::dsl::*;

    let mut conn = state.pool.get().await?;

    let deleted_rows = diesel::delete(homeworks.filter(id.eq(target_id as i32)))
        .execute(&mut conn)
        .await?;

    if deleted_rows == 0 {
        return Err(not_found());
    }

    Ok(())
}
