use crate::schema::{profile_users, users};

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;
use uuid::Uuid;

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, PartialEq, Eq)]
#[diesel(table_name = users)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub changed_password_at: NaiveDateTime,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize, Validate)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewUserDTO {
    #[validate(length(min = 5, max = 20), required)]
    pub username: Option<String>,
    #[validate(length(min = 8, max = 30), required)]
    pub password: Option<String>,
    #[validate(email, required)]
    pub email: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub changed_password_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserDTO {
    #[validate(length(min = 5, max = 20))]
    pub username: Option<String>,
    #[validate(length(min = 8, max = 30))]
    pub password: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = profile_users)]
#[diesel(primary_key(profile_id))]
pub struct ProfileUser {
    pub profile_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years_old: Option<NaiveDate>,
    pub preference_lang: String,
    pub gender: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Uuid,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = profile_users)]
pub struct NewProfileUser {
    pub photo_url: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub years_old: Option<NaiveDate>,
    pub preference_lang: String,
    pub gender: String,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewProfileUserDTO {
    #[validate(url)]
    pub photo_url: Option<String>,
    #[validate(length(min = 5, max = 20), required)]
    pub first_name: Option<String>,
    #[validate(length(min = 5, max = 20))]
    pub last_name: Option<String>,
    pub years_old: Option<NaiveDate>,
    #[validate(length(min = 1, max = 2), required)]
    pub preference_lang: Option<String>,
    #[validate(length(min = 1, max = 10), required)]
    pub gender: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = profile_users)]
pub struct UpdateProfileUser {
    pub photo_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub years_old: Option<NaiveDate>,
    pub preference_lang: Option<String>,
    pub gender: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateProfileUserDTO {
    #[validate(url)]
    pub photo_url: Option<String>,
    #[validate(length(min = 5, max = 20))]
    pub first_name: Option<String>,
    #[validate(length(min = 5, max = 20))]
    pub last_name: Option<String>,
    pub years_old: Option<NaiveDate>,
    #[validate(length(min = 1, max = 2))]
    pub preference_lang: Option<String>,
    #[validate(length(min = 1, max = 10))]
    pub gender: Option<String>,
}
