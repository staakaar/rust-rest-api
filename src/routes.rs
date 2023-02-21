use actix_web::{get, Responder, HttpResponse, web, App, post};
use serde::Deserialize;
use crate::controllers::estimationController;

/* request　form 作成してベットファイル用意 */
#[derive(Deserialize)]
struct EstimateRequest {
    estimation_request_id: i128,
    name: String,
    desired_amount: String
}

pub fn estimation_request_config(config: &mut web::ServiceConfig) {
    config.service(web::resource("/estimate/{code}")
            .route(web::get().to(estimationController::create)));
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