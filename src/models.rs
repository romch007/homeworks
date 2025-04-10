use crate::schema::*;
use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, utoipa::ToSchema)]
#[diesel(table_name = crate::schema::homeworks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, utoipa::ToSchema)]
#[diesel(table_name = crate::schema::subjects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subject {
    pub id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub hex_color: Option<String>,
}

#[derive(Debug, Insertable, Deserialize, utoipa::ToSchema)]
#[diesel(table_name = crate::schema::subjects)]
pub struct NewSubject {
    pub name: String,
    pub hex_color: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize, utoipa::ToSchema)]
#[diesel(table_name = crate::schema::subjects)]
pub struct UpdatedSubject {
    pub name: Option<String>,
    pub hex_color: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct HomeworkWithSubject {
    #[serde(flatten)]
    pub homework: Homework,

    pub subject: Option<Subject>,
}
