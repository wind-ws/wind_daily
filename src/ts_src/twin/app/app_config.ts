import {Command, invoke} from "../../invoke";

export enum CommandMark {
    CreateNewUser="CreateNewUser",
    GetActiveUser="GetActiveUser"
}

export {CreateNewUser,ActiveUser};


namespace CreateNewUser{
    type CreateNewUserData={
        name:string,
        path?:string
    };
    
    export function create_new_user(name:string,path?:string){
        return invoke<CreateNewUserData,CommandMark>(Command.app_config_command,[
            CommandMark.CreateNewUser,
            {name,path}
        ])
    }
}


namespace ActiveUser{
    type ActiveUser = {
        name:string,
        path?:string
    }
    export function get_active_user():Promise<ActiveUser>{
        return invoke<null,CommandMark,ActiveUser>(Command.app_config_command,[
            CommandMark.GetActiveUser,
            null
         ])
    }
}
