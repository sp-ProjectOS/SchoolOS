// models/mod.rs

use std::time;

use crate::schema::groups;
use crate::schema::students;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = groups)]
pub struct Group {
    pub auto_id: i64,
    pub external_id: String,
    pub name: String,
    pub created_at: time::SystemTime,
    pub updated_at: time::SystemTime,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = students)]
pub struct Student {
    pub auto_id: i64,
    pub external_id: String,
    pub name: String,
    pub group_id: i64,
    pub created_at: time::SystemTime,
    pub updated_at: time::SystemTime,
}
