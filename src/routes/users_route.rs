use crate::jwt::UserToken;
use crate::services::color_service::get_colors_with_user_uuid;
use crate::services::place_service::get_places_with_user_uuid;
use crate::services::think_service::get_thinks_with_user_uuid;
use crate::services::trash_service::get_trash_thinks_with_user_uuid;
use crate::services::users_service::*;
use crate::utils::responde_request::{response_api_bool, response_api_entity};

use align_mind_server::models::color_model::Color;
use align_mind_server::models::place_model::Place;
use align_mind_server::models::response_model::Response;
use align_mind_server::models::think_model::{Think, TrashThink};
use align_mind_server::models::user_model::*;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/<id_user>")]
pub fn getting_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());

    if let Ok(uuid) = uuid_user {
        let result_user: Option<User> = get_user(uuid);
        response_api_entity(result_user)
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("That is not uuid"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}

#[get("/<id_user>/profile")]
pub fn getting_profile(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());

    if let Ok(uuid) = uuid_user {
        let result_profile: Option<ProfileUser> = get_user_profile(uuid);
        response_api_entity(result_profile)
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("That is not uuid"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}

#[get("/<id_user>/places")]
pub fn getting_places_of_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());

    if let Ok(uuid) = uuid_user {
        let result_place: Option<Vec<Place>> = get_places_with_user_uuid(uuid);
        response_api_entity(result_place)
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("That is not uuid"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}

#[get("/<id_user>/colors")]
pub fn getting_colors_of_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());

    if let Ok(uuid) = uuid_user {
        let result_color: Option<Vec<Color>> = get_colors_with_user_uuid(uuid);
        response_api_entity(result_color)
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("That is not uuid"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}

#[get("/<id_user>/thinks")]
pub fn getting_thinks_of_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());

    if let Ok(uuid) = uuid_user {
        let result_think: Option<Vec<Think>> = get_thinks_with_user_uuid(uuid);
        response_api_entity(result_think)
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("That is not uuid"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}

#[get("/<id_user>/trash")]
pub fn getting_trash_of_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());

    if let Ok(uuid) = uuid_user {
        let result_trash: Option<Vec<TrashThink>> = get_trash_thinks_with_user_uuid(uuid);
        response_api_entity(result_trash)
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("That is not uuid"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}

#[post("/<id_user>/profile", data = "<payload>")]
pub fn save_profile(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
    payload: Json<NewProfileUser>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());

    if let Ok(uuid) = uuid_user {
        let action = create_profile(uuid, payload.into_inner());
        response_api_bool(action)
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("That is not uuid"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}

#[put("/<id_user>", data = "<payload>")]
pub fn updating_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
    payload: Json<UpdateUser>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());

    if let Ok(uuid) = uuid_user {
        let action = update_user(uuid, payload.into_inner());
        response_api_bool(action)
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("That is not uuid"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}

#[put("/<id_user>/profile", data = "<payload>")]
pub fn update_profile_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
    payload: Json<UpdateProfileUser>,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());

    if let Ok(uuid) = uuid_user {
        let action = update_profile(uuid, payload.into_inner());
        response_api_bool(action)
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("That is not uuid"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}

#[delete("/<id_user>")]
pub fn delete_account(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id_user: String,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let uuid_user = Uuid::parse_str(id_user.as_str());

    if let Ok(uuid) = uuid_user {
        let action = delete_user_with_profile(uuid);
        response_api_bool(action)
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("That is not uuid"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}
