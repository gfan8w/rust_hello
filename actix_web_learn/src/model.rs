use chrono::{DateTime, Local, TimeZone, Utc};
use diesel::backend::Backend;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::sql_types::Timestamp;
use super::schema::{posts, users};
use diesel::sql_types::*;
use serde::Serialize;

// 用于查询， chrono引入要支持serde，这里的NaiveDateTime才能支持序列化
// QueryableByName用于识别列名和类型
#[derive(QueryableByName, Debug, Serialize)]
#[table_name = "users"]
pub struct Users {
    pub id: i64,
    #[diesel(deserialize_as = "LowercaseString")]   //自定义序列化
    pub name: String,
    pub hair_color: Option<String>,                  // 这里要跟schema一致，
    pub created_at: chrono::NaiveDateTime,
    #[diesel(deserialize_as = "DateTimeLocalWrapper")] //自定义时间序列化
    pub updated_at: chrono::DateTime<Local>,
    #[sql_type="Text"]                               // 该列不存在，需要指定类型。
    pub fullname : String,
}


#[derive(Insertable)]
#[table_name = "users"]
pub struct UserForInsert<'a> {
    pub name: &'a str,
    pub hair_color: Option<&'a str>,
}




///时间格式转换
struct DateTimeLocalWrapper(chrono::DateTime<Local>);

impl Into<chrono::DateTime<Local>> for DateTimeLocalWrapper {
    fn into(self) -> DateTime<Local> {
        self.0
    }
}

impl<DB, ST>  FromSql<ST,DB> for DateTimeLocalWrapper
where
DB: Backend,
chrono::NaiveDateTime: FromSql<ST, DB>
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        chrono::NaiveDateTime::from_sql(bytes).map(|s|{
            let aa =Local.from_utc_datetime(&s);
            Self(aa)
        })
        //DateTime::<Utc>::from_sql(bytes).map(|s|Self(s.with_timezone(&Local)))
    }
}


// 转换为小写
struct LowercaseString(String);

impl Into<String> for LowercaseString {
    fn into(self) -> String {
        self.0
    }
}

impl<DB, ST> FromSql<ST, DB> for LowercaseString
    where
        DB: Backend,
        String: FromSql<ST, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        String::from_sql(bytes)
            .map(|s| LowercaseString(s.to_lowercase()))
    }
}








