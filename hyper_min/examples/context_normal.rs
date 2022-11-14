
/*
[dependencies]
hyper = { version = "0.14", features = ["full"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
route-recognizer = "0.2"
async-trait = "0.1"
*/
use async_trait::async_trait;
use hyper::{Body, Method, Request, StatusCode};
use route_recognizer::{Params, Router as InternalRouter};
use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use hyper::service::{make_service_fn, service_fn};

#[tokio::main]
async fn main() {
    let mut srv = Router::new();
    let some_state = "state".to_string();

    srv.register(Method::GET, "/test", test_handler);

    let srv = Arc::new(srv);
    let make_svc = make_service_fn(|conn: &hyper::server::conn::AddrStream| {
        let app_state = AppState {
            state_thing: some_state.clone(),
        };

        let srv = srv.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                let srv = srv.clone();
                let app_state_inner = app_state.clone();
                async move {
                    let endpoint = srv.find(req.uri().path(), &req.method());

                    let ctx = Context::new(app_state_inner, req, endpoint.params);

                    let resp = endpoint.handler.invoke(ctx).await;
                    Ok::<_, Error>(resp)
                }
            }))
        }
    });
    let addr = "0.0.0.0:8080".parse::<SocketAddr>().unwrap();
    let server = hyper::Server::bind(&addr).serve(make_svc);
    server.await.expect("error");
}

async fn test_handler(_cx: Context) -> Response {
    hyper::Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("FOUND"))
        .unwrap()
}

pub type Response = hyper::Response<hyper::Body>;
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;



pub struct Router {
    method_map: HashMap<Method, InternalRouter<Box<dyn Handler>>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            method_map: HashMap::default(),
        }
    }

    pub fn register(
        &mut self,
        method: Method,
        path: impl AsRef<str>,
        handler: impl Handler,
    ) {
        self.method_map
            .entry(method)
            .or_insert_with(InternalRouter::new)
            .add(path.as_ref(), Box::new(handler));
    }


    pub fn find(&self, path: &str, method: &Method) -> RouterMatch<'_> {
        let endpoint = match self.method_map.get(method) {
            Some(router) => match router.recognize(path) {
                Ok(route_recognizer::Match { handler, params }) => {
                    RouterMatch {
                        handler: &**handler,
                        params,
                    }
                }
                Err(_) => RouterMatch {
                    handler: &not_found_handler,
                    params: Params::new(),
                },
            },
            None => RouterMatch {
                handler: &not_found_handler,
                params: Params::new(),
            }
        };
        endpoint
    }
}

async fn not_found_handler(_cx: Context) -> Response {
    hyper::Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("NOT FOUND"))
        .unwrap()
}


#[derive(Debug)]
pub struct Context {
    pub state: AppState,
    pub request: Request<Body>,
    pub params: Params,
}

impl Context {
    pub fn new(state: AppState, req: Request<Body>, params: Params) -> Context {
        Context {
            state,
            request: req,
            params,

        }
    }
}

/// put your extra data here, if you want to pass data through the http request chain
#[derive(Clone, Debug)]
pub struct AppState {  // I would like change to generics version to support any kind of data
    pub state_thing: String,
}


#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn invoke(&self, context: Context) -> Response;
}

#[async_trait]
impl<F: Send + Sync + 'static, Fut> Handler for F
    where
        F: Fn(Context) -> Fut,
        Fut: Future + Send + 'static,
        Fut::Output: IntoResponse,
{
    async fn invoke(&self, context: Context) -> Response {
        (self)(context).await.into_response()
    }
}

pub struct RouterMatch<'a> {
    pub handler: &'a dyn Handler,
    pub params: Params,
}


pub trait IntoResponse: Send + Sized {
    fn into_response(self) -> Response;
}
impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}