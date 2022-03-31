use actix_web::{get,Responder, Scope, web,Result as ActixResult, Error as ActixError};
use actix_web::web::route;
use crate::AppState;

pub fn todos_handler() -> Scope {
    web::scope("/api").service(get_todos).service(get_cakes)
}

///access http://localhost:8000/todos/
#[get("/todo")]
async fn get_todos(app_state: web::Data<AppState>) -> ActixResult<impl Responder, ActixError> {

    let todos = app_state.repository.get_todos().await;
    Ok(web::Json(todos))

}

#[get("/cake/{id}")]
async fn get_cakes(app_state: web::Data<AppState>, id: web::Path<(i32)>) -> ActixResult<impl Responder, ActixError> {

    let cakes = app_state.repository.get_cake(id.into_inner()).await;



    Ok(web::Json(cakes))

}



