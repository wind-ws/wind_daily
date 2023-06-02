use diesel::{*, sql_types::Text};

use crate::{from_to_sql_json, sqlite::schema::config, other::user::user_db::Db};




#[derive(Queryable)]
#[derive(Debug,serde::Serialize, serde::Deserialize,diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct ThemeJson {
    theme:String
}
from_to_sql_json!(ThemeJson);

impl super::ConfigJson for ThemeJson {
    fn get_key()-> &'static str {
        "theme"
    }
}

// impl ThemeJson {
//     fn get_key() -> &'static str {
//         "theme"
//     }
//     fn get_json()->Self{
//         use crate::sqlite::schema::config::dsl::*;
//         let mut db=Db.get_db().unwrap();
//         let s:String = config
//             .filter(key.eq(Self::get_key()))
//             .select(json)
//             .first::<String>(&mut db).unwrap();
//         serde_json::from_str(&s).unwrap()
//     }
//     pub fn set_json(&self,v_json:Self)
//     where
//         Self:serde::Serialize+serde::de::DeserializeOwned{
//         use crate::sqlite::schema::config::dsl::*;
//         let mut db=Db.get_db().unwrap();
//         diesel::insert_into(config)
//             .values(super::Config{
//                 id:0,//自动增加,不用关心
//                 key:Self::get_key().to_string(),
//                 json:serde_json::to_string(&v_json).unwrap()
//             })
//             .execute(&mut db)
//             .unwrap();
//     }

// }
