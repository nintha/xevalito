use rbatis::crud::{CRUDEnable};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub create_time: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserSave {
    pub username: String,
    pub password: String,
}