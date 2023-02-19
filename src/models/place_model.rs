use crate::models::color_model::Color;
use crate::models::user_model::User;
use crate::schema::places;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;
use uuid::Uuid;

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(table_name = places)]
#[diesel(primary_key(place_id))]
pub struct Place {
    #[serde(rename = "id")]
    pub place_id: Uuid,
    #[serde(rename = "name")]
    pub name_place: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
    #[serde(rename = "color")]
    pub color_id: Uuid,
    #[serde(rename = "user")]
    pub user_id: Uuid,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = places)]
pub struct NewPlace {
    pub name_place: String,
    pub color_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewPlaceDTO {
    #[validate(length(min = 2, max = 30), required)]
    #[serde(rename = "name")]
    pub name_place: Option<String>,
    #[validate(length(equal = 6), required)]
    #[serde(rename = "codeColor")]
    pub code_color: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = places)]
pub struct UpdatePlace {
    pub name_place: Option<String>,
    pub color_id: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePlaceDTO {
    #[validate(length(min = 2, max = 30))]
    #[serde(rename = "name")]
    pub name_place: Option<String>,
    #[validate(length(equal = 6))]
    #[serde(rename = "codeColor")]
    pub code_color: Option<String>,
}
