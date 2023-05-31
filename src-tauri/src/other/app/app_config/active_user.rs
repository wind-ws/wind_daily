

use std::{path::PathBuf};

use serde::{Deserialize, Serialize};




/// 启动开启的用户 (上一次使用的用户)  
/// 也是正在使用的用户
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct ActiveUser {
    pub name: String,
    pub path: PathBuf,
}


