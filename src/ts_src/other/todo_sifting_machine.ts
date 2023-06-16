/// 服务todo的筛选机

import { DateTime } from "../type/datetime";



// 筛选器... 要求可以存储和命名
// 每一个筛选器都有独立的状态
// 通过筛选器 来显示列表
// 列表都是通过筛选器生成
// 筛选器可以筛选: 标签,是否完成,是否隐藏,根据优先级,设置排序规则和可拖动规则

// 筛选机 会加载一组 Todo (符合筛选要求的)

// 并且生成 对应的 排序列表状态,它会被存入 state表

// todo 完善 筛选机 的抽象
// ! 先把0.1发布后再来写它吧

type _f = {//函数们


};

/// 一个筛选机的设置状态
type TodoSiftingMachineSetting = {
    name: string,//筛选机 名
    sort:{//排序规则

    },
    sifiting:{//筛选规则
        invisible?:{
            load_number:number,//每次加载不可见todo的数量
            between_date:{//只加载这个日期范围的 不可见todo 
                start:DateTime,
                end:DateTime,
            },
        }
        load_done: boolean,//true 加载完成的todo
        load_undone:boolean,//true 加载未完成的todo
        load_prieroity:boolean,//true
        //todo...
    }
};

/// 一个筛选机
type TodoSiftingMachine = {};

export const default_sifting_machine: TodoSiftingMachine = {};
