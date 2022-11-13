use std::time::Instant;
use super::{Next, Response, Context};
use super::Middleware;

pub struct ResponseTime;

#[async_trait::async_trait]
impl Middleware for ResponseTime {
    async fn handle<'a>(&'a self, ctx: Context, next: Next<'a>) -> Response {
        let start = Instant::now();
        let method = ctx.request.method().to_string();
        let path = ctx.request.uri().path().to_string();
        let remote_addr = ctx.remote_addr;
        let res = next.run(ctx).await;
        println!(
            "{} {:?} {} {} {}ms",
            method,
            path,
            res.status().as_str(),
            remote_addr,
            start.elapsed().as_millis()
        );
        res
    }
}