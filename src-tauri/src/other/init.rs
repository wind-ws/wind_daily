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

use crate::other::chaos::version_migration::example::{MY_JSON};

pub mod init_path;



pub fn init(){
    init_path::init_path();
    println!("{:?}",*MY_JSON);
}

