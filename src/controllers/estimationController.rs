use actix_web::{HttpResponse, web};
use serde::Deserialize;
use crate::application::service::estimationService::EstimationService;

#[derive(Deserialize)]
pub struct EstimateRequest {
    id: i128,
    name: String,
    desired_amount: String
}

/** estimationControllerのtrait定義 */
pub trait EstimationController {
    fn create(&self, estimate: web::Json<EstimateRequest>) -> HttpResponse;
}

/** estimationControllerの構造体の定義 */
pub struct EstimationControllerImpl<E: EstimationService> {
    estimation_service: E,
}

/** estimationControllerの実装定義 */
impl<E: EstimationService> EstimationController for EstimationControllerImpl<E> {
    fn create(&self, estimate: web::Json<EstimateRequest>) -> HttpResponse {
        let response = self.estimation_service.estimationRequest(estimate);
        /* estimationRequestの登録を行う dxo -> call service layer -> response */
        format!("estimateRquestの中身は id: {}, name: {}, amount: {}", estimate.id, estimate.name, estimate.desired_amount);
        HttpResponse::Ok().body("json返すよ!")
    } 
}
