use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug)]
#[diesel(primary_key(think_emotion_id))]
pub struct ThinkEmotion {
    pub think_emotion_id: Uuid,
    pub think_id: Uuid,
    pub emotion_id: Uuid,
}

#[derive(Queryable, Debug)]
#[diesel(primary_key(think_id))]
pub struct Think {
    pub think_id: Uuid,
    pub user_id: Uuid,
    pub place_id: Uuid,
    pub is_archive: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
#[diesel(primary_key(trash_th_id))]
pub struct TrashThink {
    pub trash_th_id: Uuid,
    pub think_id: Uuid,
    pub date_start: NaiveDate,
    pub date_end: NaiveDate,
}