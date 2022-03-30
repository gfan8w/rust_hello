use std::fmt::{Display, Formatter};
use actix_web::{HttpRequest, HttpServer,dev, web, App, error, HttpResponse, FromRequest, Result,middleware,middleware::{Logger}};
use actix_web::http::{HeaderValue, StatusCode};
use actix_web::http::header::{CONTENT_TYPE, ContentType};
use serde::Deserialize;
use derive_more::{Display, Error};
use log::info;
use actix_session::{CookieSession, Session};
use actix_web::middleware::errhandlers::{ErrorHandlers, ErrorHandlerResponse};
use actix_web::dev::Service;
use futures::future::FutureExt;

// 演示中间件，https://actix.rs/docs/middleware/



async fn cookie_session(session: Session) ->Result<HttpResponse, error::Error> {
    // access session data
    let SESSION_COUNTER: String =String::from("counter");

    if let Some(acnt) =session.get::<i32>(&SESSION_COUNTER)? {
        info!("session:{}",acnt);
        println!("print session:{}",acnt);
        session.set(&SESSION_COUNTER, acnt+1);
    }else {
        session.set(&SESSION_COUNTER, 1);
    }

    Ok(HttpResponse::Ok().body(format!("counter is {:?}!",session.get::<i32>(&SESSION_COUNTER).unwrap())))
}

async fn index2() -> Result<&'static str> {
    Err(error::ErrorInternalServerError("service has an error"))
}

async fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("Error"));
    Ok(ErrorHandlerResponse::Response(res))
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
            .wrap(middleware::DefaultHeaders::new().header("x-Version","4.0")) // 内置中间件——Default headers
            .wrap(CookieSession::signed(&[0;32]).secure(false)) //创建一个session中间件
            //.wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR,add_error_header))
            //使用  wrap_fn 注册简单的中间件
            .wrap_fn(|req,srv|{
                info!("Hi from start, You requested:{}",req.path());
                srv.call(req).map(|res|{
                    info!("hi from response");
                    res
                })

            })
            .wrap_fn(|req,srv|{
                info!("Hi from start, a path is:{}",req.path());
                let fut =srv.call(req);
                async {
                    let res = fut.await;
                    info!("got a response");
                    res
                }
            })
            .service(web::scope("/middle")
                .route("/index", web::get().to(cookie_session))
                .route("/error", web::get().to(index2))
                     )

    }).bind("127.0.0.1:8088")?
        .run()
        .await

}


