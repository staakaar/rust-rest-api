use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

struct ArticleRepositoryImpl {
    connection: SqliteConnection,
}

impl ArticleRepository for ArticleRepositoryImpl {
    /** 記事全件取得 */
    fn find_all(&self) -> Vec<Article> {
        use diesel_demo::schema::articles::dsl::*;

        let results = articles.load::<Article>(&self.connection).unwrap();

        println!("Results {} articles", results.len());
    }

    fn find_by_published(&self) -> Vec<Article> {
        use diesel_demo::schema::articles::dsl::*;

        let results = articles.filter(published.eq(true))
            .limit(5)
            .load::<Article>(&connection)
            .expect("記事の読み込みに失敗しました");
    }
}