

use std::{fmt::Debug, fs::File, any::{TypeId, Any}};

use serde::{de::DeserializeOwned, Serialize, Deserialize};
use serde_json::Value;

use crate::other::chaos::file_name::FilePath;

use super::*;



#[derive(Debug,Serialize,Deserialize)]
pub struct FileJson<D>
where
    D:  Sized+
        MigData+
        MigUp+
        MigDown+
        MigGetSelf+
        FilePath+
        Debug+
        MigVersion{
    now_version:VersionMarkType,
    data:D
}


impl<D> Mig<D> for FileJson<D>
where 
    Self: MigVersion,
    D:  Sized+
        MigData+
        MigUp+
        MigDown+
        MigGetSelf+
        FilePath+
        Debug+
        MigVersion{

}

impl<D> MigVersion for FileJson<D> 
where 
    D:  Sized+
        MigData+
        MigUp+
        MigDown+
        MigGetSelf+
        FilePath+
        Debug+
        MigVersion{
    fn now_version()->VersionMarkType {
        D::now_version()
    }
}

impl<D> FileJson<D> 
where 
    D:  Sized+
        MigData+
        MigUp+
        MigDown+
        MigGetSelf+
        FilePath+
        Debug+
        MigVersion{
    fn new(data:D)->Self{
        Self { now_version: D::MY_VERSION, data }
    }
}

impl<D> MigVersion for D 
where
    D:  Sized+
        MigData+
        MigUp+
        MigDown+
        MigGetSelf+
        FilePath+
        Debug+,
    <D as MigData>::Old  : Sized+GetOldVersion,
    <D as MigData>::Next : Sized+GetNextVersion,{
    fn now_version()->VersionMarkType {
        if let Some(v) = <D as MigData>::Old::get_old_version(){
            v
        } else if let Some(v) = <D as MigData>::Next::get_next_version() {
            v
        }else {
            panic!("Old和Next都没找到版本")
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
        MigUp+
        MigDown+
        MigGetSelf+
        FilePath+
        Debug+,
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
        MigUp+
        MigDown+
        MigGetSelf+
        FilePath+
        Debug+,
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
    impl MigGetSelf for A0 {
        fn get_self()->Self {
            if let Ok(file) = File::open(Self::get_file_position()) { 
                serde_json::from_reader::<_,
                    <Self as MigData>::Data>(file).unwrap().data
            }else {
                if let Ok(file) = File::create(Self::get_file_position()){
                    let json = Self::default();
                    serde_json::to_writer_pretty::<_,
                        <Self as MigData>::Data>(file,
                            &<Self as MigData>::Data::new(json.clone())).unwrap();
                    json
                } else {
                    panic!("不应该出现其他错误")
                }
            }
        }
    }
    impl MigUp for A0 {
        fn _up_from_old(_old:Self::Old)->Self {
            panic!("没有老版本,这里不应该被执行")
        }
    }
    impl MigDown for A0 {}


    #[derive(Debug,Deserialize,Serialize)]
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
    impl MigGetSelf for A1 {
        fn get_self()->Self {
            todo!()
        }
    }
    impl MigUp for A1 {
        fn _up_from_old(old:Self::Old)->Self {
            Self{
                a1:format!("String->[{}]",old.a0)
            }
        }
    }
    impl MigDown for A1 {}


    struct B0{
        b0:Vec<String>
    }


    type ExampleJson = FileJson<A0>;
    #[test]
    fn f(){
        ExampleJson::mig();
    }

}