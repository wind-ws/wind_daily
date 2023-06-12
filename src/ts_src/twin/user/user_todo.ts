import {Command, invoke} from "../../invoke";
import {DateTime} from "../../type/datetime";

export enum CommandMark {
    AddTodo             = "AddTodo",
    GetAllTodo          = "GetAllTodo",
    GetTodoById         = "GetTodoById",
    GetTodoByIsVisible  = "GetTodoByIsVisible",
    UpdataTodo          = "UpdataTodo",
}
export default {AddTodo,GetTodo,UpdataTodo};


export type Todo = {
    id          :number,
    is          :boolean,
    is_visible  :boolean,
    title       :string,
    priority    :Priority,
    father_id   ?:number,
    remind_time ?:DateTime,
    create_time :DateTime,
    done_time   ?:DateTime,
}

export enum Priority {
    Indifferent = "Indifferent",
    Ordinary    = "Ordinary",
    Normal      = "Normal",
    Urgent      = "Urgent",
    Haunted     = "Haunted",
}

/// 返回未完成状态的 颜色
export function get_priority_color_undone(self:Priority):string{//要求返回 #000000 格式,而非class名
    switch (self) {//todo 降低色彩亮度,变为暗色
        case Priority.Indifferent   : return "#52525b";//灰色 //zinc-600
        case Priority.Ordinary      : return "#059669";//绿色 //emerald-600
        case Priority.Normal        : return "#0284c7";//蓝色 //sky-600
        case Priority.Urgent        : return "#d97706";//黄色 //amber-600
        case Priority.Haunted       : return "#b91c1c";//红色 //red-700
    }
}

/// 返回完成状态的 颜色
export function get_priority_color_done(self:Priority):string{//要求返回 #000000 格式,而非class名
    switch (self) {//todo 降低色彩亮度,变为暗色
        case Priority.Indifferent   : return "#52525b";//灰色 //zinc-600
        case Priority.Ordinary      : return "#059669";//绿色 //emerald-600
        case Priority.Normal        : return "#0284c7";//蓝色 //sky-600
        case Priority.Urgent        : return "#d97706";//黄色 //amber-600
        case Priority.Haunted       : return "#b91c1c";//红色 //red-700
    }
}



namespace AddTodo{
    export type AddTodo = {
        title:string
        priority:Priority
        father_id?:number
        remind_time?:DateTime
    }
    export function add_todo(data:AddTodo) {
        return invoke<AddTodo,CommandMark>(Command.user_todo_command,[
            CommandMark.AddTodo,
            data
        ])
    }
}

namespace GetTodo{
    
    export function get_all_todo():Promise<Todo[]>{
        return invoke<null,CommandMark,Todo[]>(Command.user_todo_command,[
            CommandMark.GetAllTodo,
            null
        ])
    }
    export function get_todo_by_id(id:number):Promise<Todo>{
        return invoke<number,CommandMark,Todo>(Command.user_todo_command,[
            CommandMark.GetTodoById,
            id
        ])
    }
    
    export function get_todo_by_is_visible(is_visible:boolean):Promise<Todo[]>{
        return invoke<boolean,CommandMark,Todo[]>(Command.user_todo_command,[
            CommandMark.GetTodoByIsVisible,
            is_visible
        ])
    }
    
}

namespace UpdataTodo{
    
    
    /// 更新会直接根据json中的id进行更新,必须保证id的存在
    export function updata_todo(json:Todo){
        return invoke<Todo,CommandMark>(Command.user_todo_command,[
            CommandMark.UpdataTodo,
            json
        ])
    }
}