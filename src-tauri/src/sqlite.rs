//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-30 15:41:29
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-31 22:59:37
//! 
//! `FilePath        ` : /src-tauri/src/sqlite.rs
//! 
//! ## Description  : 
//! SQLite 模块
//! 
//! 下面来介绍类型系统 (这个比较混乱)  
//! 首先是diesel框架的SQL类型文档:https://docs.diesel.rs/master/diesel/sql_types/index.html  
//! 这是diesel对SQLite的类型文档(这个不重要):https://docs.diesel.rs/master/diesel/sqlite/enum.SqliteType.html  
//! 
//! 注意 SQLite 只有: text,real,blob,integer 这4个类型
//! 是不是非常不够用? 好在diesel对sql语句做出独立的处理,让我们可以在sql中编写其他类型,后对应进SQLite中的4个类型
//! 并且它还会生成更加正确的 Rust类型,例如 bool(sql语句) 对应integer(SQLite),生成bool(Rust)
//! 
//! todo 确实有点乱,明天再看


use std::path::Path;

use diesel::{Connection, SqliteConnection};



pub mod table;
pub mod migrations;
pub mod schema;


/// 获取指定路径的db
pub fn get_db(path:&Path)->SqliteConnection{
    let db: SqliteConnection = diesel::SqliteConnection::establish(path.to_str().unwrap()).unwrap();
    db
}

