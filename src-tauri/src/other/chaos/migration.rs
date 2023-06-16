use std::ops::{Deref, DerefMut};


pub mod file_json;
pub mod sql_json;

// 结构 分流 和 合流
// 分流: 把结构A分为 B和C
// 合流: 把结构B和C 合为 A

/// 版本标识 的类型
/// 暂时未确定是否选择u32,可能选择str
type VersionMarkType = &'static str;


/// 包装类型 实现
trait Mig<D>
where
    Self: MigVersion,
    D:  MigData,{
    
    /// 执行迁移,并获取最新版本数据
    fn mig()->D{
        let now_version = Self::now_version();
        let my_version = D::MY_VERSION;

        if now_version > my_version{// 存储版本大于最新版本, 执行回溯
            D::down(now_version)
        }else if now_version < my_version{// 存储版本小于最新版本, 执行更新
            D::up(now_version)
        }else {//一毛一样,那么直接获取数据
            D::get_self()
        }
    }
}

/// 包装类型 实现
/// 最新版本由 被包装类型决定
trait MigVersion {
    /// 获取本地(存储)版本
    fn now_version()->VersionMarkType;
    
    // 使用的版本
    // const VERSION:u32;// 由 被包装的类型 提供使用的版本号

    // fn version<T:MigVersionData>()->u32{
    //     T::MY_VERSION
    // }

    //最新版本
    // const LATEST_VERSION:u32;
}


/// 版本结构体实现
trait MigVersionData{
    /// 当前结构体的实际数据
    type Data;
    
    /// 上一个版本
    type Old:MigUp+MigDown;
    /// 下一个版本
    type Next:MigUp+MigDown;

    /// 当前结构体的版本号
    /// 这更像一个标识符,若标识符相等则直接获取当前的类型
    /// 不相等,则继续 更新或回溯
    const MY_VERSION:VersionMarkType;

    // fn ref_data(&self)->&Self::Data;

    // fn mut_data(&mut self)->&mut Self::Data;
    
    /// 不基于 老版本或新版本 去创建一个自己
    /// 往往是直接从 目标数据 转为 自己 , 比如 从 文件数据(.json文件) 转为 json(Self)
    fn get_self()->Self;
}

/// 版本结构体实现
/// 更新版本
trait MigUp : MigVersionData + Sized {
    fn up(now_version:VersionMarkType)->Self{
        if now_version == Self::MY_VERSION {
            Self::get_self()
        } else {
            let old = Self::Old::up(now_version);
            Self::_up_from_old(old)
        }
    }
    /// 这里实现 从老版本到现在的版本, 不可手动调用它, 优先调用 `up`函数
    fn _up_from_old(old:Self::Old)->Self;
}

/// 版本结构体实现
/// 回溯版本
trait MigDown: MigVersionData + Sized  {
    fn down(now_version:VersionMarkType)->Self{
        if now_version == Self::MY_VERSION {
            Self::get_self()
        } else {
            let next = Self::Next::down(now_version);
            Self::_down_from_next(next)
        }
    }
    fn _down_from_next(_next:Self::Next)->Self{//往往不需要回溯,可以懒的去写它
        panic!("喔喉~你执行了回溯,却没有实现MigDown的_down_from_next")
    }
}

/// 简化一下,这是 被包装类型 必须实现的全部trait
trait MigData:Sized+MigVersionData+MigUp+MigDown{}

/// 若没有 Next版本或Old版本 ,那么可以用()代替
impl MigVersionData for (){
    type Data=();

    type Old=();

    type Next=();

    const MY_VERSION:VersionMarkType="()";

    fn get_self()->Self {
        panic!("你永远都不应该执行到这里,除非你的版本标识符搞错了")
    }
}
impl MigUp for () {
    fn _up_from_old(_old:Self::Old)->Self {
        panic!("你永远都不应该执行到这里,除非你的版本标识符搞错了")
    }
}
impl MigDown for () {
    fn _down_from_next(_next:Self::Next)->Self{
        panic!("你永远都不应该执行到这里,除非你的版本标识符搞错了")
    }
}

