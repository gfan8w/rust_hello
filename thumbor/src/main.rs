use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr::hash;
use std::sync::{Arc};
use tokio::sync::Mutex;
use axum::extract::{Extension, Path};
use axum::Router;
use percent_encoding::{percent_decode, percent_decode_str};
use reqwest::{StatusCode, Url};
use serde::Deserialize;
use axum::handler::get;
use axum::http::{HeaderMap, HeaderValue};
use bytes::Bytes;
use lru::LruCache;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tracing::{info, instrument};

mod pb;
mod engine;

pub use pb::ImageSpec;
use crate::pb::{filter, resize, Spec};

///参数使用 serde 做 Deserialize，axum 会自动识别并解析
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String
}



















#[instrument(level="info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> anyhow::Result<Bytes> {
    let mut hasher =DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();

    let g =&mut cache.lock().await;

    let data =match g.get(&key) {
        Some(v) => {
            println!("Match cache:{}", key);
            v.to_owned()
        },
        None => {
            info!("Cahe miss, retrieve from url");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        }
    };

    Ok(data)

}





async fn generate(
    Path(Params {spec, url}): Path<Params>,
    Extension(cache): Extension<Cache>) -> Result<(HeaderMap, Vec<u8>), StatusCode> {

    let url = percent_decode_str(&url).decode_utf8_lossy();

    let spec: ImageSpec =spec.as_str().try_into().map_err(|_| StatusCode::BAD_REQUEST)?;

    let str =format!("url:{}\n spec:{:#?}", url, spec);

    let images =retrieve_image(&url, cache).await
        .map_err(|_|StatusCode::BAD_REQUEST)?;

    let mut header =HeaderMap::new();
    header.insert("content-type", HeaderValue::from_static("image/jpeg"));

    Ok((header, images.to_vec()))


}





type Cache = Arc<Mutex<LruCache<u64,Bytes>>>;


#[tokio::main]
async fn main() {
    println!("Hello, thumbor!");

    //初始化 tracing
    tracing_subscriber::fmt::init();

    let cache: Cache =Arc::new(Mutex::new(LruCache::new(1024)));

    //构建路由
    let app = Router::new()
        // `GET /image` 会执行 generate 函数，并把 spec 和 url 传递过去
        .route("/image/:spec/:url", get(generate))
        .layer(ServiceBuilder::new()
            .layer(AddExtensionLayer::new(cache))
            .into_inner());

    // 运行 web 服务器
    let addr ="127.0.0.1:3001".parse().unwrap();
    tracing::debug!("listenig on: {}", addr);
    print_test_url("https://images.pexels.com/photos/1562477/pexels-photo-1562477.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();














}


fn print_test_url(url: &str) {
    let spec1 =Spec::new_resize(500,800,resize::SampleFilter::CatmullRom);
    let spec2 =Spec::new_watermask(20,20);
    let spec3 =Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1,spec2,spec3]);
    let s: String =image_spec.borrow().into();
    let test_image =percent_encoding::percent_encode(url.as_bytes(),
                                                     percent_encoding::NON_ALPHANUMERIC)
        .to_string();
    println!("test url: http://localhost:3001/image/{}/{}", s, test_image);
}






