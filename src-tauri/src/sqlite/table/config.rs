

use diesel::{prelude::*, select, dsl::exists};
use serde::{Deserialize, Serialize};

pub use crate::sqlite::schema::config::dsl::config as TableConfig;
use diesel_autoincrement_new_struct::apply;
use diesel_autoincrement_new_struct::NewInsertable;

pub mod theme;


#[apply(NewInsertable!)]
#[derive(Queryable, Selectable,Deserialize, Serialize)]
#[diesel(table_name = crate::sqlite::schema::config)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Config {
    pub id   : i32    ,
    pub key  : String ,
    pub json : String ,//todo 好像还要做一个 json结构变化后迁移的 抽象
}




trait ConfigJson
where
    Self:serde::Serialize+serde::de::DeserializeOwned{
    fn get_key()-> &'static str;
    /// 从Config表中获取对应key的json结构
    fn get_json(db:&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>)->Self{
        use crate::sqlite::schema::config::dsl::*;
        let s:String = config
            .filter(key.eq(Self::get_key()))
            .select(json)
            .first::<String>(db).unwrap();
        serde_json::from_str(&s).unwrap()
    }
    /// 方便的用 . 调用 Self::get_json
    fn get_json_(&self,db:&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>)->Self{
        Self::get_json(db)
    }
    /// 非必要,优先选择调用 `set_json`
    fn _insert_json(&self,db:&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>){
        use crate::sqlite::schema::config::dsl::*;
        diesel::insert_into(config)
            .values(NewConfig{
                // id:0,//自动增加,不用关心
                key:Self::get_key().to_string(),
                json:serde_json::to_string(self).unwrap()
            })
            .execute(db)
            .unwrap();
    }
    /// 非必要,优先选择调用 `set_json`
    fn _updata_json(&self,db:&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>){
        use crate::sqlite::schema::config::dsl::*;
        diesel::update(config.filter(key.eq(Self::get_key().to_string())))
            .set(json.eq(serde_json::to_string(self).unwrap()))
            .execute(db)
            .unwrap();
    }
    /// 如果存在key,那么updata ,不存在则 insert
    fn set_json(&self,db:&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>){
        use crate::sqlite::schema::config::dsl::*;
        let a =select(exists(config.filter(key.eq(Self::get_key().to_string()))))
            .get_result::<bool>( db);
        match a {
            Ok(v) => {
                if v {
                    println!("更新TableConfig-key:{}",Self::get_key());
                    self._updata_json(db);
                }else {
                    println!("插入TableConfig-key:{}",Self::get_key());
                    self._insert_json(db);
                }
            },
            Err(_) => panic!("谁会知道为什么这里会panic呢~"),
        }
    }
}



#[cfg(test)]
mod test{

    use crate::sqlite::{ get_db_pool_from_env};
    use super::{theme::ThemeJson, ConfigJson};


    /// 测试 trait ConfigJson 是否可以成功使用
    #[test]
    fn test_config_json(){
        let pool = get_db_pool_from_env();
        let mut db = pool.get().unwrap();

        let theme = ThemeJson {
            theme: "light".to_string()
        };
        theme.set_json(&mut db);
        let json = theme.get_json_(&mut db);
        println!("{json:#?}");
    }
}