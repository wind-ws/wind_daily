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
    habit (id) {
        id -> Integer,
    }
}

diesel::table! {
    plan (id) {
        id -> Integer,
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
    task (id) {
        id -> Integer,
    }
}

diesel::table! {
    todo (id) {
        id -> Integer,
        is -> Bool,
        is_visible -> Bool,
        title -> Text,
        priority -> Text,
        father_id -> Nullable<Integer>,
        remind_time -> Nullable<Timestamp>,
        create_time -> Timestamp,
        done_time -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    config,
    example,
    habit,
    plan,
    state,
    task,
    todo,
);
