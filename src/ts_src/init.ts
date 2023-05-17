
import {path} from "./twin"


/// 在App初始化前 运行的代码
export function app_init_before(){
    path.path_send_rust();
}


/// 在App初始化后 运行的代码
export function app_init_after(){

}

