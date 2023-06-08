
use crate::from_to_sql_json;



#[derive(Debug,serde::Serialize, serde::Deserialize,diesel::AsExpression,diesel::FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum Priority {
    Indifferent, //无所谓啦级(灰色),做不做都可以或者说想做就做
    Ordinary,    //一般般啦级(绿色),有资源(时间)在做
    Normal,      //正常级(蓝色),需要完成,
        // Normal优先Indifferent & Ordinary,即建议完成所有Normal后在进行优先级更低的
        // Normal被优先Urgent & Haunted ,即建议 所有被优先 级 完成后在进行Normal级
    Urgent,      //紧急级(黄色),放下爱情和理想,赶快去完成这个todo
    Haunted,     //要命级(红色),当它出现后,会派出一只鬼去追杀你,直到你完成todo
}

from_to_sql_json!(Priority);


impl From<Priority> for &str {
    fn from(value: Priority) -> Self {
        match value {
            Priority::Indifferent => "Indifferent"  ,
            Priority::Ordinary    => "Ordinary"     ,
            Priority::Normal      => "Normal"       ,
            Priority::Urgent      => "Urgent"       ,
            Priority::Haunted     => "Haunted"      ,
        }
    }
}
impl From<&str> for Priority {
    fn from(value: &str) -> Self {
        match value {
            "Indifferent" => Priority::Indifferent    ,
            "Ordinary"    => Priority::Ordinary       ,
            "Normal"      => Priority::Normal         ,
            "Urgent"      => Priority::Urgent         ,
            "Haunted"     => Priority::Haunted        ,
            _             => panic!("意外的Priority值")
        }
    }
}


