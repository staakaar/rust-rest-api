## テーブル設計(仮)
## Articles
|カラム|PRIMARY|FOREIGN|INDEX|NOT NULL|
|-----------|-----------|-----------|-----------|-----------|
|id|||
|title|||
|description|||
|genre|||
|state|||
|created_at|||
|updated_at|||

## Reviews
|カラム|PRIMARY|FOREIGN|INDEX|NOT NULL|
|-----------|-----------|-----------|-----------|-----------|
|id|||
|article_id|||
|message|||
|state|||
|created_at|||
|updated_at|||

## ArticleTags
|カラム|PRIMARY|FOREIGN|INDEX|NOT NULL|
|-----------|-----------|-----------|-----------|-----------|
|id|||
|article_id|||
|name|||
|created_at|||
|updated_at|||

## ArticleMemos
|カラム|PRIMARY|FOREIGN|INDEX|NOT NULL|
|-----------|-----------|-----------|-----------|-----------|
|id|||
|article_id|||
|comment|||
|created_at|||
|updated_at|||

## ArticleImages
|カラム|PRIMARY|FOREIGN|INDEX|NOT NULL|
|-----------|-----------|-----------|-----------|-----------|
|id|||
|article_id|||
|image|||
|name|||
|path|||
|created_at|||
|updated_at|||

## ArticleFiles
|カラム|PRIMARY|FOREIGN|INDEX|NOT NULL|
|-----------|-----------|-----------|-----------|-----------|
|id|||
|article_id|||
|name|||
|size|||
|path|||
|created_at|||
|updated_at|||

## Users
|カラム|PRIMARY|FOREIGN|INDEX|NOT NULL|
|-----------|-----------|-----------|-----------|-----------|
|id|||
|first_name|||
|first_name_kana|||
|last_name|||
|last_name_kana|||
|gender|||
|age|||
|birthday|||
|address|||
|post_code|||
|phone_number|||
|created_at|||
|updated_at|||

## Company
|カラム|PRIMARY|FOREIGN|INDEX|NOT NULL|
|-----------|-----------|-----------|-----------|-----------|
|id|||
|company_name|||
|company_name_kana|||
|address|||
|post_code|||
|phone_number|||
|contact|||
|created_at|||
|updated_at|||

## Department
|カラム|PRIMARY|FOREIGN|INDEX|NOT NULL|
|-----------|-----------|-----------|-----------|-----------|
|id|||
|company_id|||
|department_name|||
|department_name_kana|||
|created_at|||
|updated_at|||

## Director
|カラム|PRIMARY|FOREIGN|INDEX|NOT NULL|
|-----------|-----------|-----------|-----------|-----------|
|id|||
|department_id|||
|director_name|||
|director_name_kana|||
|created_at|||
|updated_at|||

## Schedule
|カラム|PRIMARY|FOREIGN|INDEX|NOT NULL|
|-----------|-----------|-----------|-----------|-----------|

## Event
|カラム|PRIMARY|FOREIGN|INDEX|NOT NULL|
|-----------|-----------|-----------|-----------|-----------|


## ER図