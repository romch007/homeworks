// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Int4,
        name -> Varchar,
        s3_key -> Varchar,
        created_at -> Timestamptz,
        homework_id -> Int4,
    }
}

diesel::table! {
    homeworks (id) {
        id -> Int4,
        created_at -> Timestamptz,
        due_date -> Nullable<Timestamptz>,
        title -> Varchar,
        description -> Varchar,
        done -> Bool,
        subject_id -> Nullable<Int4>,
    }
}

diesel::table! {
    subjects (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(files -> homeworks (homework_id));
diesel::joinable!(homeworks -> subjects (subject_id));

diesel::allow_tables_to_appear_in_same_query!(
    files,
    homeworks,
    subjects,
);
