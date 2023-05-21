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

pub mod app_user_list;


pub struct AppConfig {
    app_user_list: app_user_list::AppUserList,
}