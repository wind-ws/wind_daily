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
//! App的核心目录结构
//! 你不应该直接在其他地方创建目录, 而是应该在这里创建后 在其他地方应用



use std::{path::{PathBuf}, fs};

use crate::{twin::path::BaseDirectory, other::init::init_file::InitPath};


/// 管理 App 的核心目录结构
pub enum AppPath {
    Root,//以这个为App的根目录
    DefaultUser,//默认的 用户文件夹 存储路径, 即 默认创建的用户文件夹放在这里
    App,
    AppData,
    AppConfig,
}

impl AppPath {
    pub fn get_path(&self)->PathBuf {
        match self {
            AppPath::Root => BaseDirectory::AppData.get_base_path().to_path_buf(),
            AppPath::DefaultUser => AppPath::Root.get_path().join("user"),
            AppPath::App => AppPath::Root.get_path().join("app"),
            AppPath::AppData => AppPath::Root.get_path().join("app/app_data"),
            AppPath::AppConfig => AppPath::Root.get_path().join("app/app_config"),
        }
    }
}
impl InitPath for AppPath {
    fn init_path() {
        let vec_path = vec![
            AppPath::Root, //虽然它已经存在,但是万一未来发生变动,也免又写一遍
            AppPath::DefaultUser,
            AppPath::App,
            AppPath::AppData,
            AppPath::AppConfig,
        ];
        vec_path.into_iter().for_each(
            |v| {
                if let Err(e) = fs::create_dir(v.get_path()){
                    
                }
            }
        );
    }
}


