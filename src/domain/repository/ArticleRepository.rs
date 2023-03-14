pub trait ArticleRepository {

    fn find_by_article(&self, number: &ArtcileId) -> Result<Article>;

    fn find_all(&self) -> Vec<Article>;

    fn find_by_published(&self) -> Vec<Article>;
}