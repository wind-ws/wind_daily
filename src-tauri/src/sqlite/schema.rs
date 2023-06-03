// @generated automatically by Diesel CLI.

diesel::table! {
    config (id) {
        id -> Integer,
        key -> Text,
        json -> Text,
    }
}

diesel::table! {
    example (id) {
        id -> Integer,
        text -> Text,
        real -> Float,
        blob -> Binary,
        integer -> Integer,
        boolean -> Bool,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    state (id) {
        id -> Integer,
        key -> Text,
        json -> Text,
    }
}

diesel::table! {
    todo (id) {
        id -> Integer,
        is -> Bool,
        title -> Text,
        create_time -> Timestamp,
        done_time -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    config,
    example,
    state,
    todo,
);
