
#[derive(Debug, Serialize)]
pub struct ArticleDto {
    /** フィールド定義 */
    pub id: Integer,
    pub title: String,
    pub description: String,
    pub genre: String,
    pub state: String,
    pub created_at: Datetime,
    pub updated_at: Datetime
}