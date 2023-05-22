//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-20 17:22:01
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-20 17:23:31
//! 
//! `FilePath        ` : /src-tauri/src/other/struct_change_control.rs
//! 
//! ## Description  : 
//! 结构变化控制
//! 若未来数据结构改变,则需要去迁移旧数据


// todo : 等会设计

trait Updata<To> {
    
    fn updata(self)->To;

}

trait Backtrack {
    fn backtrack();
    
}

