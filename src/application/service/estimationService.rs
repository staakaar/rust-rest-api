use actix_web::{ web, HttpResponse };

use crate::routes::EstimateRequest;

pub trait EstimationService {
    fn estimationRequest(estimate: web::Json<EstimateRequest>) -> HttpResponse;
}

