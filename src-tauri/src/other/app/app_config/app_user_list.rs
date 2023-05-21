//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-21 22:28:46
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-21 22:28:47
//! 
//! `FilePath        ` : /src-tauri/src/other/app/app_config/app_user_list.rs
//! 
//! ## Description  : 
//! 保存 App 中的用户列表
use std::{path::PathBuf, collections::HashMap, ops::{Deref, DerefMut}};

use serde::{Deserialize, Serialize};
// 我们需要保证 高内聚低耦合 , 所以 不要在为了方便 而调用 这内部的结构体(或者说 本不应该服务你的结构体)
// 例如 UserName(String) 很简单,是吧, 但是它只服务这里,所以你在其他地方需要 用户名 的话,请在重新定义一个
// 事实上,为了可读性,你应该需要保持 这样的缩进定义, 因为我写多重定义的宏,写失败了🥹🥹🥹

/// App 存储 用户列表
#[derive(Debug,Clone,Default,Serialize, Deserialize)]
pub struct AppUserList{
    user_count:i32,
    user_list:HashMap<UserName,UserFolderPath>
}
    /// 用户名
    #[derive(   Debug,Clone,Default,
                PartialEq, Eq, Hash,
                Serialize, Deserialize)]
    pub struct UserName(String);
        impl Deref for UserName {
            type Target=String;
            fn deref(&self) -> &Self::Target {                
                &self.0
            }
        }
        impl DerefMut for UserName {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    /// 用户文件夹路径
    #[derive(   Debug,Clone,Default,
                Serialize, Deserialize)]
    pub struct UserFolderPath(PathBuf);
        impl Deref for UserFolderPath {
            type Target=PathBuf;
            fn deref(&self) -> &Self::Target {
                &self.0
            } 
        }
        impl DerefMut for UserFolderPath {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }










