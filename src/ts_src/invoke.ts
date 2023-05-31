import { invoke as tauri_invoke ,InvokeArgs} from '@tauri-apps/api/tauri'

/// 全部的命令
export enum Command {
    init_rust_path="init_rust_path",
    rust_init="rust_init",
    app_config_command="app_config_command",
    user_example_command="user_example_command",
}

type Data<M extends string,D> = [M,D];//[mark,data],写的更舒服

export function invoke<D,M extends string=string,T=never>(command:Command,data:Data<M,D>):Promise<T>{
    return tauri_invoke(command, {mark:data[0],data:data[1]})
}
