use juniper::{EmptyMutation,RootNode};

use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use crate::schema::todos;
use crate::models::ToDo;


/// 定义那些成员需要返回
#[juniper::object(description="A todo")]
impl ToDo {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
    pub fn completed(&self) -> bool {
        self.completed
    }
}

///定义一个queryroot，
pub struct QueryRoot;

///返回几个示例对象
#[juniper::object]
impl QueryRoot {
    fn todos() ->Vec<ToDo> {
        vec![
            ToDo{
               id: 1,title:"lint".to_string(),completed:false
            },
            ToDo{
                id: 2,title:"ppg".to_string(),completed:false
            }
        ]
    }
}


///定义schema
pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

///创建schema
pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}


pub fn establish_connection1() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("failed to connect to {}",database_url))
}











