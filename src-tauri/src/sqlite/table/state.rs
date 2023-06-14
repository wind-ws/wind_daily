
use diesel::dsl::exists;
use diesel::prelude::*;
use diesel::select;
use serde::{Deserialize, Serialize};
use crate::sqlite::table::*;
use diesel_autoincrement_new_struct::apply;
use diesel_autoincrement_new_struct::NewInsertable;


#[apply(NewInsertable!)]
#[derive(Queryable, Selectable,Deserialize, Serialize)]
#[diesel(table_name = crate::sqlite::schema::state)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct State {
    pub id   : i32    ,
    pub key  : String ,
    pub json : String ,//todo 好像还要做一个 json结构变化后迁移的 抽象
}


trait StateJson
where
    Self:serde::Serialize+serde::de::DeserializeOwned{
    fn get_key()-> &'static str;
    /// 从State表中获取对应key的json结构
    fn get_json(db:&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>)->Self{
        let s:String = TableState
            .filter(ColState::key.eq(Self::get_key()))
            .select(ColState::json)
            .first::<String>(db).unwrap();
        serde_json::from_str(&s).unwrap()
    }
    /// 方便的用 . 调用 Self::get_json
    fn get_json_(&self,db:&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>)->Self{
        Self::get_json(db)
    }
    /// 非必要,优先选择调用 `set_json`
    fn _insert_json(&self,db:&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>){
        diesel::insert_into(TableState)
            .values(NewState{
                // id:0,//自动增加,不用关心
                key:Self::get_key().to_string(),
                json:serde_json::to_string(self).unwrap()
            })
            .execute(db)
            .unwrap();
    }
    /// 非必要,优先选择调用 `set_json`
    fn _updata_json(&self,db:&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>){
        diesel::update(TableState
                .filter(ColState::key.eq(Self::get_key().to_string())))
            .set(ColState::json.eq(serde_json::to_string(self).unwrap()))
            .execute(db)
            .unwrap();
    }
    /// 如果存在key,那么updata ,不存在则 insert
    fn set_json(&self,db:&mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>){
        let a =select(exists(
                TableState.filter(ColState::key.eq(Self::get_key().to_string()))))
            .get_result::<bool>( db);
        match a {
            Ok(v) => {
                if v {
                    println!("更新TableState-key:{}",Self::get_key());
                    self._updata_json(db);
                }else {
                    println!("插入TableState-key:{}",Self::get_key());
                    self._insert_json(db);
                }
            },
            Err(_) => panic!("谁会知道为什么这里会panic呢~"),
        }
    }
}
