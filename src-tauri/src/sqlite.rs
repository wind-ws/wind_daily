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