

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub use crate::sqlite::schema::config::dsl::config as TableConfig;

#[derive(Queryable, Selectable,Insertable,Deserialize, Serialize)]
#[diesel(table_name = crate::sqlite::schema::config)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Config {
    pub id: i32,
    pub key: String,
    pub json: String
}