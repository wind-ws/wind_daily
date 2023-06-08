import {Command, invoke} from "../../invoke";

export enum CommandMark {
    AddTodo             = "AddTodo",
    GetAllTodo          = "GetAllTodo",
    GetTodoById         = "GetTodoById",
    GetTodoByIsVisible  = "GetTodoByIsVisible",
    UpdataTodo          = "UpdataTodo",
}
export {AddTodo};

export enum Priority {
    Indifferent = "Indifferent",
    Ordinary    = "Ordinary",
    Normal      = "Normal",
    Urgent      = "Urgent",
    Haunted     = "Haunted",
}


namespace AddTodo{
    export type AddTodo = {
        title:string
        priority:Priority
        father_id?:number
        remind_time?:string
    }
    export function add_todo(data:AddTodo) {
        return invoke<AddTodo,CommandMark>(Command.user_example_command,[
            CommandMark.AddTodo,
            data
        ])
    }
}

namespace GetTodo{

}
namespace UpdataTodo{

}