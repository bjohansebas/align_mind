use crate::schema::places;
use crate::models::user_model::User;
use crate::models::color_model::Color;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;
use rocket::serde::{Deserialize, Serialize};


#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(table_name = places)]
#[diesel(primary_key(place_id))]
pub struct Place {
    pub place_id: Uuid,
    pub name_place: String,
    pub color_id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = places)]
pub struct NewPlace {
    pub name_place: String,
    pub color_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = places)]
pub struct UpdatePlace {
    pub name_place: Option<String>,
    pub color_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
}
