// @generated automatically by Diesel CLI.

diesel::table! {
    example (id) {
        id -> Integer,
        name -> Text,
        age -> Integer,
        address -> Nullable<Text>,
        salary -> Nullable<Float>,
    }
}
