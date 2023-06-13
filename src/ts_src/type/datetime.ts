/// 专注服务 Rust中的 DateTime(pub NaiveDateTime) 类型

import dayjs from 'dayjs';//dayjs中文文档:https://dayjs.gitee.io/zh-CN/


const format: string = "YYYY-MM-DD HH:mm:ss.SSS";

export type DateTime = string;


export function now(): DateTime {
    return dayjs().format(format)
}
