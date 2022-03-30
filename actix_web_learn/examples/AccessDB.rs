

//访问DB

use actix_web_learn::*;
use diesel::prelude::*;
use diesel::sql_query;
use std::future::{Ready, ready};
use std::sync::Mutex;
use actix_web::{App, Either, Error, get, HttpRequest, HttpResponse, HttpServer, Responder, web};
use chrono::{FixedOffset, Local, TimeZone};
use futures::future::ok;
use futures::stream::once;
use serde::Serialize;
use actix_web_learn::model::Users;

// http://localhost:8080/data/insert/lllltttt
// 插入一个数据后，在查询，以json格式返回
async fn create_user(pool: web::Data<PoolConnection>, name: web::Path<(String)>) -> impl Responder {
    let name = name.into_inner();

    let conn = pool.get().expect("couldn't get db connection from poo");
    let r = web::block(move ||{
        use schema::users;
        //use actix_web_learn::schema::users::dsl::*;

        let mut user = model::UserForInsert {
            name: "name",
            hair_color: Some("blank"),
        };
        if name!="" {
            user.name=&name;
        }
        diesel::insert_into(users::table)
            .values(user)
            .execute(&conn)

    }).await;

    if let Err(e) =r {
        String::from(format!("error when insert user:{}",e));
    }else {
        String::from("insert user success");
    }

    let conn2 =pool.get().expect("connection can't get from pool");

    let tz_offset = FixedOffset::east(8 * 3600);

    let u =web::block(move ||{
        let mut users = sql_query("select *, CONCAT(name, hair_color) as fullname from users ")
            .load::<Users>(&conn2);


        let  aa =users.as_mut().unwrap();
        for mut x in aa {
            let new_created_at = tz_offset.from_utc_datetime(&x.created_at);
            x.created_at=new_created_at.naive_local();
            //x.created_at =chrono::offset::Local::now().naive_local()
        }

        // users.as_mut().unwrap().push(Users{
        //     id: 100,
        //     name: "aa".to_string(),
        //     hair_color: None,
        //     created_at: chrono::offset::Local::now().naive_local(),
        //     updated_at: chrono::offset::Local::now().naive_local(),
        //     fullname: chrono::offset::Local::now().naive_local().to_string()
        // });

        println!("{:?}",users.as_ref());
        users
    }).await;

    let mut aa :Vec<Users> =vec![];
    aa.push(Users{
        id: 100,
        name: "ABLINT".to_string(),
        hair_color: None,
        created_at: chrono::offset::Local::now().naive_local(),
        updated_at: Local::now(),
        fullname: chrono::offset::Local::now().naive_local().to_string()
    });

    //直接返回 String
    //serde_json::to_string(&u.unwrap()).unwrap()
    web::Json(u.unwrap())
    //web::Json(aa)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("server started");
    println!("{:?}", std::env::current_exe());
    let pool = new_connection_pool();

    //HttpServer 的构造函数 以 App 工厂作为参数（必须实现Send + Sync），然后使用 bind 绑定端口，run 函数将启动 Http Server
    HttpServer::new(move||{

        // actix_web::App 是 actix_web 的核心，所有的路由、服务、共享数据都围绕 App 构建。
        // 在 actix_web 中，每个线程持有一个 App 实例。在创建 Server 时，需要传递一个 App 的 工厂函数
        App::new()
            // 由于 HttpServer::new 接收的是 App 工厂函数
            // 所以不同线程的 data 不是同一个实例，所以不是进程级别共享数据，而是线程级别的共享数据
            // 因此只能用于访问只读数据，如全局配置等
            .data(pool.clone())
            .service(
                web::scope("/data")
                    .route("/insert/{name}", web::get().to(create_user)))
    }).workers(4)   // 默认情况下，HttpServer 以多线程方式 启动 Server，线程为等于当先系统的核心数。指定线程数。
        .bind("0.0.0.0:8080")?
        .run()
        .await
}


























