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
    id: Integer,
    name: String,
    desired_amount: String
}