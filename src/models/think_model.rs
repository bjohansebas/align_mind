use crate::models::place_model::Place;
use crate::models::user_model::User;
use crate::schema::{thinks, trash_thinks};

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;
use uuid::Uuid;

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Place, foreign_key = place_id))]
#[diesel(table_name = thinks)]
#[diesel(primary_key(think_id))]
pub struct Think {
    pub think_id: Uuid,
    pub text_think: String,
    pub user_id: Uuid,
    pub place_id: Uuid,
    pub is_archive: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = thinks)]
pub struct NewThink {
    pub think_id: Uuid,
    pub text_think: String,
    pub user_id: Uuid,
    pub place_id: Uuid,
    pub is_archive: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewThinkDTO {
    #[validate(length(min = 5), required)]
    pub text_think: Option<String>,
    #[validate(length(equal = 36), required)]
    pub place_id: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = thinks)]
pub struct UpdateThink {
    pub text_think: Option<String>,
    pub is_archive: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateThinkDTO {
    #[validate(length(min = 5))]
    pub text_think: Option<String>,
    pub is_archive: Option<bool>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Place, foreign_key = place_id))]
#[diesel(table_name = trash_thinks)]
#[diesel(primary_key(trash_th_id))]
pub struct TrashThink {
    pub trash_th_id: Uuid,
    pub text_think: String,
    pub user_id: Uuid,
    pub place_id: Uuid,
    pub date_start: NaiveDate,
    pub date_end: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = trash_thinks)]
pub struct NewTrashThink {
    pub trash_th_id: Uuid,
    pub text_think: String,
    pub user_id: Uuid,
    pub place_id: Uuid,
    pub date_start: Option<NaiveDate>,
    pub date_end: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
