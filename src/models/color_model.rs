use crate::models::user_model::User;
use crate::schema::colors;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = colors)]
#[diesel(primary_key(color_id))]
pub struct Color {
    pub color_id: Uuid,
    pub name_color: String,
    pub code_color: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Option<Uuid>,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = colors)]
pub struct NewColor {
    pub name_color: String,
    pub code_color: String,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = colors)]
pub struct UpdateColor {
    pub name_color: Option<String>,
    pub code_color: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}
