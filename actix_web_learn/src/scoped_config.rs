use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};

//actix_web 提供了 configure 用来传递一个配置函数，这样就可以将实现拆分到不同的模块中实现。
// 该配置函数 传递一个参数 ServiceConfig，该参数可以配置自己的 data, routes, 和 services。

// curl http://localhost:8088/app/test
pub fn app3_scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/test")
        .route(web::get().to(|| HttpResponse::Ok().body("test ~~~")))
        .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
        );
}

// curl http://localhost:8088/app2
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/app2")
        .route(web::get().to(|| HttpResponse::Ok().body("app2")))
        .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
    );
}





