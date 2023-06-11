import {Command, invoke} from "../../invoke";

export enum CommandMark {
    InsertValue="InsertValue",
}
export default {InsertValue};

namespace InsertValue {
    export type Example = {
        id: number,
        name: string,
        age: number,
        address?:string,
        salary?:number,
    };
    export function insert_value(data:Example) {
        return invoke<Example,CommandMark>(Command.user_example_command,[
            CommandMark.InsertValue,
            data
        ])
    }
}