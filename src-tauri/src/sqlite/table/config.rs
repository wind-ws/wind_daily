

use diesel::{prelude::*, sqlite::Sqlite};
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


/// 为json结构实现,方便快速存取
/// 专门服务 Config 表
trait ConfigKey<ST>
where
    Self: serde::Serialize + 
        serde::de::DeserializeOwned + 
        diesel::Queryable<ST,Sqlite>+
        'static{
    fn get_key() -> &'static str;
    fn get_json()->Self{
        use crate::sqlite::schema::config::dsl::*;
        let mut db=Db.get_db().unwrap();
        config
            .filter(key.eq(Self::get_key()))
            .select(json)
            .first::<Self>(&mut db).unwrap()
    }
}


