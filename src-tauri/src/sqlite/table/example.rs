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

#[derive(Queryable, Selectable,Insertable,Deserialize, Serialize)]
#[diesel(table_name = crate::sqlite::schema::example)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Example {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub address:Option<String>,
    pub salary:Option<f32>,
}