//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-30 22:06:46
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-31 13:33:44
//! 
//! `FilePath        ` : /src-tauri/src/other/user/user_db.rs
//! 
//! ## Description  : 
//! 用户数据库控制 
//! diesel这个框架好像目前只能配置一个数据库(迁移配置),所以 App仍然用文件,User改成数据库

use std::path::PathBuf;

use diesel::{SqliteConnection};

use diesel::r2d2::{ ConnectionManager, Pool, PooledConnection};

use crate::other::app::app_config::AppConfigRJson;
use crate::other::path::user_path::UserPath;
use crate::sqlite::migrations::run_migrations;


pub static DB_NAME: &'static str = "user.sqlite3";

/// 获取当前活动用户的db文件路径
pub fn get_db_file_path()->PathBuf{
    let lock = AppConfigRJson::get_lock().read().unwrap();
    let user_root_path = &lock.get_active_user().unwrap().path;
    let user_db_path = UserPath::Db.get_path(user_root_path);
    let db_file_path = user_db_path.join(DB_NAME);
    db_file_path
}


// todo Pool 配置还不是很懂
// todo r2d2 文档: https://docs.diesel.rs/master/diesel/r2d2/index.html
static mut DB:Option<Pool<ConnectionManager<SqliteConnection>>> = None;
pub struct UserDb;
impl UserDb {
    /// 若是None则初始化
    fn init(){
        unsafe{
            if let &None = &DB{
                Self::refresh();
            }
        }
    }
    /// 重新创建 连接池 ,一般是在 活动用户修改后运行此函数
    pub fn refresh(){
        unsafe {
            //? 似乎不需要 关闭之前的连接池,并且等待之前的连接池工作完毕
            dotenvy::dotenv().ok();
            let manager = 
                if cfg!(test) {
                println!("\n 正在连接 Test数据库 \n");
                // 在实体机上如何观察 数据库的数据?
                // 使用 Android Studio 去找到目录即可,提前是实体机已经连接电脑
                // 这十分不方便,目前还没有其他更好的方法
                let database_url = std::env::var("DATABASE_URL").unwrap();
                let manager = ConnectionManager::<SqliteConnection>::new(database_url);
                manager
            } else {
                ConnectionManager::<SqliteConnection>::new(get_db_file_path().to_str().unwrap())
            };
            let pool = Pool::builder()
                .build(manager)
                .expect("Failed to create pool.");
            let mut db = pool.get().unwrap();
            run_migrations(&mut db);//执行一次迁移
            DB = Some(pool);
        }
    }
    /// 获取完整的 db pool
    pub fn get()->&'static Pool<ConnectionManager<SqliteConnection>>{
        Self::init();
        unsafe { 
            DB.as_ref().unwrap() 
        }
    }
    /// 简简单单的获取db
    pub fn get_db(&self)->Result<PooledConnection<ConnectionManager<SqliteConnection>>,r2d2::Error>{
        Self::get().get()
    }
}

