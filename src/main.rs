use rust_rest_api::routes;

use actix_web::{ App, HttpServer };


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::estimation_request)
            // .service(web::resource("/hey").route(web::get().to(manual_hello())))
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}