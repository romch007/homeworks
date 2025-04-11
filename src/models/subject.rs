use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
