

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{sqlite::schema::config, other::user::user_db::Db, from_to_sql_json};
pub use crate::sqlite::schema::config::dsl::config as TableConfig;

pub mod theme;


#[derive(Queryable, Selectable,Insertable,Deserialize, Serialize)]
#[diesel(table_name = crate::sqlite::schema::config)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Config {
    pub id: i32,
    pub key: String,
    pub json: String
}



pub enum ConfigKey {
    Theme
}
impl ConfigKey {
    pub fn get_key(&self) -> &'static str{
        match self {
            ConfigKey::Theme => "theme",
        }
    }
    pub fn get_json<T:serde::Serialize+serde::de::DeserializeOwned>(&self)->T{
        use crate::sqlite::schema::config::dsl::*;
        let mut db=Db.get_db().unwrap();
        let s:String = config
            .filter(key.eq(self.get_key()))
            .select(json)
            .first::<String>(&mut db).unwrap();
        serde_json::from_str::<T>(&s).unwrap()
    }
}