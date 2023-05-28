//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-22 12:15:52
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-22 12:16:31
//! 
//! `FilePath        ` : /src-tauri/src/other/user/user_config.rs
//! 
//! ## Description  : 
//! 每一个单独的用户文件夹配置管理

use std::{path::PathBuf, sync::RwLock};

use serde::{Serialize, Deserialize};

use crate::other::{chaos::{version_migration::{RJson, Mig}, file_name::{FileName, FilePath}}, app::app_config::{ AppConfigRJson}, path::user_path::UserPath};

pub mod theme;

// lazy_static!{
//     pub static ref USER_CONFIG_RJSON_LOCK: RwLock<&'static mut UserConfigRJson> = {
//         // UserConfigRJson::updata().into()//todo 没能成功被创建,可能是 死循环 或 死锁
//         // RwLock::new(&USER_CONFIG_RJSON)
//     };
//     // pub static ref USER_CONFIG_RJSON:UserConfigRJson = {
//     //     UserConfigRJson::updata()
//     // };
// }


/// User配置文件的根
pub type UserConfigRJson = RJson<UserConfigRJson0>;
static mut USER_CONFIG_RJSON:Option<RwLock<UserConfigRJson>> =None;
impl UserConfigRJson {
    fn init() {
        unsafe {
            if let None = USER_CONFIG_RJSON {
                USER_CONFIG_RJSON = Some(RwLock::new(UserConfigRJson::updata()));
            }
        }
    }
    pub fn get_mut_lock()->&'static mut RwLock<UserConfigRJson>{
        Self::init();
        unsafe {
            USER_CONFIG_RJSON.as_mut().unwrap()
        }
    }
    pub fn get_lock()->&'static RwLock<UserConfigRJson>{
        Self::init();
        unsafe { 
            USER_CONFIG_RJSON.as_ref().unwrap() 
        }
    }
}


#[derive(Debug,Clone,Default,Serialize, Deserialize)]
pub struct UserConfigRJson0{
    // theme:
}
impl FileName for UserConfigRJson0 {
    fn get_file_name() -> &'static str {
        "user_config.json"
    }
}
impl FilePath for UserConfigRJson0 {
    fn get_file_path()->std::path::PathBuf {
        // let lock = APP_CONFIG_RJSON.read().unwrap();//! 死锁_156845
        let lock = AppConfigRJson::get_lock().read().unwrap();
        UserPath::Config.get_path(&lock.get_active_user().path)
    }
}
impl Mig for UserConfigRJson0 {
    fn get_version()->usize {
        0
    }
    
    fn _old_version(now_version:usize)->(Self, PathBuf) {
        todo!()
    }
}
