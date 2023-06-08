
use chrono::{NaiveDateTime, NaiveDate};
use diesel::{Queryable, sql_types::Timestamp, sqlite::{Sqlite}, deserialize::{FromSql, self}, serialize::ToSql, backend::Backend};
use serde::{Serialize, Deserialize, de::Visitor};


pub static DATE_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.f";

/// 由于NaiveDateTime没有实现Deserialize, Serialize,所以我们需要包装一下
/// 对应的sql类型是 timestamp 
#[derive(Debug,diesel::AsExpression)]
#[diesel(sql_type = Timestamp)]
pub struct DateTime(pub NaiveDateTime);

impl std::ops::Deref for DateTime {
    type Target=NaiveDateTime;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl Into<String> for DateTime {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl Queryable<Timestamp, Sqlite> for DateTime
where
    String: FromSql<Timestamp, Sqlite>{
    type Row = String;
    fn build(s: String) -> deserialize::Result<Self> {
        Ok(serde_json::from_str(&s).unwrap())
    }
}

impl FromSql<Timestamp,Sqlite> for DateTime{
    fn from_sql(bytes:<Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let a = <String as FromSql<Timestamp, Sqlite>>::from_sql(bytes).unwrap();
        Ok(serde_json::from_str(&a).unwrap())
    }
}
impl ToSql<Timestamp,Sqlite> for DateTime {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(serde_json::to_string(self).unwrap());
        Ok(diesel::serialize::IsNull::No)
    }
}


impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer
            .serialize_str(self.0.to_string().as_str())
    }
}
impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(
            DateTime(
                deserializer
                    .deserialize_str(NaiveDateTimeVisitor)
                    .unwrap()
            )
        )
    }
}

struct NaiveDateTimeVisitor;

impl<'de> Visitor<'de> for NaiveDateTimeVisitor {

    type Value=NaiveDateTime;

    fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        panic!("喔喉~ 反序列化失败,是不是手动改时间格式了?或者是改json格式了(要求是字符串哦~)")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(NaiveDateTime::parse_from_str(v,DATE_TIME_FORMAT).unwrap())
    }
    // fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    //     where
    //         E: serde::de::Error, {
    //     Ok(NaiveDateTime::parse_from_str(&v,DATE_TIME_FORMAT).unwrap())
    // }
}


impl DateTime {
    pub fn new(date_time:NaiveDateTime)->Self{
        DateTime(date_time)
    }
    pub fn new_milli(date:(i32,u32,u32),day:(u32,u32,u32,u32))->Self{
        Self::new(
            NaiveDate::from_ymd_opt(date.0,date.1,date.2)
            .unwrap().
            and_hms_milli_opt(day.0,day.1,day.2,day.3)
            .unwrap()
        )
    }
    pub fn new_sec(date:(i32,u32,u32),day:(u32,u32,u32))->Self{
        Self::new_milli(date,(day.0,day.1,day.2,0))
    }

    pub fn now()->Self{
        Self::new(chrono::Local::now()
            .naive_local())
    }
    
}
