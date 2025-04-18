// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::*;

    homeworks (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        due_date -> Nullable<Timestamptz>,
        title -> Varchar,
        description -> Varchar,
        done -> Bool,
        textsearchable_index_col -> Tsvector,
        subject_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::*;

    subjects (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        hex_color -> Nullable<Varchar>,
    }
}

diesel::joinable!(homeworks -> subjects (subject_id));

diesel::allow_tables_to_appear_in_same_query!(
    homeworks,
    subjects,
);
