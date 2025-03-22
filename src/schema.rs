// @generated automatically by Diesel CLI.

diesel::table! {
    homeworks (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
