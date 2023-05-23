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

use crate::{ other::init::init_file::InitPath};

use super::core_path::CorePath;


/// 管理 App 的目录结构
pub enum AppPath {
    DefaultUser,//默认的 用户文件夹 存储路径, 即 默认创建的用户文件夹放在这里
    App,
    AppData,
    AppConfig,
}

impl AppPath {
    pub fn get_path(&self)->PathBuf {
        match self {
            AppPath::DefaultUser => CorePath::Now.get_path().join("user"),
            AppPath::App => CorePath::Now.get_path().join("app"),
            AppPath::AppData => CorePath::Now.get_path().join("app/app_data"),
            AppPath::AppConfig => CorePath::Now.get_path().join("app/app_config"),
        }
    }
}
impl InitPath for AppPath {
    fn init_path() {
        let vec_path = vec![
            AppPath::DefaultUser,
            AppPath::App,
            AppPath::AppData,
            AppPath::AppConfig,
        ];
        vec_path.into_iter().for_each(
            |v| {
                if let Err(e) = fs::create_dir(v.get_path()){
                    match e.kind(){
                        std::io::ErrorKind::AlreadyExists =>(),
                        _=> panic!("除了路径以存在,不应该出现其他错误")
                    }
                }
            }
        );
    }
}


