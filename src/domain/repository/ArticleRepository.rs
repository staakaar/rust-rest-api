trait ArticleRepository {
    fn find_all(&self) -> Vec<Article>;

    fn find_by_published(&self) -> Vec<Article>;
}