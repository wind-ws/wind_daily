//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-22 15:20:59
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-22 15:21:02
//! 
//! `FilePath        ` : /src-tauri/src/other/init.rs
//! 
//! ## Description  : 
//! 初始化模块



pub mod init_path;


/// 软件每次启动都会进行一次初始化
pub fn init(){
    init_path::init_path();
}

