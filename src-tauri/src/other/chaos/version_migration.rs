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





use std::{ fmt::Debug, fs::{File, self, OpenOptions}, io::{Read, Write, BufReader},ops::{Deref,DerefMut}, path::{Path, PathBuf}, sync::RwLock};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{json, Value};
use super::file_name::FilePath;

// 术语统一
// 存储版本号 : 用户手机上存储的 数据结构的实际版本号
// 最新版本号 : 数据结构最新的版本号,  若存储版本号不等于最新版本号 ,则需要更新


/// 注意: 若未来我们使用了 tauri的异步功能, 那么这个结构需要保证多线程安全,即需要升级这个结构(添加锁),也可以选择用锁包装它(这是最好的选择)
/// 这是一个包装类型,它可以包装,需要迭代更新的数据结构
/// 迭代需要 D 实现 Mig 来达到 
#[derive(Debug,Default,Serialize,Deserialize)]
pub struct RJson<D> 
    where D : Debug+Default+Clone+FilePath +Sized{//其实 D 要被 Mig 约束,但是不知道为什么 Deserialize 总是报错无法编译
    
    // 当前的版本 , 以防应用长期不更新,出现 中间版本 ,从0开始,若是1则从0迁移到1,若是n则从0->1->...->n-1->n
    // 若 存储版本号 和 最新版本号不一致 则需要做出更新
    pub version:usize,
    
    pub data:D,//D类型是最新版本类型
    
}

impl<D:Mig> Deref for RJson<D>{
    type Target=D;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<D:Mig> DerefMut for RJson<D>{
    
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}



impl<D:Mig> RJson<D> {
    
    /// 不更新,直接读取文件
    /// 不存在会panic
    // pub fn get()->Self{ //不应该有这个,这只会让bug越来越多, 当文件的静态状态还没存储时,获取的文件状态和静态状态是不等同的
    //     let file = File::open(D::get_file_position()).unwrap();
    //     serde_json::from_reader(file).unwrap()
    // }

    /// 想要D具有迁移能力,只需要D实现Mig和它的老版本也实现Mig
    /// 这个函数在 初始化 创建结构体的 时候运行一次就可以了
    pub fn updata()->Self{
        let json = D::_updata(0);// 直到0为止,或搜索到now_version
        let mut json = Self{
            version:D::get_version(),
            data:json,
        };
        let d_type_name = std::any::type_name::<D>();
        let position = D::get_file_position();
        println!("\n文件更新:{position:?}\nRJson<{d_type_name}>:\n{json:#?}\n");
        Self::_updata_file(&json);
        json
    }
    
    /// 返回一个自动 存储的 Json
    pub fn auto<'d>(&'d mut self)->AutoRJson<'d,D>{//为了清晰点,显性标记'd
        AutoRJson{
            data:&mut self.data,
        }
    }

    /// 不存在 则 创建文件,并且传入数据
    /// 存在 则 更新数据
    fn _updata_file(data:&RJson<D>){
        match File::create(D::get_file_position()) {//文件不存在,则创建文件+写入数据,存在则,写入数据
            Ok(file) => {//文件目录未变更,只是 数据结构变更,则会运行下面的代码
                serde_json::to_writer_pretty(&file,data).unwrap();
            },
            Err(_) =>{
                panic!("不应该出现其他错误")
            },
        }
    }
    
    /// 存储文件
    pub fn save(&self){
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(D::get_file_position()).unwrap();
        serde_json::to_writer_pretty(
            file,
            self).unwrap();
    }

    /// 刷新自己的数据  
    /// 在有些地方非常有用,例如 切换用户后 程序也需要重新加载用户数据
    pub fn refresh(&mut self){
        *self = Self{..Self::updata()};
    }
}


/// 当这个结构体生命结束时,它会存储一次数据
#[derive(Debug)]
pub struct AutoRJson<'d,D:Mig>{
    data:&'d mut D,
}
impl<'d,D:Mig> Drop for AutoRJson<'d,D>{
    fn drop(&mut self) {
        self.save()
    }
}
impl<'d,D:Mig> Deref for AutoRJson<'d,D> {
    type Target=D;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}
impl<'d,D:Mig> DerefMut for AutoRJson<'d,D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}
impl<'d,D:Mig> AutoRJson<'d,D>  {
    /// 存储文件
    pub fn save(&self){
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(D::get_file_position()).unwrap();
        serde_json::to_writer_pretty(
            file,
            &json!({
                "version":D::get_version(),
                "data":*self.data,//json宏会自动加个&, 即它现在是 &(*self.data) 类型是 &D
            })
        ).unwrap();
    }
}




/// 迁移 特征  
/// 正常的话,你只需要实现 [get_version]和[_old_version]  
/// 第一版甚至连 [_old_version] 都不用实现 加个 todo!() 直接跳过就好
pub trait Mig 
    where Self: FilePath+Debug+Default+Clone+Sized+DeserializeOwned+Serialize{
    /// 获取当前类型的 版本
    fn get_version()->usize;

    /// 你不应该手动调用这个函数,它理应被包装好, 除了它被下一个版本的_old_version函数调用  
    /// 注意的是, 这个函数是处理 数据迭代更新,包括文件生成  
    /// 注意: 若用户是第一次使用软件, 数据迭代会从 第一版的默认值 迭代到 最新版 后 生成文件  
    ///      
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
            if let Ok(file) = File::open(Self::get_file_position()){
                //Ok : 说明找到版本
                let json:serde_json::Value =serde_json::from_reader(file).unwrap();
                let version:usize=json["version"].as_i64().unwrap() as usize;
                now_version = version;
            }
        }
        if now_version == Self::get_version() {//存储版本 与 类型版本 一致,停止向老版本传递,开始向新版本更新
            match File::open(Self::get_file_position()) {
                Ok(file) => { //版本一致必然获取, 除非 软件第一次被使用,压根没有任何数据(旧数据新数据都没有)
                    let json:RJson<Self> = serde_json::from_reader(file).unwrap();
                    json.data
                },
                Err(_) => {//能运行到这里 就说明 : now_version==0 并且 这是软件第一次启动,压根没有一点点数据
                    Self::default()
                },
            }
        
        }else {// 向老版本传递, 并且检查老版本的路径变更,若变化则删除老版本文件 , 第一版永远都不应该运行到这里
            let (json,old_path)=Self::_old_version(now_version);
            Self::_delete_file(&old_path);
            json
        }
    }
    
    /// 你不应该手动调用这个函数,它理应被包装好  
    /// 声明 上一个版本  
    /// 0版本,也就是第一版,不用实现它,毕竟它是最老的版本  
    /// 并且 处理 上一个版本 迁移 到 这个版本  
    /// ```
    /// let old=Old::_updata(now_version);
    /// ...;// 迁移
    /// return (self_json,Old::get_file_position());//返回 迁移后的版本
    /// ```
    fn _old_version(now_version:usize)->(Self,PathBuf);
    
    /// 你不应该手动调用这个函数,它理应被包装好  
    /// 若老版本 文件路径 不等于 当前版本文件路径 ,则删除老版本文件  
    fn _delete_file(old_path:&Path){
        if old_path != Self::get_file_position(){
            fs::remove_file(old_path);
        }
    }

}





/// 以下 是可运行的
pub mod example {
    
    use std::{path::PathBuf, io::Read, sync::RwLock};

    use crate::other::{chaos::file_name::FileName, path::app_path::AppPath};

    use super::*;

    lazy_static!{
        pub static ref MY_JSON:RwLock<MyJson> = {//todo 删除RwLock试试
            MyJson::updata().into()
        };
    }
    pub fn example_use_myjson(){
        println!("{:#?}",*MY_JSON);
        let mut my_json_r = MY_JSON.write().unwrap();
        {
            let mut auto = my_json_r.auto();
            auto.i=999;
            // auto.save();//可以手动保存,虽然也可以自动保存
        }
        {
            my_json_r.i = 888;
            my_json_r.save();//若没有用 auto ,那么就需要手动存储
            // 事实上,若需要存储,应该一直用 auto,以免忘记
            // 注意:auto返回的数据 和 它是同步的
        }
    }

    /// 这才是你 需要用的类型
    /// D 永远是你的最新版本 
    /// 你只需要切换D类型,就可以决定当前使用的版本
    pub type MyJson = RJson<NewPlus>;

    #[derive(Debug,Default,Clone,Serialize, Deserialize)]
    pub struct NewPlus{//它只变更了数据结构
        i:i32,
        s:String,
    }
    impl FileName for NewPlus {
        fn get_file_name() -> &'static str {
            "new.json"
        }
    }
    impl FilePath for NewPlus  {
        fn get_file_path()->std::path::PathBuf {
            AppPath::Test.get_path()
        }
    }
    impl Mig for NewPlus {
        fn get_version()->usize {
            2
        }

        fn _old_version(now_version:usize)->(Self,PathBuf) {
            let old = New::_updata(now_version);
            (Self{
                i:666,
                s:old.0
            },New::get_file_position())
        }
    }

    #[derive(Debug,Default,Clone,Serialize, Deserialize)]
    pub struct New(String);//它在老版本上更新了路径
    impl FileName for New {
        fn get_file_name() -> &'static str {
            "new.json"
        }
    }
    impl FilePath for New  {
        fn get_file_path()->std::path::PathBuf {
            AppPath::Test.get_path()
        }
    }
    impl Mig for New {
        fn get_version()->usize {
            1
        }

        fn _old_version(now_version:usize)->(Self, PathBuf) {
            let old = Old::_updata(now_version);
            let mut now = Self::default();
            now.0 = format!("New:{}",old.0.to_string());
            (now,Old::get_file_position())
        }

    
    }

    #[derive(Debug,Default,Clone,Serialize, Deserialize)]
    pub struct Old(i32);
    impl FileName for Old {
        fn get_file_name() -> &'static str {
            "old.json"
        }
    }
    impl FilePath for Old  {
        fn get_file_path()->std::path::PathBuf {
            AppPath::Test.get_path()
        }
    }
    impl Mig for Old {
        fn get_version()->usize {
            0
        }

        fn _old_version(now_version:usize)->(Self, PathBuf) {
            todo!("我是第一版,我不应该被运行")
        }
    }
    
    
    
}











