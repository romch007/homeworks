mod homework;
mod subject;

pub use self::homework::*;
pub use self::subject::*;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct HomeworkWithSubject {
    #[serde(flatten)]
    pub homework: Homework,

    pub subject: Option<Subject>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SubjectWithHomeworks {
    #[serde(flatten)]
    pub subject: Subject,

    pub homeworks: Vec<Homework>,
}
