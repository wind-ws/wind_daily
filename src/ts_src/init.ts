
import {path} from "./twin"
import {rust_init} from "./twin/init";
import {init_rust_path} from "./twin/path";


/// 在App初始化前 运行的代码
export function app_init_before(){
    init_rust_path();
    rust_init();
}


/// 在App初始化后 运行的代码
export function app_init_after(){

}

