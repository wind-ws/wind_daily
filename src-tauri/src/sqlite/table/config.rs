

use diesel::{prelude::*, select, dsl::exists};
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




trait ConfigJson
where
    Self:serde::Serialize+serde::de::DeserializeOwned{
    fn get_key()-> &'static str;
    fn get_json()->Self{
        use crate::sqlite::schema::config::dsl::*;
        let mut db=Db.get_db().unwrap();
        let s:String = config
            .filter(key.eq(Self::get_key()))
            .select(json)
            .first::<String>(&mut db).unwrap();
        serde_json::from_str(&s).unwrap()
    }
    fn _insert_json(&self,db:&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>){
        use crate::sqlite::schema::config::dsl::*;
        diesel::insert_into(config)
            .values(Config{
                id:0,//自动增加,不用关心
                key:Self::get_key().to_string(),
                json:serde_json::to_string(self).unwrap()
            })
            .execute(db)
            .unwrap();
    }
    fn _updata_json(&self,db:&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>){
        use crate::sqlite::schema::config::dsl::*;
        diesel::update(config.filter(json.eq(Self::get_key().to_string())))
            .set(json.eq(serde_json::to_string(self).unwrap()))
            .execute(db)
            .unwrap();
    }
    /// 如果存在key,那么updata.不存在则 insert
    fn set_json(&self){
        use crate::sqlite::schema::config::dsl::*;
        let mut db=Db.get_db().unwrap();
        let a =select(exists(config.filter(key.eq(Self::get_key().to_string()))))
            .get_result::<bool>(&mut db);
        match a {
            Ok(v) => {
                if v {
                    self._updata_json(&mut db);
                }else {
                    self._insert_json(&mut db);
                }
            },
            Err(_) => panic!("谁会知道为什么这里会panic呢~"),
        }
    }
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
    pub fn get_json<T>(&self)->T
    where
        T:serde::Serialize+serde::de::DeserializeOwned{
        use crate::sqlite::schema::config::dsl::*;
        let mut db=Db.get_db().unwrap();
        let s:String = config
            .filter(key.eq(self.get_key()))
            .select(json)
            .first::<String>(&mut db).unwrap();
        serde_json::from_str::<T>(&s).unwrap()
    }
    pub fn set_json<T>(&self,v_json:T)
    where
        T:serde::Serialize+serde::de::DeserializeOwned{
        use crate::sqlite::schema::config::dsl::*;
        let mut db=Db.get_db().unwrap();
        diesel::insert_into(config)
            .values(Config{
                id:0,//自动增加,不用关心
                key:self.get_key().to_string(),
                json:serde_json::to_string(&v_json).unwrap()
            })
            .execute(&mut db)
            .unwrap();
    }
}