use async_trait::async_trait;
use hyper::{Body, Method, Request, StatusCode};
use route_recognizer::{Match, Params, Router as InternalRouter};
use std::collections::HashMap;
use std::future::Future;


fn main() {
    let some_state = "state".to_string();

    let mut srv = Router::new();

    srv.register(Method::GET, "/test", test_handler);
}

async fn test_handler(_cx: Context) -> Response {
    hyper::Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("FOUND"))
        .unwrap()
}

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

pub type Response = hyper::Response<hyper::Body>;
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;


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
pub struct AppState {
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
        Fut: Future + Send + 'static, // 对于 Response的约束还以：
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