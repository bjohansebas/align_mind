use crate::services::think_service::*;
use align_mind_server::models::think_model::*;

use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/<id_trash>")]
pub fn getting_trash(id_trash: String) -> Json<TrashThink> {
    let uuid_trash: Uuid = Uuid::parse_str(id_trash.as_str()).unwrap();
    Json(get_trash_think(uuid_trash))
}

#[post("/<id_trash>")]
pub fn restore_think(id_trash: String) -> Json<Think> {
    let uuid_trash: Uuid = Uuid::parse_str(id_trash.as_str()).unwrap();
    Json(remove_of_trash(uuid_trash))
}
