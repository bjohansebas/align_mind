use rocket::serde::json::Json;
use uuid::Uuid;

use align_mind_server::models::user_model::*;

use crate::services::users_service::*;

#[get("/<id_user>")]
pub fn getting_user(id_user: String) -> Json<User> {
     Json(get_user(id_user))
}

#[get("/<id_user>/profile")]
pub fn getting_profile(id_user: String) -> Json<ProfileUser> {
     Json(get_user_profile(id_user))
}

#[post("/", data = "<payload>")]
pub fn save_profile_user(payload: Json<(NewProfileUser,NewUser)>) -> Json<(ProfileUser,User)> {
     let (data_profile,data_user)=payload.into_inner();
     Json(create_user_with_profile(data_profile, data_user))
}

#[delete("/<id_user>")]
pub fn delete_profile_with_user(id_user: String) {
     let uuid_user = Uuid::parse_str(id_user.as_str()).unwrap();
     delete_user_with_profile(uuid_user)
}

#[put("/<id_user>", data = "<payload>")]
pub fn updating_user(id_user: String, payload: Json<UpdateUser>) {
     let uuid_user = Uuid::parse_str(id_user.as_str()).unwrap();
     update_user(uuid_user, payload.into_inner())
}

#[put("/<id_user>/profile",data = "<payload>")]
pub fn update_profile_user(id_user: String, payload: Json<UpdateProfileUser>) {
     let uuid_user = Uuid::parse_str(id_user.as_str()).unwrap();
     update_profile(uuid_user, payload.into_inner())
}