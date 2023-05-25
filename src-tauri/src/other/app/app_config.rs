//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-21 22:26:30
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-21 22:26:31
//! 
//! `FilePath        ` : /src-tauri/src/other/app/app_config.rs
//! 
//! ## Description  : 
//! App的配置管理


use std::{fs::File, io::Write};

use serde::{Serialize, Deserialize};

use crate::other::{chaos::{file_name::{FileName, FilePath}, version_migration::{RJson, Mig}}, path::app_path::AppPath, init::init_file::InitFile};

pub mod app_user_list;

/// App配置文件的根
pub type AppConfigRJson = RJson<AppConfigRJson_0>;


#[derive(Debug,Clone,Default,Serialize, Deserialize)]
#[serde(default)] 
struct AppConfigRJson_0 {
    app_user_list: self::app_user_list::AppUserListJson,
}

impl FileName for AppConfigRJson_0 {
    fn get_file_name() -> &'static str {
        "app_config.json"
    }
}
impl FilePath for AppConfigRJson_0 {
    fn get_file_path()->std::path::PathBuf {
        AppPath::AppConfig.get_path()
    }
}
impl Mig for AppConfigRJson_0 {
    fn get_version()->usize {
        0
    }
    
    fn _old_version(now_version:usize)->Self {
        todo!()
    }
}


