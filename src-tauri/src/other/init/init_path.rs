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

use std::path::PathBuf;

use crate::other::{path::{app_path::AppPath, core_path::CorePath}};


/// 我认为 每一个 目录都应该被管理, 就算只是一个小目录
/// 所以 每一个目录 都应该被一个结构代理, 而实现 InitPath 这个trait,来达到初始化
/// 每一层目录都应该单独定义,并且有意义
/// 只允许使用 [fs::create_dir] 去创建目录, 所以需要注意 父路径是否是先创建的
/// 这个初始化是用来 软件第一次被打开 的初始化
pub trait InitPath  {
    fn get_vec_paths()->Vec<PathBuf>;
    fn init_path(){
        Self::get_vec_paths().into_iter().for_each(
            |v| {
                println!("路径创建:\t{v:?}");//输出创建的全部路径
                if let Err(e) = std::fs::create_dir(v){
                    match e.kind(){
                        std::io::ErrorKind::AlreadyExists =>(),
                        _=> panic!("除了路径以存在,不应该出现其他错误")
                    }
                }
            }
        );
    }
}

/// 所有路径初始化的地方
pub(super) fn init_path(){
    let vec:Vec<fn ()> = vec![
        CorePath::init_path,
        AppPath::init_path
    ];
    
    vec.into_iter().for_each(|v|v());
}
