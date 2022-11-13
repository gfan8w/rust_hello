#[warn(unused_imports)]
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use hyper::{Request, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
#[warn(unused_imports)]
use route_recognizer::Params;
use super::{Error, Context, AppState, Response, Router, MiddleWare, Middleware, Next, Handler};


macro_rules! register_method {
    ($method_name: ident, $method_def: expr) => {
        #[allow(unused)] // suppress warning: unused variable， #[allow(unused)] 可以替代 2个：#[warn(unused_variables)]、#[allow(dead_code)]
        pub fn $method_name(&mut self, path: impl AsRef<str>, handler: impl Handler) {
            self.router.register($method_def, path, handler)
        }
    };
}

pub struct Server {
    router: Router,
    middlewares: MiddleWare,
}

impl Server {
    pub fn new() -> Self {
        Server {
            router: Router::new(),
            middlewares: MiddleWare::new(),
        }
    }

    pub fn middleware(&mut self, middleware: impl Middleware) {
        self.middlewares.add(middleware);
    }

    register_method!(get, Method::GET);
    register_method!(head, Method::HEAD);
    register_method!(post, Method::POST);
    register_method!(put, Method::PUT);
    register_method!(delete, Method::DELETE);
    register_method!(connect, Method::CONNECT);
    register_method!(options, Method::OPTIONS);
    register_method!(trace, Method::TRACE);
    register_method!(patch, Method::PATCH);


    pub async fn run(self, addr: SocketAddr) -> Result<(), Error> {

        let some_state = "state".to_string();

        let Self {
            router,
            middlewares,
        } = self;

        let router = Arc::new(router);
        let middlewares = Arc::new(middlewares);

        let make_svc = make_service_fn(|conn: &hyper::server::conn::AddrStream| {
            let router = router.clone();
            let middlewares = middlewares.clone();
            let remote_addr = conn.remote_addr();

            let app_state = AppState {
                state_thing: some_state.clone(),
            };


            async move {
                Ok::<_, Error>(service_fn(move |req| {
                    let router = router.clone();
                    let middlewares = middlewares.get();
                    let app_state_inner = app_state.clone();
                    async move {
                        let endpoint = router.find(req.uri().path(), &req.method());

                        let next = Next {
                            endpoint: endpoint.handler,
                            next_middleware: &middlewares,
                        };

                        let ctx = Context::new(app_state_inner, req, endpoint.params, remote_addr);

                        let resp = next.run(ctx).await;

                        Ok::<_, Error>(resp)
                    }
                }))
            }
        });

        let server = hyper::Server::bind(&addr).serve(make_svc);
        let graceful = server.with_graceful_shutdown(shutdown_signal());

        println!("Listening on //{}", addr);
        graceful.await.map_err(|e| Error::from(format!("server run error: {:?}", e)))?;
        Ok(())
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .map(|m|println!("exit graceful {:?}",m))
        .expect("failed to install CTRL+C signal handler");
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}





