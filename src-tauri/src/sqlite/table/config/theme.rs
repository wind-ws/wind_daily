use diesel::*;

use crate::from_to_sql_json;




#[derive(Queryable)]
#[derive(Debug,serde::Serialize, serde::Deserialize,diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Text)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ThemeJson {
    theme:String
}
from_to_sql_json!(ThemeJson);