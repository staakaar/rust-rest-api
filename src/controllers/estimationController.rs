use actix_web::{ HttpResponse, HttpRequest, web };
use serde::Deserialize;
use crate::application::service::estimationService::{EstimationService, self};

#[derive(Deserialize)]
pub struct EstimateRequest {
    id: i128,
    name: String,
    desired_amount: String
}

/** estimationControllerの実装定義 */
pub async fn create(req: HttpRequest, estimate: web::Json<EstimateRequest>) -> HttpResponse {
    format!("estimationRequestの値は {:?}", req);
    EstimationService::estimationRequest(estimate);
    // let response = self.estimation_service.estimationRequest(estimate);
    /* estimationRequestの登録を行う dxo -> call service layer -> response */
    // format!("estimateRquestの中身は id: {}, name: {}, amount: {}", estimate.id, estimate.name, estimate.desired_amount);
    HttpResponse::Ok().body("json返すよ!")
} 
