use crate::schema::{think_emotions, think_trash_emotions};
use crate::models::think_model::{TrashThink, Think};
use crate::models::emotion_model::Emotion;

use diesel::prelude::*;
use uuid::Uuid;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(Think, foreign_key = think_id))]
#[diesel(belongs_to(Emotion, foreign_key = emotion_id))]
#[diesel(table_name = think_emotions)]
#[diesel(primary_key(think_emotion_id))]
pub struct ThinkEmotion {
    pub think_emotion_id: Uuid,
    pub think_id: Uuid,
    pub emotion_id: Uuid,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = think_emotions)]
pub struct NewThinkEmotion {
    pub think_id: Uuid,
    pub emotion_id: Uuid,
}

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(TrashThink, foreign_key = trash_th_id))]
#[diesel(belongs_to(Emotion, foreign_key = emotion_id))]
#[diesel(table_name = think_trash_emotions)]
#[diesel(primary_key(think_trash_emotion_id))]
pub struct ThinkTrashEmotion {
    pub think_trash_emotion_id: Uuid,
    pub trash_th_id: Uuid,
    pub emotion_id: Uuid,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = think_trash_emotions)]
pub struct NewThinkTrashEmotion {
    pub trash_th_id: Uuid,
    pub emotion_id: Uuid,
}