use juniper::{EmptyMutation,RootNode};

use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use crate::schema::todos;
use crate::models::{NewTodo, ToDo};


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

        use crate::schema::todos::dsl::*;
        let conn = crate::establish_connection();
        let mut result = todos.load::<ToDo>(&conn).expect("Error load todos");
        let append =vec![
            ToDo{
               id: 1,title:"lint".to_string(),completed:false
            },
            ToDo{
                id: 2,title:"ppg".to_string(),completed:false
            }
        ];
        for a in append {
            result.push(a);
        }

        result
    }
}


///定义一个变更

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    ///插入新的todo，返回刚才新插入的那个todo
    fn create_todo(new_todo: NewTodo) -> ToDo {
        use crate::schema::todos::dsl::*;
        let conn = crate::establish_connection();
        let to_do = diesel::insert_into(todos)
            .values(&new_todo)
            .get_result::<ToDo>(&conn)
            .expect("Error saving new Todo");
        to_do
    }

    ///插入title,completed,生成新的todo，返回刚才新插入的那个todo
    fn insert_todo(tit: String, comp: bool) -> ToDo {
        use crate::schema::todos::dsl::*;
        let conn = crate::establish_connection();
        let to_do = diesel::insert_into(todos)
            .values(NewTodo{title:tit,completed:comp})
            .get_result::<ToDo>(&conn)
            .expect("Error saving new Todo");
        to_do
    }
}



///定义schema
//pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;  //如果没有定义MutationRoot，就用 EmptyMutation<()>
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;  // 定义了MutationRoot，就替换掉原来的 EmptyMutation<()>

///创建schema
pub fn create_schema() -> Schema {
    //Schema::new(QueryRoot {}, EmptyMutation::new())
    Schema::new(QueryRoot {}, MutationRoot{}) // 定义了MutationRoot，就替换掉原来的 EmptyMutation::new()
}


pub fn establish_connection1() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("failed to connect to {}",database_url))
}











