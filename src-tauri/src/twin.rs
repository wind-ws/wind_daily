//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-15 19:45:34
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-15 19:45:41
//! 
//! `FilePath        ` : /tauri-app/src-tauri/src/twin.rs
//! 
//! ## Description  : 
//! twin module 是专门用于与Ts做交互的  
//! Rust中的twin和Ts中的twin 要目录结构保持相等,文件名自然也得相等
//! 在这里Rust做后端数据处理等等...  
//! 而Ts与Rust连接后 赋予Vue能力 . Vue用于专门渲染前端

use crate::tauri_install_everything;

pub mod path;
pub mod debug;
pub mod config;

tauri_install_everything!{
    |path
}