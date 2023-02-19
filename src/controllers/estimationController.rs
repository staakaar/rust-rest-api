use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

/** URLにマッチしたら該当コントローラーの該当アクションへ遷移するように変更      */
#[get("/estimate/{code}")]
async fn estimate(path: web::Path<u32>, req_body: String) -> impl Responder {
    let _estimation_code: Option<u32> = Some(path.into_inner());
    HttpResponse::Ok().body(req_body)
}

#[post("/estimate")]
async fn estimate() -> impl Responder {}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}