/**
 * DTO(Data Transfer Object)
 * DXOでの呼び出しに利用
 * 参照は別で管理
 */

#[derive(Debug, Serialize)]
pub struct EstimationRequestDto {
    pub id: i128,
    pub name: String,
    pub desired_amount: String
}