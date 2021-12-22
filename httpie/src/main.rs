pub mod http_main;


pub fn main(){
    // run --package httpie --bin httpie -- post http://httpbin.org/post a=991 b=8
    http_main::run();
}
