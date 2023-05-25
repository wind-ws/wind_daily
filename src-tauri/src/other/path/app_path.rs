//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-22 13:56:27
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-22 13:56:37
//! 
//! `FilePath        ` : /src-tauri/src/other/path/app_path.rs
//! 
//! ## Description  : 
//! App的目录结构




use std::{path::{PathBuf}, fs};

use crate::{ other::init::init_path::InitPath};

use super::core_path::CorePath;


/// 管理 App 的目录结构
pub enum AppPath {
    DefaultUser,//默认的 用户文件夹 存储路径, 即 默认创建的用户文件夹放在这里
    App,
    AppData,
    AppConfig,
    Test,//测试用的路径
}

impl AppPath {
    pub fn get_path(&self)->PathBuf {
        match self {
            AppPath::DefaultUser => CorePath::Now.get_path().join("user"),
            AppPath::App => CorePath::Now.get_path().join("app"),
            AppPath::AppData => CorePath::Now.get_path().join("app/app_data"),
            AppPath::AppConfig => CorePath::Now.get_path().join("app/app_config"),
            AppPath::Test => CorePath::Now.get_path().join("test"),
        }
    }
}
impl InitPath for AppPath {
    
    fn get_vec_paths()->Vec<PathBuf> {
        vec![
            AppPath::DefaultUser.get_path(),
            AppPath::App.get_path(),
            AppPath::AppData.get_path(),
            AppPath::AppConfig.get_path(),
            AppPath::Test.get_path()
        ]
    }
}


