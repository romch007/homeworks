use axum::{extract::State, http::header, response::IntoResponse};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use icalendar::{Calendar, Component, Event, EventLike};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{errors::AppResult, models, AppState};

const TAG: &str = "Homeworks";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(generate_icalendar))
}

/// Generates a ical calendar
#[utoipa::path(
    get,
    path = "/",
    tag = TAG,
)]
async fn generate_icalendar(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    use crate::schema::homeworks::dsl::*;
    use crate::schema::subjects;

    let mut calendar = Calendar::new();
    let mut calendar = calendar.name("Homeworks");

    let mut conn = state.pool.get().await?;

    let results = homeworks
        .left_join(subjects::table)
        .select((
            models::HOMEWORK_ALL_COLUMNS,
            Option::<models::Subject>::as_select(),
        ))
        .filter(due_date.is_not_null())
        .load::<(models::Homework, Option<models::Subject>)>(&mut conn)
        .await?;

    let results = results
        .into_iter()
        .map(|(homework, subject)| models::HomeworkWithSubject { homework, subject })
        .collect::<Vec<_>>();

    for res in results {
        let due_date_val = res.homework.due_date.expect("no due date");

        let mut summary = res.homework.title.clone();

        if let Some(subject) = res.subject {
            summary.push_str(" - ");
            summary.push_str(&subject.name);
        }

        let event = Event::new()
            .summary(&summary)
            .description(&res.homework.description)
            .starts(due_date_val)
            .ends(due_date_val + chrono::Duration::hours(1))
            .done();

        calendar = calendar.push(event);
    }

    let calendar = calendar.done();

    let res = calendar.to_string();

    Ok(([(header::CONTENT_TYPE, "text/calendar")], res))
}
