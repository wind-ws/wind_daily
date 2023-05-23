use std::{fs::File, io::Read};

use crate::other::{app::app_config::AppConfigRJson, chaos::file_name::FilePath};




lazy_static!{
    static ref app_config_rjson:AppConfigRJson={
        let mut file_json = File::open(AppConfigRJson::get_file_position()).unwrap();//这里若错误,请检查 文件初始化 是否成功
        let mut json = String::new();
        file_json.read_to_string(&mut json).unwrap();
        serde_json::from_str(json.as_str()).unwrap()
    };
}

