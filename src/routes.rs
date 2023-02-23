use actix_web::{ web };
use crate::controllers::estimationController;


pub fn estimation_request(config: &mut web::ServiceConfig) {
    config.service(web::resource("/estimate/{code}")
            .route(web::get().to(estimationController::index)))
            .service(web::resource("/estimate").route(web::post().to(estimationController::create)));
}

/* ルートアクセスはデフォルトでトップページにリダイレクトされる実装にする */
// #[get("/")]
// async fn root() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[get("/top")]
// async fn top() -> impl Responder {
//     HttpResponse::Ok().body("トップページだよ!")
// }

/* URLにマッチしたら該当コントローラーの該当アクションへ遷移するように変更 */
// #[get("/estimate/{code}")]
// async fn estimate(path: web::Path<u32>, req_body: String) -> impl Responder {
//     let _estimation_code: Option<u32> = Some(path.into_inner());
//     HttpResponse::Ok().body(req_body)
// }

// #[post("/estimate")]
// async fn estimate() -> impl Responder {}