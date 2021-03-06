use actix_web::{App, get, HttpResponse,HttpServer,middleware::Logger,Responder, web,};
use actix_web::web::{get,post,delete, resource, scope};

mod kv_store;
mod kv_handler;

///处理函数应该是一个异步函数，返回一个 实现了 Responder 的类型
/// #[get("/")] 可以方便的配置请求宏，
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rust service ok")
}

#[get("/healthcheck")]
async fn healthCheck() -> impl Responder {
    HttpResponse::Ok().body("I'm alive")
}

/// 参考：https://levelup.gitconnected.com/using-rocksdb-with-rust-and-actix-web-98507c9db267
/// #[actix_rt::main] 用于生成异步函数运行时，需要引入 actix-rt = "1.0" 依赖
#[actix_rt::main]
async fn main() -> std::io::Result<()>{

    let db: kv_store::RocksDb = kv_store::KVStore::init("./data");

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .wrap(Logger::default())
            .service(
                scope("/api").service(resource("/{key}")
                    .route(get().to(kv_handler::get))
                    .route(post().to(kv_handler::post))
                    .route(delete().to(kv_handler::delete))))
            .service(
                web::scope("")
                    .service(index)
                    .service(healthCheck))
    }).bind("0.0.0.0:8080")?
        .run()
        .await


}

