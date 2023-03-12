use crate::service::estimationService;

// pub struct EstimationService {
//     estimate: web::Json<EstimateRequest>
// }

pub struct EstimationServiceImpl {}

impl estimationService for EstimationServiceImpl {
    fn estimation_request(&self, estimate: &Json) -> HttpResponse {
        /** 値オブジェクト */
        let estimationRequestId = EstimationRequestId::new(estimate.id);
        /** ドメインオブジェクト  */
        let estimationRequest = EstimationRequest::new(estimationRequestId, estimate.name, estimate.desired_amount);
        /** ドメインサービスのロジック実装 */
        // EstimationRequestService

        /** 最終的なレスポンスデータを返却 */
        HttpResponse::Ok().json()
    }
}