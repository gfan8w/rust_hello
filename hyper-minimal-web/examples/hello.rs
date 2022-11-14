use std::convert::Infallible;
use std::fmt::Debug;
use std::io;
use std::net::SocketAddr;
use hyper::{Body, Request, StatusCode};
use serde::Deserialize;
use hyper_minimal_web::{Server, Context, Response, Error, Handler};
use multipart::server::hyper::{Switch, MultipartHandler, HyperRequest};
use multipart::server::{Multipart, Entries, SaveResult};
use multipart::mock::StdoutTee;

#[derive(Clone, Debug)]
pub struct Extra {
    pub state: i32,
    pub success: bool,
    pub message: String,
}

impl Default for Extra {
    fn default() -> Self {
        Self{
            state: 0,
            success: false,
            message: "".to_string()
        }
    }
}


#[tokio::main]
async fn main() {
    let some_state = "state".to_string();



    let mut srv = Server::<Extra>::new();

    srv.get("/test", test_handler);
    srv.post("/send", send_handler);
    srv.get("/params/:some_param", param_handler);
    srv.get("/name/:name/age/:age", param_handler_multiy);

    srv.get("/", |_req| async move {
        hyper::Response::builder()
            .status(hyper::StatusCode::OK)
            .body(hyper::Body::from("Welcome!"))
            .unwrap()
    });

    srv.run("0.0.0.0:8080".parse::<SocketAddr>().unwrap()).await.expect("server error");
}


pub async fn test_handler<T: Debug>(ctx: Context<T>) -> String {
    format!("test called, state_thing was: {:?}", ctx.state.data)
}

#[derive(Deserialize)]
struct SendRequest {
    name: String,
    active: bool,
}

pub async fn send_handler<T>(mut ctx: Context<T>) -> Response {
    let body: SendRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap()
        }
    };

    Response::new(
        format!(
            "send called with name: {} and active: {}",
            body.name, body.active
        )
            .into(),
    )
}

pub async fn param_handler<T>(ctx: Context<T>) -> String {
    let param = match ctx.params.find("some_param") {
        Some(v) => v,
        None => "empty",
    };
    format!("param called, param was: {}", param)
}

pub async fn param_handler_multiy<T>(ctx: Context<T>) -> String {
    let param = ctx.params;
    let name = match param.find("name") {
        Some(v) => v,
        None => "empty",
    };
    let age = match param.find("age") {
        Some(v) => v.parse::<u32>().unwrap_or(0),
        None => 0,
    };

    format!("name&age called, name was: {}, age was{}", name, age)
}


// async fn handle_file(mut files: multipart::server::Multipart<HyperRequest>) -> String {
//     files.foreach_entry(|field| {
//         // contains name, filename, type ..
//         println!("Info: {:?}",field.headers);
//         // contains data
//         let mut bytes:Vec<u8> = Vec::new();
//         field.data.read_to_end(&mut bytes);
//     });
//     format!("Received the files!")
// }
//
// async fn handle_file_upload(req: hyper::Request<hyper::Body>) -> String {
//     let mut files = multipart::server::Multipart::from_request(req);
//     format!("Received the files!")
// }

// struct NonMultipart;
// impl hyper::server::Handler for NonMultipart {
//     fn handle(&self, _: Request, mut res: Response) {
//         *res.status_mut() = StatusCode::ImATeapot;
//         res.send(b"Please send a multipart req :(\n").unwrap();
//     }
// }
//
// struct EchoMultipart;
// impl MultipartHandler for EchoMultipart {
//     fn handle_multipart(&self, mut multipart: Multipart<HyperRequest>, res: hyper::Response) {
//         match multipart.save().temp() {
//             SaveResult::Full(entries) => process_entries(res, entries).unwrap(),
//             SaveResult::Partial(entries, error) => {
//                 println!("Errors saving multipart:\n{:?}", error);
//                 process_entries(res, entries.into()).unwrap();
//             }
//             SaveResult::Error(error) => {
//                 println!("Errors saving multipart:\n{:?}", error);
//                 res.send(format!("An error occurred {}", error).as_bytes()).unwrap();
//             }
//         };
//     }
// }
//
// fn process_entries(res: hyper::Response, entries: Entries) -> io::Result<()> {
//     let mut res = res.start()?;
//     let stdout = io::stdout();
//     let out = StdoutTee::new(&mut res, &stdout);
//     entries.write_debug(out)
// }

