

use std::{fmt::Debug, fs::File, any::{TypeId, Any}};

use serde::{de::DeserializeOwned, Serialize, Deserialize};
use serde_json::Value;

use crate::other::chaos::file_name::FilePath;

use super::*;

#[derive(Debug,Serialize,Deserialize)]
struct FileJson<D>
where
    D:  Sized+
        MigVersionData+ 
        MigUp+ 
        MigDown+
        MigVersion+
        FilePath+
        Debug+{
    now_version:VersionMarkType,
    data:D
}


impl<D> Mig<D> for FileJson<D>
where 
    D:  Sized+
        MigVersionData+ 
        MigUp+ 
        MigDown+
        MigVersion+
        FilePath+
        Debug+
        DeserializeOwned+
        Serialize+{

}

impl<D> MigVersion for FileJson<D> 
where 
    D:  Sized+
        MigVersionData+ 
        MigUp+ 
        MigDown+
        MigVersion+
        FilePath+
        Debug+ 
        DeserializeOwned+
        Serialize+{
    fn now_version()->VersionMarkType {
        D::now_version()
    }
}
impl<D> FileJson<D> 
where 
    D:  Sized+
        MigVersionData+ 
        MigUp+ 
        MigDown+
        // MigVersion+
        FilePath+
        Debug+
        DeserializeOwned+
        Serialize+{
    fn new(data:D)->Self{
        Self { now_version: D::MY_VERSION, data }
    }
}


/// 实现它,会自动实现MigVersion
trait AutoMigVersion
where
    Self:   Sized+
            FilePath+
            Debug+
            DeserializeOwned+
            Serialize{
    
}

// impl<F> MigVersion for F //这个似乎会引起 Vscode中rust-analyzer的分析循环问题,导致内存炸掉
// where 
//     F:Sized+AutoMigVersion+MigVersionData,
//     <F as MigVersionData>::Old : Sized+MigVersion +'static ,
//     <F as MigVersionData>::Next : Sized+MigVersion+'static ,{
//     fn now_version()->VersionMarkType {
//         if let Ok(file) = File::open(Self::get_file_position()){
//             let json = serde_json::from_reader::<_,Value>(file).unwrap();
//             json["now_version"].as_u64().unwrap() as VersionMarkType
//         }else{
//             // 因为这里是最老版本,所以不用考虑 Old
//             let version=if TypeId::of::<<Self as MigVersionData>::Old>() != TypeId::of::<()>(){
//                 <Self as MigVersionData>::Old::now_version()
//             }else{
//                 u32::MAX//已经到达最老版本,停止搜索
//             };
//             if version != u32::MAX{//version != u32::MAX,如果等于Max说明,Old方向没有找到版本,需要去Next方向找版本
//                 return version;
//             }
//             if TypeId::of::<<Self as MigVersionData>::Next>() != TypeId::of::<()>(){
//                 <Self as MigVersionData>::Next::now_version()// ! 这会导致 重新再次搜索老版本(即使老版本找不到存储版本),不过也不算大事,无非是搜索时间变长,不管他
//             }else {
//                 panic!("没找到版本")//Old和Next方向都没有找到版本...
//             }
//         }
//     }
// }

#[cfg(test)]
mod example {
    use std::{fs::File, io::Read, any::TypeId};

    use rusqlite::types::Value;

    use crate::other::chaos::file_name::FileName;

    use super::*;


    #[derive(Debug,Deserialize,Serialize,Default,Clone)]
    struct A0{
        a0:i32,
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
    
    impl MigVersionData for A0 {
        type Data=FileJson<A0>;

        type Old=();

        type Next=A1;

        const MY_VERSION:VersionMarkType=0;

        fn get_self()->Self {
            if let Ok(file) = File::open(Self::get_file_position()) { 
                serde_json::from_reader::<_,Self::Data>(file).unwrap().data
            }else {
                if let Ok(file) = File::create(Self::get_file_position()){
                    let json = Self::default();
                    serde_json::to_writer_pretty::<_,Self::Data>(file,&Self::Data::new(json.clone())).unwrap();
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
    
    impl AutoMigVersion for A0 {}
    impl MigDown for A0 {}


    #[derive(Debug,Deserialize,Serialize)]
    struct A1{
        a1:String,
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

    impl MigVersionData for A1{
        type Data=FileJson<A1>;

        type Old=A0;

        type Next=();

        const MY_VERSION:VersionMarkType=1;

        fn get_self()->Self {
            todo!()
        }
    }
    impl MigUp for A1 {
        fn _up_from_old(_old:Self::Old)->Self {
            todo!()
        }
    }
    impl AutoMigVersion for A1 {}
    impl MigDown for A1 {}


    struct B0{
        b0:Vec<String>
    }

    #[test]
    fn f(){

    }

}