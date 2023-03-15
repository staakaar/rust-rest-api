use diesel::{prelude::*, sql_types::{Integer, Datetime}};

/**
 * 各テーブルの構造体を定義するファイル
 * schemeファイルとフィルド順序を同じにする
 */

#[derive(Queryable)]
pub struct Article {
    pub id: Integer,
    pub title: String,
    pub description: String,
    pub genre: String,
    pub state: String,
    pub created_at: Datetime,
    pub updated_at: Datetime
}

#[derive(Queryable)]
pub struct EstimateRequest {
    pub id: Integer,
    pub name: String,
    pub desired_amount: String
}

#[derive(Queryable)]
pub struct Review {
    pub id: Integer,
    pub article_id: Integer,
    pub detail: String,
    pub created_at: Datetime,
    pub updated_at: Datetime
}