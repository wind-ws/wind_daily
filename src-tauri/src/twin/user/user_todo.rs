use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value};


/// 只注册一个命令, 通过 symbol 来触发不同的业务
#[derive(Debug,Deserialize,Serialize)]
pub enum CommandMark {
    AddTodo,
    GetAllTodo,
    GetTodoById,
    GetTodoByIsVisible,
    UpdataTodo,
    RemoveTodoById,
}
/// 所有业务统一用一个 Error处理
#[derive(Debug, thiserror::Error,Serialize,Deserialize)]
pub enum CommandError {

}

type Res = Result<Value,CommandError>;

#[tauri::command]
pub fn user_todo_command(mark:CommandMark,data:Value)->Res{
    println!("命令触发:{mark:?}-->\n{data:#?}");
    match mark {
        CommandMark::AddTodo            => add_todo::add_todo(from_value(data).unwrap()),
        CommandMark::GetAllTodo         => get_todo::get_all_todo(),
        CommandMark::GetTodoById        => get_todo::get_todo_by_id(from_value(data).unwrap()),
        CommandMark::GetTodoByIsVisible => get_todo::get_todo_by_is_visible(from_value(data).unwrap()),
        CommandMark::UpdataTodo         => updata_todo::updata_todo(from_value(data).unwrap()),
        CommandMark::RemoveTodoById     => remove_todo::remove_todo_by_id(from_value(data).unwrap()),
    }
}

mod add_todo {
    use diesel::RunQueryDsl;

    use crate::{sqlite::{table::{todo::{priority::Priority, NewTodo, self}, self}, sql_type::date_time::DateTime}, other::user::user_db::UserDb};

    use super::*;

    #[derive(Debug,Deserialize,Serialize)]
    pub(super) struct AddTodo{
        pub title       : String           ,
        pub priority    : Priority         ,
        pub father_id   : Option<i32>      ,//暂时不用
        pub remind_time : Option<DateTime> ,//暂时不用
    }

    pub(super) fn add_todo(
        AddTodo{
            title,
            priority,
            father_id,
            remind_time}:AddTodo) -> Res {
        let table_v = NewTodo {
            is: false,
            is_visible: true,
            title,
            priority,
            father_id,
            remind_time,
            create_time: DateTime::now(),
            done_time: None,
        };
        diesel::insert_into(table::TableTodo)
            .values(table_v)
            .execute(&mut UserDb.get_db().unwrap())
            .unwrap();
        Ok(Value::Null)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_add_todo(){
            let add_todo_v = AddTodo{
                title: "吃饭".to_string(),
                priority: Priority::Normal,
                father_id: None,
                remind_time: None,
            };
            add_todo(add_todo_v).unwrap();
        }
    
    }
}

mod get_todo {
    use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};

    use crate::{sqlite::{table, schema::todo}, other::user::user_db::UserDb};

    use super::*;

    /// 获得所有的todo
    pub(super) fn get_all_todo()->Res{
        let vec: Vec<table::todo::Todo> = table::TableTodo
            .load::<table::todo::Todo>(&mut UserDb.get_db().unwrap())
            .unwrap();
        Ok(serde_json::to_value(vec).unwrap())
    }

    /// 根据id获取todo
    /// 传入的id必须保证它的存在,若不存在,软件会直接崩溃 (需要这样)
    pub(super) fn get_todo_by_id(id:i32)->Res{
        let json:table::todo::Todo=table::TableTodo
            .filter(table::ColTodo::id.eq(id))
            .first::<table::todo::Todo>(&mut UserDb.get_db().unwrap())
            .unwrap();
        Ok(serde_json::to_value(json).unwrap())
    }

    /// true  则返回 可见的全部todo
    /// false 则返回 不可见的全部todo
    pub(super) fn get_todo_by_is_visible(is_visible:bool)->Res{
        let vec:Vec<table::todo::Todo> = table::TableTodo
            .filter(table::ColTodo::is_visible.eq(is_visible))
            .load::<table::todo::Todo>(&mut UserDb.get_db().unwrap())
            .unwrap();
        Ok(serde_json::to_value(vec).unwrap())
    }

    #[cfg(test)]
    mod test{
        use super::*;
        
        #[test]
        fn test_get_all_todo() {
            let vec = get_all_todo().unwrap();
            println!("{vec:#?}");
        }
        
        #[test]
        fn test_get_todo_by_id(){
            let a = get_todo_by_id(1).unwrap();
            println!("{a:#?}");
        }
        
        #[test]
        fn test_get_todo_by_is_visible(){
            let vec = get_todo_by_is_visible(false).unwrap();
            println!("{vec:#?}");
            let vec = get_todo_by_is_visible(true).unwrap();
            println!("{vec:#?}");
        }
    
    
    }
}

mod updata_todo {
    use diesel::*;
    use crate::{sqlite::table::{todo::{Todo}, self}, other::user::user_db::UserDb};
    use super::*;
    
    /// 更新 todo 表
    /// 要保证id的存在
    pub(super) fn updata_todo(json:Todo)->Res{
        diesel::update(table::TableTodo
                .filter(
                    table::ColTodo::id.eq(json.id.clone()))
                )
            .set(json)
            .execute(&mut UserDb.get_db().unwrap())
            .unwrap();
        Ok(Value::Null)
    }

    #[cfg(test)]
    mod test {

        use crate::sqlite::{table::todo::priority::Priority, sql_type::date_time::DateTime};

        use super::*;

        #[test]
        fn test_update_todo() {
            let json = Todo {
                id: 1,
                is: false,
                is_visible: true,
                title: "Oh~".to_string(),
                priority: Priority::Urgent,
                father_id: None,
                remind_time: None,
                create_time: DateTime::now(),
                done_time: None,
            };
            updata_todo(json).unwrap();
        }
        
    }
}

mod remove_todo{
    use diesel::{QueryDsl,RunQueryDsl, ExpressionMethods};

    use crate::{sqlite::table, other::user::user_db::UserDb};

    use super::*;

    /// 根据id删除todo , 非必要优先选择隐藏
    pub(super) fn remove_todo_by_id(id:i32) -> Res{
        diesel::delete(table::TableTodo
                .filter(table::ColTodo::id.eq(id))
            )
            .execute(&mut UserDb.get_db().unwrap())
            .unwrap();
        Ok(Value::Null)
    }

    #[cfg(test)]
    mod test{
        use super::*;

        #[test]
        fn test_remove_todo(){
            remove_todo_by_id(1).unwrap();
        }
    }

}