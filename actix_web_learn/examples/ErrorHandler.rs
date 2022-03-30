use std::fmt::{Display, Formatter};
use actix_web::{HttpRequest, HttpServer, web, App, error, HttpResponse, FromRequest, Result,middleware::Logger};
use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use serde::Deserialize;
use derive_more::{Display, Error};
use log::info;

// 演示如何处理错误

#[derive(Display,Debug,Error)]
#[display(fmt="my error:{}",name)]
struct MyError {
    name: &'static str,
}


// Use default implementation for `error_response()` method
// ResponseError有一个默认的实现，如果发生了内部错误，会显示500 服务器内部错误
impl error::ResponseError for MyError {}


// curl -i http://localhost:8088/error/index
//访问index 页面会有错误，MyError会显示为 500，调用内部默认实现
async fn index() -> Result<&'static str, MyError> {
    Err(MyError{name:"my error throwed"})
}

#[derive(Debug, Display, Error)]
enum CustomError{
    #[display(fmt="Innternal server got a error")]
    InnternalError,
    #[display(fmt="bad request goot from client")]
    BadClientData,
    #[display(fmt="timeout occur")]
    Timeout,
}

impl error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            CustomError::InnternalError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::BadClientData => StatusCode::BAD_REQUEST,
            CustomError::Timeout=>StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .set(ContentType::html())
            .body(self.to_string())
    }

}

async fn index2() -> Result<&'static str, CustomError> {
    Err(CustomError::BadClientData)
}

//错误类型转换，从一个错误转变为另外一个错误
async fn index3() -> Result<&'static str> {
    let result: Result<&'static str, _> =Err(MyError{name:"I am error type of MyError"});
    let bb =result.map_err(|e|{
        error::ErrorBadGateway(e.name)
    })?;
    Ok(bb)
}

//通过log，env_log记录日志
async fn index4() -> Result<&'static str, MyError> {
    let myerror = MyError{name: "I have an error, I can't find db"};
    info!("{}", myerror);
    Err(myerror)
}


// actix web 的 默认在 WARN，所以如果开启RUST_BACKTRACE，log日志级别设置为 DEBUG，
// RUST_BACKTRACE=1 RUST_LOG=actix_web=debug cargo run
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG","info");
    std::env::set_var("RUST_BACKTRACE","1");

    env_logger::init();


    HttpServer::new(move ||{
        let logger = Logger::default();

        App::new()
        // 配置 Json Extractor
            .wrap(logger)
            .service(web::scope("/error")
                .route("/index", web::get().to(index))
                .route("/index2", web::get().to(index2))
                     .route("/index3", web::get().to(index3))
                         .route("/index4", web::get().to(index4)))

    }).bind("127.0.0.1:8088")?
        .run()
        .await

}


