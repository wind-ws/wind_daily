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


// todo 实在是想不出来完美的解决方案~

use serde::{Serialize, Deserialize};

/// 这是一个包装类型,它可以包装,需要迭代更新的数据结构
#[derive(Debug,Clone,Default,Serialize, Deserialize)]
struct VersionMigration<D> {
    new_version:i32, // 当前的版本 , 以防应用长期不更新,出现 中间版本 ,从0开始,代表_struct元组中的0结构,若是1则从0迁移到1,若是n则从0->1->...->n-1->n
    #[serde(skip)]
    _struct:(i32,f32,D),//改成数组试试
    data:D
}

trait Mig {
    type OldData;
    type NewData;
    fn updata(old_data:Self::OldData)->Self::NewData;
} 


struct Data<D> {
    data:D
}

/// new => next
/// ```
/// match Option<V> {
///     None => (),
///     Some(v) => next
/// }
/// ``` 
struct _A<V> {
    new_version:V, // 当前的版本 , 以防应用长期不更新,出现 中间版本
    next_version:Option<V>,//应该更新到的版本
}

trait Migration {

}

/*  版本 数据结构&目录结构 变更的各种情况
    
    ---
    程序数据 - 存储数据
    ---
    1. 当前版本 == 最新版本
        无事可做
    2. 当前版本 -> 最新版本
    3. 当前版本 -> 中间版本... -> 最新版本

 */
