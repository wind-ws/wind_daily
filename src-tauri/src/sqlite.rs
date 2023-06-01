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
//! 注意 SQLite 只有: text,real,blob,integer,null 这5个类型
//! 是不是非常不够用? 好在diesel对sql语句做出独立的处理,让我们可以在sql中编写其他类型,后对应进SQLite中的5个类型
//! 并且它还会生成更加正确的 Rust类型,例如 bool(sql语句) 对应integer(SQLite),生成bool(Rust)
//! 
//! diesel作者钟爱PostgreSQL... 那就只能麻烦的编写代码了
//! 
//! 目前以知可用的有:
//! [SQL类型]   [diesel类型]    [Rust类型(diesel类型中有介绍,包括你需要开启的feature)]
//! text        Text
//! real        Float
//! blob        Binary
//! integer     Integer
//! boolean     Bool
//! timestamp   Timestamp   todo:使用chrono::NaiveDateTime时有点问题,存储的字符串,要求是数字
//! ...
//! text(json)  Text            自定义类型:要求实现FromSql和ToSql
//! 
//! 
//! 如果想舒舒服服的用 json ,那么你需要实现 有点复杂的东西,或者使用 我们写好的宏 `struct_sql_json`
//! 想要了解如何实现,见 以下库文档:
//!     https://docs.diesel.rs/master/diesel/serialize/index.html
//!     https://docs.diesel.rs/master/diesel/deserialize/index.html



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

