use rust_rest_api::routes;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/estimate/{code}")]
async fn estimate(path: web::Path<u32>, req_body: String) -> impl Responder {
    let _estimation_code: Option<u32> = Some(path.into_inner());
    HttpResponse::Ok().body(req_body)
}

#[post("/estimate")]
async fn estimate() -> impl Responder {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::routes)
            .service(hello)
            .service(estimate)
            .service(web::resource("/hey").route(web::get().to(manual_hello())))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}