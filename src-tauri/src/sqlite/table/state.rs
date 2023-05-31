
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub use crate::sqlite::schema::state::dsl::state as TableState;

#[derive(Queryable, Selectable,Insertable,Deserialize, Serialize)]
#[diesel(table_name = crate::sqlite::schema::state)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct State {
    pub id: i32,
    pub key: String,
    pub json: String
}
