// @generated automatically by Diesel CLI.

diesel::table! {
    groups (auto_id) {
        auto_id -> Int8,
        external_id -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    students (auto_id) {
        auto_id -> Int8,
        external_id -> Varchar,
        name -> Varchar,
        group_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(students -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    students,
);
