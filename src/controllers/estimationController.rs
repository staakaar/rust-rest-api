use actix_web::{ HttpResponse, HttpRequest, web };
use crate::{application::service::estimationService::EstimationService, routes::EstimateRequest};

pub struct EstimationController {}

impl EstimationController {
    pub fn create<estimation_service: EstimationService>(req: HttpRequest, estimate: web::Json<EstimateRequest>,) -> HttpResponse {
        format!("estimationRequestの値は {:?}", req);
        let response = estimation_service::estimationRequest(estimate);
        // let response = self.estimation_service.estimationRequest(estimate);
        /* estimationRequestの登録を行う dxo -> call service layer -> response */
        // format!("estimateRquestの中身は id: {}, name: {}, amount: {}", estimate.id, estimate.name, estimate.desired_amount);
        HttpResponse::Ok().body("json返すよ!")
    } 
}