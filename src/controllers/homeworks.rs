use aws_sdk_s3::{presigning::PresigningConfig, types::ObjectCannedAcl};
use axum::{
    extract::{Path, State},
    Json,
};
use diesel::prelude::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    errors::{not_found, AppResult, BoxedAppError},
    models, AppState,
};

const TAG: &str = "Homeworks";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_homeworks, create_homework))
        .routes(routes!(find_homework, update_homework, delete_homework))
        .routes(routes!(prepare_upload))
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
) -> AppResult<Json<Vec<models::HomeworkWithSubject>>> {
    use crate::schema::homeworks::dsl::*;
    use crate::schema::subjects;

    let mut query = homeworks.left_join(subjects::table).into_boxed();

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
async fn find_homework(
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

#[derive(Debug, Deserialize, utoipa::ToSchema)]
struct PrepareUploadRequest {
    #[schema(examples("slides.pdf"))]
    pub filename: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
struct PrepareUploadResponse {
    pub file_id: u32,

    #[schema(
        examples("https://s3.xx-xxx.scw.cloud/homeworks/slides.pdf?x-id=PutObject&X-Amz-Algorithm=xxx&X-Amz-Credential=xxxx&X-Amz-Date=xxxx&X-Amz-Expires=xx&X-Amz-SignedHeaders=xxxxx&X-Amz-Signature=xxxxx")
    )]
    pub upload_url: String,

    #[schema(examples("PUT"))]
    pub upload_method: String,

    #[schema(example = json!({"x-amz-acl": "public-read"}))]
    pub upload_headers: HashMap<String, String>,
}

/// Prepares a file upload for the given homework
#[utoipa::path(
    delete,
    path = "/{id}/prepare-upload",
    tag = TAG,
    responses(
        (status = OK, body = PrepareUploadResponse),
        (status = NOT_FOUND, description = "The homework does not exist")
    ),
    params(
        ("id", description = "Id of the homework"),
    )
)]
async fn prepare_upload(
    State(state): State<AppState>,
    Path(target_id): Path<u32>,
    Json(payload): Json<PrepareUploadRequest>,
) -> AppResult<Json<PrepareUploadResponse>> {
    use crate::schema::files;
    use crate::schema::homeworks::dsl::*;

    let mut conn = state.pool.get().await?;

    let resp = conn
        .transaction::<_, BoxedAppError, _>(|mut conn| {
            {
                async move {
                    let homework = homeworks
                        .find(target_id as i32)
                        .first::<models::Homework>(&mut conn)
                        .await?;

                    let new_file = models::NewFile {
                        s3_key: format!("{}-{}", target_id, payload.filename),
                        name: payload.filename,
                        homework_id: homework.id,
                    };

                    let file = diesel::insert_into(files::table)
                        .values(&new_file)
                        .returning(models::File::as_returning())
                        .get_result(&mut conn)
                        .await?;

                    let presigned_config = PresigningConfig::expires_in(Duration::from_secs(60))?;

                    let presigned_req = state
                        .s3
                        .put_object()
                        .bucket(&state.config.s3_bucket)
                        .key(&new_file.s3_key)
                        .acl(ObjectCannedAcl::PublicRead)
                        .presigned(presigned_config)
                        .await?;

                    let upload_url = presigned_req.uri().to_string();
                    let upload_method = presigned_req.method().to_string();
                    let upload_headers = HashMap::from_iter(
                        presigned_req
                            .headers()
                            .map(|(k, v)| (k.to_string(), v.to_string())),
                    );

                    Ok(PrepareUploadResponse {
                        file_id: file.id as u32,
                        upload_url,
                        upload_method,
                        upload_headers,
                    })
                }
            }
            .scope_boxed()
        })
        .await?;

    Ok(Json(resp))
}
