
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value};





/// 只注册一个命令, 通过 symbol 来触发不同的业务
#[derive(Debug,Deserialize,Serialize)]
pub enum CommandMark {
    CreateNewUser,
}
/// 所有业务统一用一个 Error处理
#[derive(Debug, thiserror::Error,Serialize,Deserialize)]
pub enum CommandError {
    #[error("用户名已存在")]
    UserNameExist,
}

type Res = Result<(),CommandError>;

#[tauri::command]
pub fn app_config_command(mark:CommandMark,data:Value)->Res{

    match mark {
        CommandMark::CreateNewUser =>create_new_user::create_new_user(from_value(data).unwrap()),
    }
}

mod create_new_user{
    use std::{path::PathBuf, fs};

    use serde::{Deserialize, Serialize};

    use crate::other::{path::{app_path::AppPath, user_path::UserPath}, app::app_config::{APP_CONFIG_RJSON, active_user::ActiveUser,}};

    use super::*;

    #[derive(Debug,Deserialize,Serialize)]
    pub(super) struct CreateNewUserData {
        name:String,
        #[serde(default = "user_default_path")] 
        path:PathBuf,//将会在这个路径下创建 用户文件夹 (以name创建文件夹名)
    }
    pub(super) fn create_new_user(CreateNewUserData{name,path}:CreateNewUserData)->Res{
        let mut lock = APP_CONFIG_RJSON.write().unwrap();
        let mut auto = lock.auto();
        //我讨厌 写这种代码, 就像我无聊且充实的日常生活一样, 事多 还不好玩🥀🥀🥀
        if auto.app_user_list.user_list.contains_key(&name){//用户名 已存在
            return Err(CommandError::UserNameExist);
        }
        let user_root_path = path.join(&name);
        fs::create_dir(&user_root_path).unwrap();//create root user directory
        UserPath::updata_user_path(&user_root_path);//create all user directory
        auto.app_user_list.user_list.insert(name.clone(), user_root_path.clone());
        auto.app_user_list.user_count = auto.app_user_list.user_list.len();
        
        auto.switch_active_user(ActiveUser{name,path:user_root_path});//将它设为活动用户
        
        // 用户文件会在使用时 进行更新(若不存在则创建) , 所以这里不用关心 文件们

        Ok(())
    }

    fn user_default_path()->PathBuf {
        AppPath::DefaultUser.get_path()
    }
}



