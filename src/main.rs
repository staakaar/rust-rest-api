use rust_rest_api::routes;

use actix_web::{ App, HttpServer };

mod lib;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = db::establish_connection();

    HttpServer::new(|| {
        App::new()
            /** ペイロードとして送れる最大容量 */
            .app_data(web::Form::default().limit(4096))
            .app_data(Data::new(pool.clone()))
            .configure(routes::estimation_request)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}