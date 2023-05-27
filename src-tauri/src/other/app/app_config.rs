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


use std::{fs::File, io::Write, path::PathBuf, sync::RwLock};

use serde::{Serialize, Deserialize};

use crate::other::{chaos::{file_name::{FileName, FilePath}, version_migration::{RJson, Mig}}, path::app_path::AppPath};

use self::{active_user::ActiveUser, app_user_list::AppUserListJson};

pub mod app_user_list;
pub mod active_user;

lazy_static!{
    pub static ref APP_CONFIG_RJSON:RwLock<AppConfigRJson> ={
        AppConfigRJson::updata().into()
    };
}
/// App配置文件的根
pub type AppConfigRJson = RJson<AppConfigRJson0>;


#[derive(Debug,Clone,Default,Serialize, Deserialize)]
pub struct AppConfigRJson0 {
    pub app_user_list: AppUserListJson,
    /// 启动开启的用户 (上一次使用的用户)  
    /// 也是正在使用的用户
    /// 若它是None,会导致所有用户状态无法使用
    active_user:Option<ActiveUser>,
}

impl FileName for AppConfigRJson0 {
    fn get_file_name() -> &'static str {
        "app_config.json"
    }
}
impl FilePath for AppConfigRJson0 {
    fn get_file_path()->std::path::PathBuf {
        AppPath::AppConfig.get_path()
    }
}
impl Mig for AppConfigRJson0 {
    fn get_version()->usize {
        0
    }
    
    fn _old_version(now_version:usize)->(AppConfigRJson0, PathBuf) {
        todo!()
    }
}

impl AppConfigRJson0 {
    
    /// 切换活动用户
    pub fn switch_active_user(&mut self,active_user:ActiveUser){
        self.active_user = Some(active_user);
        ActiveUser::refresh_all_user_states();//刷新 所有 用户 文件状态
    }

    pub fn get_active_user(&self)->&ActiveUser{
        self.active_user.as_ref().unwrap() //调用这个函数的位置,它不应该不存在,既 它必须存在
    }
}

