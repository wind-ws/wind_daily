//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-22 12:29:13
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-22 12:29:16
//! 
//! `FilePath        ` : /src-tauri/src/other/chaos/file_name.rs
//! 
//! ## Description  : 
//! Json 文件管理

use std::path::{PathBuf};


/// 一切 文件根结构 需要实现它 
/// 获取文件名
pub trait FileName {
    fn get_file_name() -> &'static str;

}
/// 获取文件路径
/// 如果一个文件根结构被实现它,那么这个文件理应只属于这个地方,而非 还可以在其他地方创建它
pub trait FilePath : FileName{
    fn get_file_path()->PathBuf;
    fn get_file_position()->PathBuf{
        Self::get_file_path().join(Self::get_file_name())
    }
}
