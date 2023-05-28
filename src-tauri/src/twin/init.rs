use crate::{other};


#[tauri::command]
pub fn rust_init(){
    other::init::init();
}

