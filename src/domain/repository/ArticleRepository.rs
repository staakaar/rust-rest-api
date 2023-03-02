trait ArticleRepository {
    fn find_all(&self) -> Vec<Article>;
}