use crate::schema::homeworks;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::Subject;

#[derive(Debug, Queryable, Identifiable, Selectable, Associations, Serialize, utoipa::ToSchema)]
#[diesel(table_name = crate::schema::homeworks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Subject))]
pub struct Homework {
    pub id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub subject_id: Option<i32>,
}

type HomeworkAllColumns = (
    homeworks::id,
    homeworks::created_at,
    homeworks::updated_at,
    homeworks::due_date,
    homeworks::title,
    homeworks::description,
    homeworks::done,
    homeworks::subject_id,
);

pub const HOMEWORK_ALL_COLUMNS: HomeworkAllColumns = (
    homeworks::id,
    homeworks::created_at,
    homeworks::updated_at,
    homeworks::due_date,
    homeworks::title,
    homeworks::description,
    homeworks::done,
    homeworks::subject_id,
);

#[derive(Debug, Insertable, Deserialize, utoipa::ToSchema)]
#[diesel(table_name = crate::schema::homeworks)]
pub struct NewHomework {
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub title: String,
    pub description: Option<String>,
    pub subject_id: Option<i32>,
}

#[derive(Debug, AsChangeset, Deserialize, utoipa::ToSchema)]
#[diesel(table_name = crate::schema::homeworks)]
pub struct UpdatedHomework {
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub subject_id: Option<i32>,
    pub done: Option<bool>,
}
