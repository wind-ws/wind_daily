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

use crate::other::{chaos::file_name::{FileName, FilePath}, path::app_path::AppPath, init::init_file::InitFile};

pub mod app_user_list;

/// App配置文件的根
#[derive(Debug,Clone,Default,Serialize, Deserialize)]
#[serde(default)] 
pub struct AppConfigRJson {
    app_user_list: self::app_user_list::AppUserListJson,
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
            Ok(_) => {
                // 我认为 文件读取不应该出现在这里...虽然很方便
            },
            Err(e) =>{
                // ! e 没有被精准处理,感觉没必要
                let mut file=File::create(AppConfigRJson::get_file_position()).unwrap();
                let json = serde_json::to_string(&AppConfigRJson::default()).unwrap();
                file.write_all(json.as_bytes()).unwrap();
            },
        }
    }
}

