import { invoke } from '@tauri-apps/api/tauri'


export enum CommandMark {
    CreateNewUser,
}
type Data<D> = {mark:string,data:D};
function get_data<D>(mark:string,data:D):Data<D>{
    return {mark,data}
}


namespace CreateNewUser{
    type CreateNewUserData={
        name:string,
        path?:string
    };
    
    export function create_new_user(name:string,path?:string){
        return invoke('app_config_command',get_data<CreateNewUserData>(
            "CreateNewUser",
            {name,path}))
    }
}


export {CreateNewUser};
