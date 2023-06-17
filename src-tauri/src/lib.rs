#![allow(dead_code)]//太吵了,全局用用
// #![feature(associated_type_defaults)]//算了,还是用稳定版吧
#![recursion_limit = "512"]

use tauri::{Builder, Wry};


#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel_autoincrement_new_struct;
#[macro_use]
extern crate diesel_derive_newtype;

pub mod twin;
pub mod other;
pub mod test;
pub mod sqlite;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut tauri:Builder<Wry> =tauri::Builder::default();
    tauri = install_command(tauri);
    tauri.run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// 因为 invoke_handler 这个函数只能运行一次... 所以全部 command 在这里安装
fn install_command(ta: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    ta.invoke_handler(tauri::generate_handler![
        crate::twin::path::init_rust_path,
        crate::twin::init::rust_init,
        crate::twin::app::app_config::app_config_command,
        crate::twin::user::user_example::user_example_command,
        crate::twin::user::user_todo::user_todo_command,

    ])
}



