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
//! SQLite 迁移&更新&回溯 模块


use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");


pub fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>)  {

    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS).unwrap();

}