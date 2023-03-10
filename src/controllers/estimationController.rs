use actix_web::{ HttpResponse, HttpRequest, web };
use serde::{Deserialize, Serialize};
use crate::application::service::estimationService::EstimationService;

#[derive(Serialize, Deserialize)]
pub struct EstimateRequest {
    id: i128,
    name: String,
    desired_amount: String
}

pub struct EstimationController {
    service: EstimationService,
}

impl EstimationController {
    /** constractor DI */
    pub fn new(service: EstimationService) -> EstimationController {
        EstimationController { service }
    }
    /** estimationControllerの実装定義 */
    pub async fn create(req: HttpRequest, estimate: web::Json<EstimateRequest>) -> HttpResponse {
        format!("estimationRequestの値は {:?}", req);
        let response = EstimationService::estimationRequest(estimate);
        // let response = self.estimation_service.estimationRequest(estimate);
        /* estimationRequestの登録を行う dxo -> call service layer -> response */
        // format!("estimateRquestの中身は id: {}, name: {}, amount: {}", estimate.id, estimate.name, estimate.desired_amount);
        HttpResponse::Ok().body("json返すよ!")
    }
} 
