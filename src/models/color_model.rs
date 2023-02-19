use crate::models::user_model::User;
use crate::schema::colors;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;
use uuid::Uuid;

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = colors)]
#[diesel(primary_key(color_id))]
pub struct Color {
    #[serde(rename = "id")]
    pub color_id: Uuid,
    #[serde(rename = "code")]
    pub code_color: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
    #[serde(rename = "userId")]
    pub user_id: Option<Uuid>,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = colors)]
pub struct NewColor {
    pub code_color: String,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewColorDTO {
    #[serde(rename = "code")]
    #[validate(length(equal = 6), required)]
    pub code_color: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = colors)]
pub struct UpdateColor {
    pub code_color: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateColorDTO {
    #[serde(rename = "code")]
    #[validate(length(equal = 6), required)]
    pub code_color: Option<String>,
}
