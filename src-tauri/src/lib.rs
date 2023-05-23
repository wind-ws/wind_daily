#![allow(dead_code)]//太吵了,全局用用
use tauri::{Builder, Wry};

#[macro_use]
extern crate lazy_static;
pub mod twin;
pub mod other;
pub mod test;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut tauri:Builder<Wry> =tauri::Builder::default();
    tauri = tauri_install_everything_(tauri);
    tauri.run(tauri::generate_context!())
        .expect("error while running tauri application");
}

tauri_install_everything!{
    |no pub(super)
    |twin
}

