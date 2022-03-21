
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use futures::future::Future;
use std::io;
use juniper::http::GraphQLRequest;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use std::sync::Arc;
use juniper::http::graphiql::graphiql_source;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


mod graphql_schema;
mod schema;
mod models;

use graphql_schema::{create_schema, Schema};


///设置 GraphQL server
/// graphql是查询引擎。
/// graphiql 是graphql的查询界面
fn main() -> std::io::Result<()> {

    println!("start GraphQL server");

    //初始化GraphQL schema
    let schema = Arc::new(create_schema());

    //启动web 服务，move是让schema移动到内部
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
        .bind("0.0.0.0:8080")?
        .run()
}

///把GraphQLRequest的插入放入schema去查询
/// 从json中获取GraphQL的request，从web::block中创建future，该future把错误处理(map_err)和成功处理(and_then)链接起来
///
fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>
) -> impl Future<Item=HttpResponse, Error=Error> {

    web::block(move ||{
        let res = data.execute(&st,&());
        let r =serde_json::to_string(&res);
        println!("resp:{:?}",r);
        return r;
    }).map_err(|e|{
        println!("error:{}",e);
        Error::from(e)
    })
        .and_then(|user|{
            println!("user:{}",user);
            Ok(HttpResponse::Ok().content_type("application/json").body(user))
        })
}

///构建一个graphql的查询界面。http://localhost:8080/graphiql
/// 在面板的左边输入：
/*
 query GetTodos{
     todos{
         id
         title
         completed
     }
  }
 */
fn graphiql() ->HttpResponse {
    let html = graphiql_source("http://localhost:8080/graphql");
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("failed to connect to {}",database_url))
}


