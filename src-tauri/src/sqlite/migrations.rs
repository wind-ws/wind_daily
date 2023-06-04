//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-30 15:42:17
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-30 15:42:30
//! 
//! `FilePath        ` : /src-tauri/src/sqlite/migrations.rs
//! 
//! ## Description  : 
//! SQLite 迁移(更新&回溯) 模块
//! 这个模块是为 用户存储的数据库 迁移 (即 存储版本 和 最新版本 不同时,对本地更新到最新版本)
//! 
//! 本地代码编写 运行 
//! 创建一个迁移 `diesel migration generate name` name 是迁移名
//! todo 看看这些命令的细节
//! 进行 `diesel migration run`
//! 进行 `diesel migration redo`
//! 进行 `diesel migration revert`
//! 
//! 注意!!! : 一但版本发布, 迁移目录下的sql不应该再被修改, 只能创造新的迁移
//! 因为 修改已经发布后的sql,会导致 用户的存储版本 与 当前版本 不一致
//! 迁移应该避免 删除上个版本的旧迁移数据(表) ,而是 删除上上个版本的旧迁移数据 
//!     中间必须隔一个检查版本(检查期),以免数据丢失
//!     说白了,就是 留一手

use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");//迁移的目录

/// 迁移函数的文档: https://docs.rs/diesel_migrations/2.0.0/diesel_migrations/struct.HarnessWithOutput.html
pub fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>)  {

    connection.run_pending_migrations(MIGRATIONS).unwrap();

}