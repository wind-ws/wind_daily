//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-22 12:18:17
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-22 12:18:19
//! 
//! `FilePath        ` : /src-tauri/src/other/path.rs
//! 
//! ## Description  : 
//! 这个模块 才是 App 的全部路径管理 ,twin模块里path不用管它
//! 一切 路径 的管理
//! 你不应该直接在其他地方创建目录, 而是应该在这里创建后 在其他地方应用

/* 目前的路径结构是:
.
└── 0.0.0
    ├── app
    │   ├── app_config
    │   │   └── app_config.json
    │   └── app_data
    ├── test
    └── user
        └── abc
            ├── config
            │   └── user_config.json
            └── data
 */

// todo add log 
// todo 节约性能,把能静态的目录全部用静态代替,而非函数层级获取

pub mod core_path;
pub mod app_path;
pub mod user_path;

