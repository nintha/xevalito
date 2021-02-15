use chrono::NaiveDateTime;
use rbatis::crud::{CRUDEnable};
use serde::{Serialize, Deserialize};

#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub content: String,
    pub creator_id: String,
    pub create_time: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ArticleLite {
    pub title: String,
    pub content: String,
}

