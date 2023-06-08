//! 一个操作 用户数据库中的example表的 例子

use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value};


/// 只注册一个命令, 通过 symbol 来触发不同的业务
#[derive(Debug,Deserialize,Serialize)]
pub enum CommandMark {
    InsertValue,
    
}
/// 所有业务统一用一个 Error处理
#[derive(Debug, thiserror::Error,Serialize,Deserialize)]
pub enum CommandError {

}

type Res = Result<Value,CommandError>;

#[tauri::command]
pub fn user_example_command(mark:CommandMark,data:Value)->Res{
    println!("命令触发:{mark:?}-->\n{data:#?}");
    match mark {
        CommandMark::InsertValue => insert_value::insert_value(from_value(data).unwrap())
    }
}

mod insert_value {
    use diesel::RunQueryDsl;

    use crate::{sqlite::{table::{example::{Example}, self}}, other::user::user_db::UserDb};

    use super::*;

    pub(super) fn insert_value(table:Example)->Res{
        let mut db = UserDb.get_db().unwrap();
        diesel::insert_into(table::TableExample)
            .values(&table)
            .execute(&mut db)
            .unwrap();
        Ok(Value::Null)
    }
}
