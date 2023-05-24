//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-20 17:22:01
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-23 21:09:28
//! 
//! `FilePath        ` : /src-tauri/src/other/chaos/version_migration.rs
//! 
//! ## Description  : 
//! 结构变化后, 完成数据迁移&迭代&更新
//! 若未来数据结构改变,则需要去迁移旧数据



// todo 设计一个包装类型, 要求实现
// todo 1. 自动更新数据类型,包括 结构版本迭代
// todo 2. 自动存取

use std::{ fmt::Debug, fs::File, io::Read};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use super::file_name::FilePath;

// 术语统一
// 存储版本号 : 用户手机上存储的 数据结构的实际版本号
// 最新版本号 : 数据结构最新的版本号,  若存储版本号不等于最新版本号 ,则需要更新



/// 这是一个包装类型,它可以包装,需要迭代更新的数据结构
/// 迭代需要 D 实现 Mig 来达到 
#[derive(Debug,Default,Clone,Serialize,Deserialize)]
pub struct RJson<D> 
    where D : Debug+Default+Clone+FilePath +Sized{
    // 当前的版本 , 以防应用长期不更新,出现 中间版本 ,从0开始,若是1则从0迁移到1,若是n则从0->1->...->n-1->n
    // 若 存储版本号 和 最新版本号不一致 则需要做出更新
    version:usize,
    data:D,//D类型是最新版本类型
}


impl<D> RJson<D> 
    where D : Debug+Default+Clone+Mig{
    
    fn updata(&mut self){
        let json = D::_updata(0);// 直到0为止,或搜索到now_version
        self.data = json;
        self.version = D::get_version();
    }
    
}

pub trait Mig 
    where Self: FilePath+Debug+Default+Clone+Sized+DeserializeOwned{
    /// 获取当前类型的 版本
    fn get_version()->usize;

    ///
    /// 苦思冥想,想出一个离谱的更新方法  
    /// 你需要这样写这个函数  
    /// ```
    /// 
    /// if now_version == 0 {//说明 now_version 仍然没有被搜索到,需要这个版本尝试搜索// 这里是处理 文件改名 或 文件移动到其他文件夹内
    ///     if let Ok(mut file) = File::open(Self::get_file_position()){
    ///         ...;//Ok : 说明找到版本
    ///     }
    /// }
    /// if now_version == Self::get_version() {//存储版本 与 当前类型版本 一致,停止向老版本传递,开始向新版本传递. 当然了,当前类型版本就可能上最新版本
    ///     ...;
    /// }else {
    ///     ...;//这里是 处理 当前类型版本不是存储版本,无法处理存储版本的数据,继续向老版本传递,直到上个版本返回数据,后对数据做迁移
    /// }
    /// ```
    /// now_version : 存储版本
    fn _updata(mut now_version:usize)->Self{
        if now_version == 0 {//说明 now_version 仍然没有被搜索到,需要这个版本尝试搜索
            if let Ok(mut file) = File::open(Self::get_file_position()){
                //Ok : 说明找到版本
                let mut s = String::new();
                file.read_to_string(&mut s).unwrap();
                let json:serde_json::Value = serde_json::from_str(&s).unwrap();
                let version:usize=json["version"].as_i64().unwrap() as usize;
                now_version = version;
            }
        }
        if now_version == Self::get_version() {//存储版本 与 类型版本 一致,停止向老版本传递,开始向新版本更新
            let mut file = File::open(Self::get_file_position()).unwrap();//版本一致必然获取
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();
            let json:RJson<Self> = serde_json::from_str::<RJson<Self>>(&s).unwrap();
            json.data
        }else {
            Self::_old_version(now_version)
        }
    }
    
    /// 声明 上一个版本
    /// 并且 处理 上一个版本 迁移 到 这个版本
    /// ```
    /// let old=Old::_updata(now_version);
    /// ...;// 迁移
    /// return now;//返回 迁移后的版本
    /// ```
    fn _old_version(now_version:usize)->Self;
    

}





/// 以下 完完全全 为模拟, 并非真实的路径和文件,并且不可运行
mod example {
    
    use std::{path::PathBuf, io::Read};

    use crate::other::chaos::file_name::FileName;

    use super::*;

    #[derive(Debug,Default,Clone,Serialize, Deserialize)]
    struct Old(i32);
    impl FileName for Old {
        fn get_file_name() -> &'static str {
            "old.json"
        }
    }
    impl FilePath for Old  {
        fn get_file_path()->std::path::PathBuf {
            Into::<PathBuf>::into("/old")
        }
    }
    impl Mig for Old {
        fn get_version()->usize {
            0
        }

        fn _updata(now_version:usize)->Self {
            //这是第一版,不用关心 now_version
            //能运行到这里,就说明, 存储类型还是 第一版的类型
            let mut file = File::open(Self::get_file_position()).unwrap();
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();
            let json:Self = serde_json::from_str(&s).unwrap();
            json
        }

        fn _old_version(now_version:usize)->Self {
            todo!()//没有上个版本,不用写
        }
    }
    
    #[derive(Debug,Default,Clone,Serialize, Deserialize)]
    struct New(String);
    impl FileName for New {
        fn get_file_name() -> &'static str {
            "new.json"
        }
    }
    impl FilePath for New  {
        fn get_file_path()->std::path::PathBuf {
            Into::<PathBuf>::into("/new")
        }
    }
    impl Mig for New {
        fn get_version()->usize {
            1
        }

        fn _old_version(now_version:usize)->Self {
            let old = Old::_updata(now_version);
            let mut now = Self::default();
            now.0 = old.0.to_string();
            now
        }

    
    }

    
}













