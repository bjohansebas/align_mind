use crate::schema::colors;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;
use rocket::serde::{Deserialize, Serialize};


#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, PartialEq, Eq)]
#[diesel(table_name = colors)]
#[diesel(primary_key(color_id))]
pub struct Color {
    pub color_id: Uuid,
    pub name_color: String,
    pub code_color: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = colors)]
pub struct NewColor {
    pub name_color: String,
    pub code_color: String
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = colors)]
pub struct UptadeColor {
    pub name_color: Option<String>,
    pub code_color: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}