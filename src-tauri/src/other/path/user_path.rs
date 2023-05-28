//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-22 13:57:11
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-22 13:58:17
//! 
//! `FilePath        ` : /src-tauri/src/other/path/user_path.rs
//! 
//! ## Description  : 
//! User的目录结构
//! 你不应该直接在其他地方创建目录, 而是应该在这里创建后 在其他地方应用

use std::path::{PathBuf, Path};


pub enum UserPath {
    Config,
    Data
}


impl UserPath {

    /// 
    /// ## function description : 
    /// param  user_root_path [&Path] : 用户文件夹的根目录                  
    pub fn get_path(&self,user_root_path:&Path)-> PathBuf{
        match self {
            UserPath::Config => user_root_path.join("config"),
            UserPath::Data => user_root_path.join("data")
        }
    }

    /// 更新 用户文件夹的目录结构,也可用来创建目录结构    
    /// 若不存在,则创建  
    pub fn updata_user_path(user_root_path:&Path){
        vec![
            UserPath::Config.get_path(user_root_path),
            UserPath::Data.get_path(user_root_path)
        ].into_iter().for_each(
            |v|{
                if let Err(e) = std::fs::create_dir(&v){
                    match e.kind(){
                        std::io::ErrorKind::AlreadyExists =>{
                            println!("已存在路径: {v:?}");//输出已存在的全部路径
                        },
                        _=> panic!("除了路径以存在,不应该出现其他错误")
                    }
                }else {
                    println!("路径创建: {v:?}");//输出创建的全部路径
                }
            }
        );
        
    }

}
