use crate::service::estimationService;

pub struct EstimationService {
    estimate: web::Json<EstimateRequest>
}

impl estimationService for EstimateServiceImpl {
    fn estimation_request(&self, estimate: &Json) -> HttpResponse {
        /** domain層の処理へ */
        HttpResponse::Ok().json()
    }
}