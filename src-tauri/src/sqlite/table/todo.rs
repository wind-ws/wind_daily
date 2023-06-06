
use diesel::*;
use serde::*;
use diesel_autoincrement_new_struct::apply;
use diesel_autoincrement_new_struct::NewInsertable;

use crate::sqlite::sql_type::date_time::DateTime;


#[apply(NewInsertable!)]
#[derive(Debug,Queryable,Selectable,Deserialize, Serialize)]
#[diesel(table_name = crate::sqlite::schema::todo)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id          : i32              ,
    pub is          : bool             ,
    pub is_visible  : bool             ,
    pub title       : String           ,
    pub e_todo_type : String /*ETodoType*/  ,
    pub create_time : DateTime         ,
    pub done_time   : Option<DateTime> ,
}


#[derive(Debug)]
enum ETodoType {
    None
}
//todo 为 ETodotype实现 FromSql和ToSql

