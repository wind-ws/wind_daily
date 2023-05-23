use crate::{tauri_install_everything, other};


#[tauri::command]
fn rust_init(){
    other::init::init();
}

tauri_install_everything!{
    let ta;
    ta.invoke_handler(tauri::generate_handler![rust_init])
}