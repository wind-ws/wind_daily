

use std::{fmt::Debug, fs::File, any::{TypeId, Any}};

use serde::{de::DeserializeOwned, Serialize, Deserialize};
use serde_json::{Value, json};

use crate::other::chaos::file_name::FilePath;

use super::*;



#[derive(Serialize,Deserialize)]
#[derive(Debug,Default,Clone)]
pub struct FileJson<D>
where
    D:  Sized+
        MigData+MigUp+MigDown+MigGetSelf+MigVersion+
        FilePath+
        Debug+Default+Clone+
        {
    now_version:VersionMarkType,
    data:D
}


impl<D> Mig<D> for FileJson<D>
where 
    Self: MigVersion,
    D:  Sized+
        MigData+MigUp+MigDown+MigGetSelf+MigVersion+
        FilePath+
        Debug+Default+Clone+{

}

impl<D> MigVersion for FileJson<D> 
where 
    D:  Sized+
        MigData+MigUp+MigDown+MigGetSelf+MigVersion+
        FilePath+
        Debug+Default+Clone+{
    fn now_version()->VersionMarkType {
        D::now_version()
    }
}

impl<D> FileJson<D> 
where 
    Self:Mig<D>+MigVersion,
    D:  Sized+
        MigData+MigUp+MigDown+MigGetSelf+MigVersion+
        FilePath+
        Debug+Default+Clone+
        Serialize{
    fn new(data:D)->Self{
        Self { now_version: D::MY_VERSION, data }
    }
    /// 一切 迁移调用这个
    fn mig()->Self{
        let d = <Self as Mig<D>>::mig();
        let file = File::create(D::get_file_position()).unwrap();
        serde_json::to_writer_pretty::<_,Value>(file,&json!({
            "now_version":D::MY_VERSION,
            "data":&d
        })).unwrap();
        Self::new(d)
    }
}

impl<D> MigVersion for D 
where
    D:  Sized+
        MigData+MigUp+MigDown+
        FilePath+
        Debug+Default+Clone+,
    <D as MigData>::Old  : Sized+GetOldVersion,
    <D as MigData>::Next : Sized+GetNextVersion,{
    fn now_version()->VersionMarkType {
        if let Ok(file) = File::open(D::get_file_position()){
            let json = serde_json::from_reader::<_,Value>(file).unwrap();
            let version = json["now_version"].as_u64().unwrap();
            version as VersionMarkType
        }else if let Some(v) = <D as MigData>::Old::get_old_version(){
            v
        } else if let Some(v) = <D as MigData>::Next::get_next_version() {
            v
        }else {//Old和Next都没找到版本,说明 压根没有存储任何关于这个结构的数据,返回当前结构版本,让MigGetSelf去创建文件
            D::MY_VERSION
        }
    }
}

trait GetOldVersion {
    fn get_old_version()->Option<VersionMarkType>;
}
impl<D> GetOldVersion for D  
where
    D:  Sized+
        MigData+
        FilePath+
        Debug+Default+Clone+,
    <D as MigData>::Old  : Sized+GetOldVersion,{
    fn get_old_version()->Option<VersionMarkType> {
        if let Ok(file) = File::open(D::get_file_position()){
            let json = serde_json::from_reader::<_,Value>(file).unwrap();
            let version = json["now_version"].as_u64().unwrap();
            return Some(version as VersionMarkType);
        }
        <D as MigData>::Old::get_old_version()
    }
}
impl GetOldVersion for () {
    fn get_old_version()->Option<VersionMarkType> {
        None
    }
}

trait GetNextVersion {
    fn get_next_version()->Option<VersionMarkType>;
}
impl<D> GetNextVersion for D  
where
    D:  Sized+
        MigData+
        FilePath+
        Debug+Default+Clone+,
    <D as MigData>::Next  : Sized+GetNextVersion,{
    fn get_next_version()->Option<VersionMarkType> {
        if let Ok(file) = File::open(D::get_file_position()){
            let json = serde_json::from_reader::<_,Value>(file).unwrap();
            let version = json["now_version"].as_u64().unwrap();
            return Some(version as VersionMarkType);
        }
        <D as MigData>::Next::get_next_version()
    }
}
impl GetNextVersion for () {
    fn get_next_version()->Option<VersionMarkType> {
        None
    }
}

impl<D> MigGetSelf for D 
where 
    D:  Sized+
        MigData+
        FilePath+
        Debug+Default+Clone+
        DeserializeOwned+Serialize,
    {
    fn get_self()->Self {
        if let Ok(file) = File::open(Self::get_file_position()) { 
            let mut json=serde_json::from_reader::<_,
                Value>(file).unwrap();
            std::fs::remove_file(Self::get_file_position()).unwrap();//可能是旧(超)版本也可能是正确版本,但 没法知晓,所以统一删除后重新创建
            serde_json::from_value::<Self>(json["data"].take()).unwrap()
        }else {
            if let Ok(file) = File::create(Self::get_file_position()){
                let json = D::default();
                serde_json::to_writer_pretty::<_,Value>(file,
                        &json!({
                            "now_version":D::MY_VERSION,
                            "data":&json
                        })).unwrap();
                json
            } else {
                panic!("不应该出现其他错误")
            }
        }
    }
}

#[cfg(test)]
mod example {
    use std::{fs::File, io::Read, any::TypeId};

    use rusqlite::types::Value;

    use crate::other::chaos::file_name::FileName;

    use super::*;
    use super::super::*;

    #[derive(Debug,Deserialize,Serialize,Default,Clone)]
    struct A0{
        pub a0:i32,
    }
    impl FilePath for A0 {
        fn get_file_path()->std::path::PathBuf {
            "./test/mig_file".into()
        }
    }
    impl FileName for A0 {
        fn get_file_name() -> &'static str {
            "A0.json"
        }
    }
    impl MigData for A0 {
        type Data=FileJson<A0>;
        type Old=();
        type Next=A1;
        const MY_VERSION:VersionMarkType=0;
    }
    impl MigUp for A0 {
        fn _up_from_old(_old:Self::Old)->Self {
            panic!("没有老版本,这里不应该被执行")
        }
    }
    impl MigDown for A0 {
        fn _down_from_next(next:Self::Next)->Self{//往往不需要回溯,可以懒的去写它
            Self { a0: next.a1.len() as i32 }
        }
    }


    #[derive(Debug,Deserialize,Serialize,Default,Clone)]
    struct A1{
        pub a1:String,
    }
    impl FilePath for A1 {
        fn get_file_path()->std::path::PathBuf {
            "./test/mig_file".into()
        }
    }
    impl FileName for A1 {
        fn get_file_name() -> &'static str {
            "A1.json"
        }
    }
    impl MigData for A1{
        type Data=FileJson<A1>;
        type Old=A0;
        type Next=();
        const MY_VERSION:VersionMarkType=1;
    }
    impl MigUp for A1 {
        fn _up_from_old(old:Self::Old)->Self {
            Self{
                a1:format!("String->[{}]",old.a0)
            }
        }
    }
    impl MigDown for A1 {}


    #[test]
    fn test_normal_a0(){
        type ExampleJson = FileJson<A0>;
        ExampleJson::mig();
    }

    #[test]
    fn test_normal_a1(){
        type ExampleJson = FileJson<A1>;
        ExampleJson::mig();
    }

}