use std::net::SocketAddr;
use bytes::Bytes;
use hyper::{Body, Request};
use hyper::body::to_bytes;
use route_recognizer::Params;

pub type Response = hyper::Response<hyper::Body>;
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;


#[derive(Debug)]
pub struct Context {
    pub state: AppState,
    pub request: Request<Body>,
    pub params: Params,
    pub remote_addr: SocketAddr,
    body_bytes: Option<Bytes>,
}

impl Context {
    pub fn new(state: AppState, req: Request<Body>, params: Params, remote_addr: SocketAddr) -> Context {
        Context {
            state,
            request: req,
            params,
            remote_addr,
            body_bytes: None,
        }
    }

    pub async fn body_json<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Error> {
        let body_bytes = match self.body_bytes {
            Some(ref v) => v,
            _ => {
                let body = to_bytes(self.request.body_mut()).await?;
                self.body_bytes = Some(body);
                self.body_bytes.as_ref().expect("body_bytes was set above")
            }
        };
        Ok(serde_json::from_slice(&body_bytes)?)
    }
}

/// put your extra data here, if you want to pass data through the http request chain
#[derive(Clone, Debug)]
pub struct AppState {
    pub state_thing: String,
}