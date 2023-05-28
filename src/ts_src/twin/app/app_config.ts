import { invoke } from '@tauri-apps/api/tauri'


export enum CommandMark {
    CreateNewUser="CreateNewUser",
}
type Data<D> = {mark:string,data:D};
function get_data<D>(mark:CommandMark,data:D):Data<D>{
    return {mark,data}
}
// todo 包装 invoke 让他可以被限制, 可读性高的和Rust交互

namespace CreateNewUser{
    type CreateNewUserData={
        name:string,
        path?:string
    };
    
    export function create_new_user(name:string,path?:string){
        return invoke('app_config_command',get_data<CreateNewUserData>(
            CommandMark.CreateNewUser,
            {name,path}))
    }
}


export {CreateNewUser};
