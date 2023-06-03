//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-30 16:11:48
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-31 14:44:52
//! 
//! `FilePath        ` : /src-tauri/src/sqlite/table/example.rs
//! 
//! ## Description  : 
//! 一个 例子 而已,不应该参与业务



use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub use crate::sqlite::schema::example::dsl::example as TableExample;//可恶, pub use as 居然不参与补全
use crate::sqlite::sql_type::date_time::DateTime;

#[derive(Debug)]
#[derive(Queryable, Selectable,Insertable,Deserialize, Serialize)]
#[diesel(table_name = crate::sqlite::schema::example)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Example {
    pub id: i32,
    pub text : String,
    pub real : f32,
    pub blob : Vec<u8>,
    pub integer : i32,
    pub boolean : bool,
    pub timestamp : DateTime,
}



#[cfg(test)]
mod test {

    use chrono::NaiveDate;

    use crate::sqlite::{get_db_pool_from_env, sql_type::date_time::DateTime};
    use crate::sqlite::schema::example::dsl::*;
    use diesel::RunQueryDsl;

    use super::Example;
    fn get_example()->Example{
        Example{
            timestamp:DateTime::new(
                NaiveDate::from_ymd_opt(2016, 7, 8)
                .unwrap().
                and_hms_milli_opt(9, 10, 11,123)
                .unwrap()),
            id: 0,
            text: Default::default(),
            real: Default::default(),
            blob: Default::default(),
            integer: Default::default(),
            boolean: Default::default(),
            
        }
    }
    #[test]
    fn test_timestamp(){
        let pool =get_db_pool_from_env();
        let mut db = pool.get().unwrap();
        let a = get_example();
        diesel::insert_into(example)
            .values(a)
            .execute(&mut db)
            .unwrap();
        let vec = example
            .load::<Example>(&mut db)
            .unwrap();
        println!("{vec:#?}");
    }
}