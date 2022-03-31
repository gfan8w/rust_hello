use actix_web::{web, App, HttpServer, middleware, HttpResponse};
use actix_web::web::{Data, ServiceConfig};
use std::time::Duration;
use sea_orm::ConnectOptions;
use tracing_subscriber::filter::LevelFilter;
use handler::prelude::todos_handler;
use repository::prelude::Repository;

mod entity;
mod repository;
mod handler;


/*
1)  安装 cargo install sqlx-cli ;
2)  创建db: sqlx database create --database-url mysql://root:123456@localhost/seaorm-db
3)  sqlx migrate add seaorm-db  建立migrations目录，写入sql。
4)  sqlx migrate run --database-url mysql://root:123456@localhost/seaorm-db  创建表
    sqlx migrate revert --database-url mysql://root:123456@localhost/seaorm-db
5） 安装 cargo install sea-orm-cli
6） 产生 entity：
    sea-orm-cli generate entity \
    --database-url mysql://root:123456@localhost/seaorm-db \
    -o src/entity
*/


//会被多线程共享，需要有clone
#[derive(Debug, Clone)]
struct AppState {
    repository: Repository,
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .with_test_writer()
        .init();

    dotenv::from_filename(".seaorm-db.env").expect("can't find .seaorm-db.env file");
    let db_url = std::env::var("DATABASE_URL").expect("can't get DATABASE_URL in env file");
    let host = std::env::var("HOST").expect("can't get HOST in env file");
    let port = std::env::var("PORT").expect("can't get PORT in env file");
    let server_url = format!("{}:{}", host,port);

    //app_state init
    let mut dbOpt =ConnectOptions::new(db_url);
    let aa = dbOpt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let db =sea_orm::Database::connect(dbOpt).await.unwrap();

    let app_state =AppState{
        repository: Repository {
            db_conn:db,
        }
    };

    let server = HttpServer::new(move ||{
        App::new()
            .app_data(Data::new(app_state.clone()))
            .wrap(middleware::Logger::default())
            //.route("/",web::get().to(|_|HttpResponse::Ok().body("hello")))
            .configure(init)

    }).bind(&server_url)?;

    // http://localhost:8000/todos/
    println!("server started:{}",&server_url);

    server.run().await?;

    Ok(())



}


fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(todos_handler());
}

