use crate::{Context, Response};
use async_trait::async_trait;
use hyper::{Body, Method, StatusCode};
use route_recognizer::{Match, Params, Router as InternalRouter};
use std::collections::HashMap;
use std::future::Future;
use bytes::Bytes;


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

pub struct Router {
    method_map: HashMap<Method, InternalRouter<Box<dyn Handler>>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            method_map: HashMap::default(),
        }
    }

    // 改为由宏来实现，否则每次要实现好多个
    // pub fn get(&mut self, path: &str, handler: Box<dyn Handler>) {
    //     self.method_map
    //         .entry(Method::GET)
    //         .or_insert_with(InternalRouter::new)
    //         .add(path, handler)
    // }
    //
    // pub fn post(&mut self, path: &str, handler: Box<dyn Handler>) {
    //     self.method_map
    //         .entry(Method::POST)
    //         .or_insert_with(InternalRouter::new)
    //         .add(path, handler)
    // }

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

    // if let Some(Match { handler, params }) = self
    //     .method_map
    //     .get(method)
    //     .and_then(|r| r.recognize(path).ok())
    // {
    //     RouterMatch {
    //         handler: &**handler,
    //         params,
    //     }
    // } else {
    //     RouterMatch {
    //         handler: &not_found_handler,
    //         params: Params::new(),
    //     }
    // }
}


impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

async fn not_found_handler(_cx: Context) -> Response {
    hyper::Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("NOT FOUND"))
        .unwrap()
}

pub trait IntoResponse: Send + Sized {
    fn into_response(self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response {
        Response::new(self.into())
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response::new(self.into())
    }
}
