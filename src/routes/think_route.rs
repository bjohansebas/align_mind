use crate::services::think_service::*;
use align_mind_server::models::think_model::*;

use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/<id_think>")]
pub fn getting_think(id_think: String) -> Json<Think> {
    let uuid_think: Uuid = Uuid::parse_str(id_think.as_str()).unwrap();
    Json(get_think(uuid_think))
}

#[post("/<id_user>", data = "<payload>")]
pub fn save_think(id_user: String, payload: Json<NewThink>) -> Json<Think> {
    let uuid_user: Uuid = Uuid::parse_str(id_user.as_str()).unwrap();
    Json(create_think(uuid_user, payload.into_inner()))
}

#[post("/<id_think>/trash")]
pub fn move_to_trash(id_think: String) -> Json<TrashThink> {
    let uuid_think: Uuid = Uuid::parse_str(id_think.as_str()).unwrap();
    Json(move_think_to_trash(uuid_think))
}

#[delete("/<id_think>")]
pub fn deleting_think(id_think: String) {
    let uuid_think: Uuid = Uuid::parse_str(id_think.as_str()).unwrap();
    delete_think(uuid_think)
}

#[put("/<id_think>", data = "<payload>")]
pub fn updating_think(id_think: String, payload: Json<UpdateThink>) {
    let uuid_think: Uuid = Uuid::parse_str(id_think.as_str()).unwrap();
    update_think(uuid_think, payload.into_inner())
}
