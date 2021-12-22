use std::collections::HashMap;
use std::str::FromStr;
use clap::{AppSettings, Parser};
use anyhow::{anyhow, Result};
use mime::Mime;
use colored::Colorize;
use reqwest::{header, Client, Response, Url};
use reqwest::header::HeaderValue;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

// 定义 HTTPie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

#[derive(Parser, Debug)]
#[clap(version = "1.0", author="lint")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

// get 子命令

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String
}

#[derive(Parser, Debug)]
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>
}

///命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug,PartialEq)]
struct KvPair {
    k:String,
    v:String
}


fn parse_url(s: &str) -> Result<String> {
    // 这里我们仅仅检查一下 URL 是否合法
    let _url:Url =s.parse()?; //parse 会自动调用 FromStr：：from_str
    Ok(s.into())
}

/// 因为我们为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}

/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // 使用 = 进行 split，这会得到一个迭代器
        let mut split=s.split("=");
        let err =||anyhow!(format!("failed to parse {}",s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            k:(split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v:(split.next().ok_or_else(err)?).to_string(),
        })
    }
}


async fn get(client:Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    println!("get resp:{:?}",resp.text().await?);
    Ok(())
}

async fn post(client:Client, args: &Post) -> Result<()>{
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }

    //设置一些头部信息
    let mut headers = header::HeaderMap::new();
    // 这里演示了如何强制类型转换，一个用的TryInto，另外一个用的parse
    headers.insert("X-POWERED-BY",TryInto::<HeaderValue>::try_into("Rust")?);
    headers.insert(header::USER_AGENT,"Rust httpie".parse::<HeaderValue>().unwrap());


    let resp =client.post(&args.url).json(&body).send().await?;
    print_response(resp).await?;
    Ok(())
}

/// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status =format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n",status)
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(),value)
    }
    println!()
}

/// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Response) ->Option<Mime> {
    resp.headers().get(header::CONTENT_TYPE).map(|v| v.to_str().unwrap().parse::<Mime>().unwrap())
}

/// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &str) {
    match m {
        // 对于 "application/json" 我们 pretty print
        Some(v) if v == mime::APPLICATION_JSON => print_syntect(body, "json"),
        Some(v) if v ==mime::TEXT_HTML => print_syntect(body,"html"),

        // 其它 mime type，我们就直接输出
        _=> println!("{}",body),
    }
}

/// 打印整个响应
async fn print_response(resp: Response) ->Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let content=get_content_type(&resp);
    let body =resp.text().await?;
    print_body(content, &body);
    Ok(())
}

fn print_syntect(s: &str, ext: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}


#[tokio::main]
async fn runasync() ->Result<()>{
    /*
     ide 中配置：
     run --package httpie --bin httpie -- post http://httpbin.org/post a=991 b=8
     输出：
     httpie url:Opts { subcmd: Post(Post { url: "httpbin.org/post", body: ["a=1", "b=2"] }) }
    */
    let opts: Opts= Opts::parse();
    println!("httpie url:{:?}", opts);

    let client =Client::new();

    let result = match opts.subcmd {
        SubCommand::Get(ref args)=>{
            get(client, args).await?
        },
        SubCommand::Post(ref args) =>{
            post(client,args).await?
        }
    };

    Ok(result)

}





pub fn run() {
    runasync();
}




// 仅在 cargo test 时才编译
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_url_work() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("http://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_is_ok(){
        assert!(parse_kv_pair("a").is_err());

        assert_eq!(parse_kv_pair("a=1").unwrap(),
        KvPair{k:"a".into(),v:"1".into()});

        assert_eq!(parse_kv_pair("b=").unwrap(),
                KvPair{k:"b".into(),v:"".into()});
    }

}





















