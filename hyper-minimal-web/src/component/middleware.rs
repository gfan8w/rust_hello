use std::sync::Arc;
use super::{Handler, Response, Context};


#[async_trait::async_trait]
pub trait Middleware<T>: Send + Sync + 'static {
    async fn handle<'a>(&'a self, ctx: Context<T>, next: Next<'a, T>) -> Response;
}

#[allow(missing_debug_implementations)]
pub struct Next<'a, T> {
    pub(crate) endpoint: &'a dyn Handler<T>,
    pub(crate) next_middleware: &'a [Arc<dyn Middleware<T>>],
}

impl<'a, T: 'static> Next<'a, T> {
    /// Asynchronously execute the remaining middleware chain.
    pub async fn run(mut self, ctx: Context<T>) -> Response {
        if let Some((current, next)) = self.next_middleware.split_first() {
            self.next_middleware = next;
            current.handle(ctx, self).await
        } else {
            (self.endpoint).invoke(ctx).await
        }
    }
}


pub struct MiddleWare<T> {
    middlewares: Vec<Arc<dyn Middleware<T>>>,
}


impl<T: Send + Sync + 'static> MiddleWare<T> {
    pub fn new() -> MiddleWare<T> {
        MiddleWare {
            middlewares: Vec::new(),
        }
    }
    pub fn add(&mut self, middleware: impl Middleware<T>) {
        self.middlewares.push(Arc::new(middleware));
    }

    pub fn get(&self) -> Vec<Arc<dyn Middleware<T>>>{
        self.middlewares.clone()
    }
}

impl<T: Send + Sync + 'static> Default for MiddleWare<T> {
    fn default() -> Self {
        Self::new()
    }
}