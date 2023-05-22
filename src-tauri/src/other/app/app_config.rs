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


use std::fs::File;

use crate::other::{chaos::file_name::{FileName, FilePath}, path::app_path::AppPath, init::init_file::InitFile};

pub mod app_user_list;

/// App配置文件的根
pub struct AppConfigRJson {
    app_user_list: app_user_list::AppUserListJson,
}

impl FileName for AppConfigRJson {
    fn get_file_name() -> &'static str {
        "app_config.json"
    }
}
impl FilePath for AppConfigRJson {
    fn get_file_path()->std::path::PathBuf {
        AppPath::AppConfig.get_path()
    }
}
impl InitFile for AppConfigRJson {
    fn init_file() {
        match File::open(AppConfigRJson::get_file_position()) {
            Ok(v) => {
                //todo写入json
            },
            Err(e) =>{
                //todo创建文件
            },
        }
    }
}

