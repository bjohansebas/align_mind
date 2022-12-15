use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::{profile_users,users};
use rocket::serde::{Deserialize,Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(ProfileUser,foreign_key = profile_id))]
#[diesel(table_name = users)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub changed_password_at: Option<NaiveDateTime>,
    pub email: String,
    pub profile_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable,Debug,Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub profile_id: Option<Uuid>,
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

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, PartialEq, Eq)]
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
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable,Debug,Deserialize)]
#[diesel(table_name = profile_users)]
pub struct NewProfileUser<'a> {
    pub photo_url: Option<&'a str>,
    pub first_name: &'a str,
    pub last_name: Option<&'a str>,
    pub years_old: Option<NaiveDate>,
    pub preference_lang: &'a str,
    pub gender: &'a str,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = profile_users)]
pub struct UpdateProfileUser<'a> {
    pub photo_url: Option<&'a str>,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub years_old: Option<NaiveDate>,
    pub preference_lang: Option<&'a str>,
    pub gender: Option<&'a str>,
    pub updated_at: Option<NaiveDateTime>,
}