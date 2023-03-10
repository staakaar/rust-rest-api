/** 
 * ドメインオブジェクト 
*/

struct EstimationRequest {
    id: EstimationRequestId,
    name: String,
    desired_amount: String
}

impl EstimationRequest {
    fn new(id: EstimationRequestId, name: String, desired_amount: String) -> EstimationRequest {
        EstimationRequest { id, name, desired_amount }
    }
}   