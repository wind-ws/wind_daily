use diesel::{*, sql_types::Text};

use crate::{from_to_sql_json, sqlite::schema::config, other::user::user_db::Db};




#[derive(Queryable)]
#[derive(Debug,serde::Serialize, serde::Deserialize,diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct ThemeJson {
    pub theme:String
}
from_to_sql_json!(ThemeJson);

impl super::ConfigJson for ThemeJson {
    fn get_key()-> &'static str {
        "theme"
    }
}

