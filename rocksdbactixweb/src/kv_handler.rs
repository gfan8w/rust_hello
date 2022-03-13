use std::ptr::null;
use actix_web::client::SendRequestError::Http;
use actix_web::HttpResponse;
use actix_web::web::{Data, Path};
use bytes::Bytes;
use crate::kv_store::{KVStore, RocksDb};


// curl -i -X GET -H "Content-Type: application/json" http://localhost:8080/api/foo
pub async fn get(key: Path<String>, db: Data<RocksDb>) -> HttpResponse {

    match db.find(&key) {
        Some(d) => {
            HttpResponse::Ok().content_type("application/json").body(d)
        },
        None => {
            HttpResponse::NotFound().content_type("application/json").finish()
        }
    }

}

// curl -i -X POST -H "Content-Type: application/json" -d '{"bar":"baz"}' http://localhost:8080/api/foo
pub async fn post(key: Path<String>, db: Data<RocksDb>, body: Bytes) -> HttpResponse {
    match String::from_utf8(body.to_vec()) {
        Ok(s) => {
            match db.save(&key,&s) {
                true => {
                    HttpResponse::Ok().content_type("application/json").finish()
                },
                false => {
                    HttpResponse::InternalServerError().content_type("application/json").finish()
                }
            }
        },
        Err(e) => {
            HttpResponse::BadRequest().content_type("application/json").finish()
        }
    }
}

// curl -i -X DELETE -H "Content-Type: application/json" http://localhost:8080/api/foo
pub async fn delete(key: Path<String>, db: Data<RocksDb>) -> HttpResponse {
    match db.delete(&key) {
        true => {
            HttpResponse::Ok().content_type("application/json").finish()
        },
        false => {
            HttpResponse::InternalServerError().content_type("application/json").finish()
        }
    }
}



























