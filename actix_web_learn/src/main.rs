use diesel::prelude::*;
use dotenv::dotenv;
use diesel::r2d2;
use std::env;

use std::sync::Mutex;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use actix_web_learn::scoped_config;

///处理函数应该是一个异步函数，返回一个 实现了 Responder 的类型
// curl http://localhost:8088/
async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

// curl http://localhost:8088/again
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("hello world again")
}

// curl http://localhost:8088/admin/index.html
async fn admin_index() -> impl Responder {
    HttpResponse::Ok().body("Welcome admin!")
}

// curl http://localhost:8088/admin/hello
///#[get("/hello")] 可以方便的配置请求宏, 这是 actix_web::get(), 不是actix_web::web::get()
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}





///#[actix_rt::main] 用于生成异步函数运行时，需要引入 actix-rt = "1.0" 依赖
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let counter =web::Data::new(AppSafeCounter {counter: Mutex::new(0)});

    //HttpServer 的构造函数 以 App 工厂作为参数（必须实现Send + Sync），然后使用 bind 绑定端口，run 函数将启动 Http Server
    HttpServer::new(move||{

        // actix_web::App 是 actix_web 的核心，所有的路由、服务、共享数据都围绕 App 构建。
        // 在 actix_web 中，每个线程持有一个 App 实例。在创建 Server 时，需要传递一个 App 的 工厂函数
        App::new()
            // 由于 HttpServer::new 接收的是 App 工厂函数
            // 所以不同线程的 data 不是同一个实例，所以不是进程级别共享数据，而是线程级别的共享数据
            // 因此只能用于访问只读数据，如全局配置等
            .data(AppState {app_name: String::from("actix-web")})
            .app_data(counter.clone())
            .route("/",web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(web::scope("/admin") // 一个 App 可以通过 scope，为路由添加统一的前缀。
                .route("/index.html", web::get().to(admin_index))
                .service(hello))
            .route("/app_state",web::get().to(app_state))
            .route("/counter", web::get().to(app_counter))
            .configure(scoped_config::config)  // 提供了 configure 用来传递一个配置函数，这样就可以将实现拆分到不同的模块中实现。
            .service(web::scope("/app").configure(scoped_config::app3_scoped_config))


    }).workers(4)   // 默认情况下，HttpServer 以多线程方式 启动 Server，线程为等于当先系统的核心数。指定线程数。
        .bind("0.0.0.0:8080")?
        .run()
        .await

    // 由于 actix_web 是异步的，所以要防止阻塞的发生，阻塞将大大降低系统的吞吐量。所以，所有 IO 操作都需要使用对应的异步版本来实现。
    // 默认情况下，actix_web 当收到 SIGTERM 信号时，将优雅关机，关机时间超时默认30s，可以通过HttpServer::shutdown_timeout() 配置
}

///actix_web 提供了 web::Data API 用来在程序间共享状态
/// 线程级别共享，共享的类型不用实现 线程交换安全，只能用于只读，如全局配置。通过 .data(T) 初始化
/// 进程级别共享，共享的类型需要实现 线程交换安全，可用于读写场景，如计数器。通过 .app_data(T) 初始化
struct AppState{
    app_name: String,
}

// curl http://localhost:8088/app_state
async fn app_state(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(&data.app_name)
}

pub struct AppSafeCounter{
    counter: Mutex<i32>,  // <- Mutex is necessary to mutate safely across threads
}

// curl http://localhost:8088/counter
async fn app_counter(data: web::Data<AppSafeCounter>) -> impl Responder {
    let mut counter =data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter+=1; // <- access counter inside MutexGuard
    HttpResponse::Ok().body(format!("request count:{}",counter))
}


pub type PoolConnection = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;

///创建连接池
pub fn new_connection_pool() -> PoolConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.");
    pool
}






















