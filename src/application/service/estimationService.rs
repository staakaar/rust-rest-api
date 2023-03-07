use actix_web::{ web, HttpResponse };

use crate::controllers::estimationController::EstimateRequest;

pub trait EstimationService {
    fn estimationRequest(&self, estimate: web::Json<EstimateRequest>) -> HttpResponse;
}

