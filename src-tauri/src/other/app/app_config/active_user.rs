

use std::{path::{Path, PathBuf}, any::Any};

use serde::{Deserialize, Serialize};

use crate::other::{user::user_config::USER_CONFIG_RJSON, chaos::version_migration::{RJson, Mig}};


/// 启动开启的用户 (上一次使用的用户)  
/// 也是正在使用的用户
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct ActiveUser {
    pub name: String,
    pub path: PathBuf,
}


impl ActiveUser {
    
    /// 刷新所有用户状态
    pub(super) fn refresh_all_user_states(){
        USER_CONFIG_RJSON.write().unwrap().refresh();
        // 一定要刷新, 否则 切换用户后,还是用的原来的用户状态
    }
}