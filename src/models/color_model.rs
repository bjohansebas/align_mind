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
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewColorDTO {
    #[validate(length(min = 5, max = 30), required)]
    pub name_color: Option<String>,
    #[validate(length(equal = 6), required)]
    pub code_color: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = colors)]
pub struct UpdateColor {
    pub name_color: Option<String>,
    pub code_color: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateColorDTO {
    #[validate(length(min = 5, max = 30))]
    pub name_color: Option<String>,
    #[validate(length(equal = 6))]
    pub code_color: Option<String>,
}
