use std::sync::Arc;
use super::{Handler, Response, Context};


#[async_trait::async_trait]
pub trait Middleware: Send + Sync + 'static {
    async fn handle<'a>(&'a self, ctx: Context, next: Next<'a>) -> Response;
}

#[allow(missing_debug_implementations)]
pub struct Next<'a> {
    pub(crate) endpoint: &'a dyn Handler,
    pub(crate) next_middleware: &'a [Arc<dyn Middleware>],
}

impl<'a> Next<'a> {
    /// Asynchronously execute the remaining middleware chain.
    pub async fn run(mut self, ctx: Context) -> Response {
        if let Some((current, next)) = self.next_middleware.split_first() {
            self.next_middleware = next;
            current.handle(ctx, self).await
        } else {
            (self.endpoint).invoke(ctx).await
        }
    }
}


pub struct MiddleWare {
    middlewares: Vec<Arc<dyn Middleware>>,
}


impl MiddleWare {
    pub fn new() -> MiddleWare {
        MiddleWare {
            middlewares: Vec::new(),
        }
    }
    pub fn add(&mut self, middleware: impl Middleware) {
        self.middlewares.push(Arc::new(middleware));
    }

    pub fn get(&self) -> Vec<Arc<dyn Middleware>>{
        self.middlewares.clone()
    }
}

impl Default for MiddleWare {
    fn default() -> Self {
        Self::new()
    }
}