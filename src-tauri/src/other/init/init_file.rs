//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-22 15:21:47
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-22 15:23:47
//! 
//! `FilePath        ` : /src-tauri/src/other/init/init_file.rs
//! 
//! ## Description  : 
//! 第一次启动 App 需要的初始化
//! 初始化 必要的目录和文件

use crate::other::{path::app_path::AppPath, app::app_config::AppConfigRJson};


/// 我认为 每一个 目录都应该被管理, 就算只是一个小目录
/// 所以 每一个目录 都应该被一个结构代理, 而实现 InitPath 这个trait,来达到初始化
/// 每一层目录都应该单独定义,并且有意义
/// 只允许使用 [fs::create_dir] 去创建目录, 所以需要注意 父路径是否是先创建的
pub trait InitPath  {
    fn init_path();
}

/// 我认为 每一个 文件都应该被管理, 就算只是一个小文件
/// 所以 每一个文件 都应该被一个结构代理, 而实现 InitFile 这个trait,来达到初始化
pub trait InitFile {
    fn init_file();
}


pub(super) fn init(){
    init_path();
    init_file();
}


fn init_path(){
    let vec:Vec<fn ()> = vec![
        AppPath::init_path
    ];
    
    vec.into_iter().for_each(|v|v());
}

fn init_file(){
    let vec:Vec<fn ()> = vec![
        AppConfigRJson::init_file
    ];

    vec.into_iter().for_each(|v|v());
}