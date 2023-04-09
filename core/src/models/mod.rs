// models/mod.rs

use std::time;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
//#[diesel(table_name = groups)]
pub struct Group {
    pub auto_id: i64,
    pub external_id: String,
    pub name: String,
    pub created_at: time::SystemTime,
    pub updated_at: time::SystemTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
//#[diesel(table_name = students)]
pub struct Student {
    pub id: i64,
    pub external_id: String,
    pub name: String,
    pub group_id: i64,
    pub created_at: time::SystemTime,
    pub updated_at: time::SystemTime,
}
