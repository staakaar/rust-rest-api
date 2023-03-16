use anyhow::Result;

pub trait EstimationRequestRepository {

    /** 1件取得 */
    fn find_by(&self) -> Result<EstimationRequest>;

    /** 全件取得 */
    fn find_all(&self) -> Result<Vec<EstimationRequest>>;

    /** データ保存 */
    fn save(&self) -> Result<()>;

    /** データ更新 */
    fn update(&self) -> Result<()>;

    /** データ削除 */
    fn delete(&self) -> Result<()>;

}