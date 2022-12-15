use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug)]
#[diesel(primary_key(place_id))]
pub struct Place {
    pub place_id: Uuid,
    pub name_place: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub color_id: Uuid,
    pub user_id: Uuid,
}