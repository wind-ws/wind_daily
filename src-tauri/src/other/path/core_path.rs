//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-23 15:44:58
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-23 15:45:29
//! 
//! `FilePath        ` : /src-tauri/src/other/path/core_path.rs
//! 
//! ## Description  : 
//! App的核心目录结构

use std::path::PathBuf;

use crate::twin::path::BaseDirectory;


pub enum CorePath {
    Root,// App的根目录
    Now,// App当前版本使用的 目录,
}
impl CorePath {
    pub fn get_path(&self)->PathBuf{
        match self {
            CorePath::Root => BaseDirectory::AppData.get_base_path().to_path_buf(),
            CorePath::Now => BaseDirectory::AppData.get_base_path().to_path_buf()
                .join(Into::<&str>::into(VersionPath::_0_0_0)),
        }
    }
}



/// 不同版本的可能会 目录结构&数据结构&其他东西 发生变更 
/// 若发生巨大变更, 为了方便迁移旧数据到新版本 ,我们会从 ./旧版本 迁移到 ./新版本 这样的文件全部迁移
/// 以防万一,我们不会 删除旧版本 数据, 但旧版本不会在被更新
/// 只有需要迁移 才会创造新的 枚举项 例如:_0_1_0 (它只是个例子)
enum VersionPath {
    _0_0_0,
}
impl From<&str> for VersionPath {
    fn from(value: &str) -> Self {
        match value {
            "0.0.0" => VersionPath::_0_0_0,
            _ => panic!("Version path not found")
        }
    }
}
impl From<VersionPath> for &str {
    fn from(value: VersionPath) -> Self {
        match value {
            VersionPath::_0_0_0 => "0.0.0",
        }
    }
}

