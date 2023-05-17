use tauri::{Builder, Wry};

pub mod twin;
pub mod other;



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

