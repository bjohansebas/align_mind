use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug)]
#[diesel(primary_key(color_id))]
pub struct Color {
    pub color_id: Uuid,
    pub name_color: String,
    pub code_color: String,
    pub created_at: Option<NaiveDateTime>,
}