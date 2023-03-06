use crate::service::estimationService;

pub struct EstimationService {
    estimate: web::Json<EstimateRequest>
}

impl estimationService for EstimateServiceImpl {
    fn estimation_request(&self, estimate: &Json) -> HttpResponse {
        /** 値オブジェクト */
        let estimationRequestId = EstimationRequestId::new(estimate.id);
        /** ドメインオブジェクト  */
        let estimationRequest = EstimationRequest::new(estimationRequestId, estimate.name, estimate.desired_amount);
        /** ドメインサービスのロジック実装 */


        /** 最終的なレスポンスデータを返却 */
        HttpResponse::Ok().json()
    }
}