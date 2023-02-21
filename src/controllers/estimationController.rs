use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub async fn create() -> impl Responder {
    /* estimationRequestの登録を行う dxo -> call service layer -> response */
    HttpResponse::Ok().body("json返すよ!")
}