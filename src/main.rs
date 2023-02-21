use rust_rest_api::routes;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::estimation_request_config)
            // .service(web::resource("/hey").route(web::get().to(manual_hello())))
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}