use crate::models::color_model::Color;
use crate::schema::emotions;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(table_name = emotions)]
#[diesel(primary_key(emotion_id))]
pub struct Emotion {
    pub emotion_id: Uuid,
    pub name_emotion: String,
    pub color_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = emotions)]
pub struct NewEmotion {
    pub name_emotion: String,
    pub color_id: Uuid,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = emotions)]
pub struct UpdateEmotion {
    pub name_emotion: Option<String>,
    pub color_id: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
}
