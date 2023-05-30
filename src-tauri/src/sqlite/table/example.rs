


use diesel::prelude::*;

#[derive(Queryable, Selectable,Insertable)]
#[diesel(table_name = crate::sqlite::schema::example)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Example {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub address:Option<String>,
    pub salary:Option<f32>,
}