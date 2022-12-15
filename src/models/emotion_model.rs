use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug)]
#[diesel(primary_key(emotion_id))]
pub struct Emotion {
    pub emotion_id: Uuid,
    pub name_emotion: String,
    pub color_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}