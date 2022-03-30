

//处理器是一个异步函数：async fn(p: impl FromRequest) -> impl Responder，其中参数p的数目支持0~10个
//
// 关于 FromRequest 参见下一节 提取器
//
// actix-web 为某些标准类型（例如＆'static str、String等）提供 Responder 实现。
//
// 本example是关于不同 Responder 的实验



use std::future::{Ready, ready};
use std::sync::Mutex;
use actix_web::{App, Either, Error, get, HttpRequest, HttpResponse, HttpServer, Responder, web};
use futures::future::ok;
use futures::stream::once;
use serde::Serialize;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    //HttpServer 的构造函数 以 App 工厂作为参数（必须实现Send + Sync），然后使用 bind 绑定端口，run 函数将启动 Http Server
    HttpServer::new(move||{

        // actix_web::App 是 actix_web 的核心，所有的路由、服务、共享数据都围绕 App 构建。
        // 在 actix_web 中，每个线程持有一个 App 实例。在创建 Server 时，需要传递一个 App 的 工厂函数
        App::new()
            // 由于 HttpServer::new 接收的是 App 工厂函数
            // 所以不同线程的 data 不是同一个实例，所以不是进程级别共享数据，而是线程级别的共享数据
            // 因此只能用于访问只读数据，如全局配置等

            .service(
                web::scope("/responder")
                    .route("/str", web::get().to(responder_str))
                    .route("/string", web::get().to(responder_string))
                    .route("/impl_responder", web::get().to(responder_impl_responder))
                    .route("/custom_responder", web::get().to(responder_custom_responder))
                    .route("/stream", web::get().to(responder_stream_responder))
                    .route("/either", web::get().to(responder_either_responder))
            )


    }).workers(4)   // 默认情况下，HttpServer 以多线程方式 启动 Server，线程为等于当先系统的核心数。指定线程数。
        .bind("0.0.0.0:8080")?
        .run()
        .await
}



async fn responder_str() -> &'static str {
    "responder_str"
}

async fn responder_string() -> String {
    "responder_string".to_string()
}

async fn responder_impl_responder() -> impl Responder {
    web::Bytes::from_static(b"responder_bytes")
}

// curl http://localhost:8088/responder/custom_responder
async fn responder_custom_responder() -> impl Responder {
    ResponseWrapper {
        code: 0,
        msg: "ok".to_string(),
        data: Some(Data{name:"nick".to_string(), age:20})
    }
}

#[derive(Serialize)]
struct Data {
    name: String,
    age: u8
}

// 自定义 Response
#[derive(Serialize)]
struct ResponseWrapper<T> {
    code: i32,
    msg: String,
    data: Option<T>
}

impl<T> Responder for ResponseWrapper<T> where T:Serialize {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok().content_type("application/json").body(body)))
    }
}

// curl http://localhost:8088/responder/stream
async fn responder_stream_responder() -> impl Responder {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"a very long stream")));
    HttpResponse::Ok().content_type("application/json").streaming(body)
}


type RegisterResult =Either<HttpResponse,Result<&'static str, Error>>;

// curl http://localhost:8088/responder/either
async fn responder_either_responder() -> RegisterResult {
    Either::A(HttpResponse::BadRequest().body("bad bad very bad request"))
    //Either::B(Ok("hello"))
}
























