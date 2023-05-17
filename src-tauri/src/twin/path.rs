//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-16 14:30:45
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-16 14:31:22
//! 
//! `FilePath        ` : /wind_daily/src-tauri/src/twin/path.rs
//! 
//! ## Description  : 
//! 通过Ts获取基本目录


use std::{path::{PathBuf, Path}};
use serde::Deserialize;
use tauri::{App, Manager};

use crate::tauri_install_everything;

#[derive(Deserialize, Debug )]
struct AppAllBasePath {
    app_cache_dir_path : PathBuf,
    app_config_dir_path : PathBuf,
    app_data_dir_path : PathBuf,
    app_local_data_dir_path : PathBuf,
    app_log_dir_path : PathBuf,
    audio_dir_path : PathBuf,
    cache_dir_path : PathBuf,
    config_dir_path : PathBuf,
    data_dir_path : PathBuf,
    // desktop_path : PathBuf,
    document_dir_path : PathBuf,
    download_dir_path : PathBuf,
    // executable_dir_path : PathBuf,
    // font_dir_path : PathBuf,
    // home_dir_path : PathBuf,
    local_data_dir_path : PathBuf,
    picture_dir_path : PathBuf,
    public_dir_path : PathBuf,
    resource_dir_path : PathBuf,
    // runtime_dir_path : PathBuf,
    // template_dir_path : PathBuf,
    video_dir_path : PathBuf,
}

/// 注意的是,[App,Log]在2.0.0版本中将被移除,也就是下个版本,因此 我已经剔除
pub enum BaseDirectory {
    Audio,
    Cache,
    Config,
    Data,
    LocalData,
    // Desktop,
    Document,
    Download,
    // Executable,
    // Font,
    // Home,
    Picture,
    Public,
    // Runtime,
    // Template,
    Video,
    Resource,
    AppConfig,
    AppData,
    AppLocalData,
    AppCache,
    AppLog
}

impl BaseDirectory {
    /// 获取你需要的[Path]
    pub fn get_base_path(self)->&'static Path{
        unsafe {
            match self {
                BaseDirectory::Audio => ALL_BASE_PATH.as_ref().unwrap().audio_dir_path.as_path(),
                BaseDirectory::Cache => ALL_BASE_PATH.as_ref().unwrap().cache_dir_path.as_path(),
                BaseDirectory::Config => ALL_BASE_PATH.as_ref().unwrap().config_dir_path.as_path(),
                BaseDirectory::Data => ALL_BASE_PATH.as_ref().unwrap().data_dir_path.as_path(),
                BaseDirectory::LocalData => ALL_BASE_PATH.as_ref().unwrap().local_data_dir_path.as_path(),
                // BaseDirectory::Desktop => ALL_BASE_PATH.as_ref().unwrap().desktop_path.as_path(),
                BaseDirectory::Document => ALL_BASE_PATH.as_ref().unwrap().document_dir_path.as_path(),
                BaseDirectory::Download => ALL_BASE_PATH.as_ref().unwrap().download_dir_path.as_path(),
                // BaseDirectory::Executable => ALL_BASE_PATH.as_ref().unwrap().executable_dir_path.as_path(),
                // BaseDirectory::Font => ALL_BASE_PATH.as_ref().unwrap().font_dir_path.as_path(),
                // BaseDirectory::Home => ALL_BASE_PATH.as_ref().unwrap().home_dir_path.as_path(),
                BaseDirectory::Picture => ALL_BASE_PATH.as_ref().unwrap().picture_dir_path.as_path(),
                BaseDirectory::Public => ALL_BASE_PATH.as_ref().unwrap().public_dir_path.as_path(),
                // BaseDirectory::Runtime => ALL_BASE_PATH.as_ref().unwrap().runtime_dir_path.as_path(),
                // BaseDirectory::Template => ALL_BASE_PATH.as_ref().unwrap().template_dir_path.as_path(),
                BaseDirectory::Video => ALL_BASE_PATH.as_ref().unwrap().video_dir_path.as_path(),
                BaseDirectory::Resource => ALL_BASE_PATH.as_ref().unwrap().resource_dir_path.as_path(),
                BaseDirectory::AppConfig => ALL_BASE_PATH.as_ref().unwrap().app_config_dir_path.as_path(),
                BaseDirectory::AppData => ALL_BASE_PATH.as_ref().unwrap().app_data_dir_path.as_path(),
                BaseDirectory::AppLocalData => ALL_BASE_PATH.as_ref().unwrap().app_local_data_dir_path.as_path(),
                BaseDirectory::AppCache => ALL_BASE_PATH.as_ref().unwrap().app_cache_dir_path.as_path(),
                BaseDirectory::AppLog => ALL_BASE_PATH.as_ref().unwrap().app_log_dir_path.as_path(),
            }
        }
        // 这path模块的代码,敲起来和搬砖一样,累死了~ 🥱
    }
}

static mut ALL_BASE_PATH:Option<AppAllBasePath> = None;


pub fn event_listening_event_modify_path(app:&mut App)->Result<(), Box<dyn std::error::Error>>{
    let mut id: tauri::EventHandler;
    id= app.listen_global("event_modify_path",
    move|event|{
        let json = event.payload().unwrap();//它不应该失败,若失败则需要检查 Ts是否成功传入Path值
        let app_all_path: AppAllBasePath = serde_json::from_str(json).unwrap();
        unsafe{
            // println!("{app_all_path:#?}");
            ALL_BASE_PATH = Some(app_all_path);
        }
        // app.unlisten(id);
        // todo:(5) 事件完成后 取消监听
    });
    Ok(())
}


tauri_install_everything!{
    let ta;
    ta.setup(event_listening_event_modify_path)
}