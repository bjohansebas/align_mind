use crate::schema::{profile_users, users};

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, PartialEq, Eq)]
#[diesel(table_name = users)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub changed_password_at: NaiveDateTime,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub changed_password_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = profile_users)]
#[diesel(primary_key(profile_id))]
pub struct ProfileUser {
    pub profile_id: Uuid,
    pub photo_url: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
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
    pub user_id: Option<Uuid>,
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
