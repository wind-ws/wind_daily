use std::{fs::File, io::Read};

use crate::other::{app::app_config::AppConfigRJson, chaos::file_name::FilePath};




lazy_static!{
    static ref app_config_rjson:AppConfigRJson={
        AppConfigRJson::updata()
    };
}

