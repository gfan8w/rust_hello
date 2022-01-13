use anyhow::{anyhow, Result};
use tokio::fs;
use async_trait::async_trait;

/// 从文件源或者 http 源中获取数据，组成 data frame
/// 这个写法，扩展性不好。拿到数据后，如果要做处理加工，就必须修改这个函数，应该加一个中间层。加中间层最好的方式就是用trait
pub async fn retrieve_data_1(source: impl AsRef<str>) -> Result<String> {
    let name =source.as_ref();
    match &name[..4] {
        // 包括http 或 https, http(s)://
        "http" =>{
            Ok(reqwest::get(name).await?.text().await?)
        },
        "file" => {
            // file://path/to/file.txt
            Ok(fs::read_to_string(&name[7..]).await?)
        },
        _ => Err(anyhow!("we only support http or file"))
    }
}


// Rust 的 async trait 还没有稳定，可以用 async_trait 宏
#[async_trait]
pub trait Fetcher {
    type Error;
    async fn fetch(&self) -> Result<String,Self::Error>;
}


pub struct HttpFetcher<'a>(pub(crate) &'a str);
pub struct FileFetcher<'a>(pub(crate) &'a str);

#[async_trait]
impl<'a> Fetcher for HttpFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(reqwest::get(self.0).await?.text().await?)
    }
}

#[async_trait]
impl<'a> Fetcher for FileFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}

/// 从文件源或者 http 源中获取数据，组成 data frame
pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
    let name =source.as_ref();
    match &name[..4] {
        // 包括http 或 https, http(s)://
        "http" =>{
            Ok(HttpFetcher(&name).fetch().await?)
        },
        "file" => {
            // file://path/to/file.txt
            Ok(FileFetcher(&name[7..]).fetch().await?)
        },
        _ => Err(anyhow!("we only support http or file"))
    }
}













