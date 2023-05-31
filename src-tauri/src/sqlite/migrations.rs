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

use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");//迁移的目录

/// 迁移函数的文档: https://docs.rs/diesel_migrations/2.0.0/diesel_migrations/struct.HarnessWithOutput.html
pub fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>)  {

    connection.run_pending_migrations(MIGRATIONS).unwrap();

}