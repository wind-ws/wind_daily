//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-22 12:58:08
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-22 12:58:28
//! 
//! `FilePath        ` : /src-tauri/src/twin/debug/path_access.rs
//! 
//! ## Description  : 
//! 检查Path是否可以成进入

use std::{fs::File, path::{PathBuf}};

use crate::{twin::path::BaseDirectory};


#[tauri::command]
fn is_path_access() -> Result<(), String>{
    let path = BaseDirectory::AppData.get_base_path();
    let mut path = PathBuf::from(path);
    path.push("abc.txt");
    println!("{path:?}");
    if let Err(e) = File::create(path) {
        println!("{e:?}");
        return Err(e.to_string());
    }
    Ok(())
}

