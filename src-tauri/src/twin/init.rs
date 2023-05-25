use crate::{tauri_install_everything, other};


#[tauri::command]
pub fn rust_init(){
    other::init::init();
}

