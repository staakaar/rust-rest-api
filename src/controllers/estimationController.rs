use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct EstimateRequest {
    estimation_request_id: i128,
    name: String,
    desired_amount: String
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("body")
}

pub async fn create(estimate: web::Json<EstimateRequest>) -> impl Responder {
    /* estimationRequestの登録を行う dxo -> call service layer -> response */
    format!("estimateRquestの中身は id: {}, name: {}, amount: {}", estimate.estimation_request_id, estimate.name, estimate.desired_amount);
    HttpResponse::Ok().body("json返すよ!")
}