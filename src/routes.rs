use actix_web::{get, Responder, HttpResponse};


/** ルートアクセスはデフォルトでトップページにリダイレクトされる実装にする */
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}