#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Article {
    pub id: Integer,
    pub title: String,
    pub description: String,
    pub genre: String,
    pub state: String,
    pub created_at: Datetime,
    pub updated_at: Datetime
}

/** 型をそれぞれ値オブジェクトを用意して置き換える */
/** コンストラクタ実装 */
impl Article {
    pub fn new(id: Integer, title: String, description: String, genre: String, state: String) -> Self {
        Self {
            id,
            title,
            description,
            genre,
            state,
        }
    }
}