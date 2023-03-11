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

    /** 別パターンのオブジェクト生成のファクトリメソッドを定義 */

    /** 希望金額を非表示にするタスク　リクエストからのフラグに応じて isPublic true 公開 false 非公開 メソッド非公開 */
}