#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use diesel::r2d2;
use std::env;

pub mod schema;
pub mod model;



pub mod scoped_config;

use std::sync::Mutex;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};



pub type PoolConnection = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;

///创建连接池
pub fn new_connection_pool() -> PoolConnection {
    dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.");
    pool
}






















