use align_mind_server::models::response_model::{Response, ResponseError, ResponseSuccess};
use rocket::{http::Status, response::status, serde::json::Json};
use serde::Serialize;

pub fn response_api(
    entity: Result<ResponseSuccess, ResponseError>,
) -> status::Custom<Json<Response>> {
    match entity {
        Ok(response) => status::Custom(
            Status::from_code(Status::Ok.code).unwrap(),
            Json(Response {
                message: response.message,
                data: serde_json::to_value(response.data).unwrap(),
            }),
        ),
        Err(response) => status::Custom(
            Status::from_code(response.code).unwrap(),
            Json(Response {
                message: response.message,
                data: serde_json::to_value("").unwrap(),
            }),
        ),
    }
}

pub fn response_api_entity<T: Serialize>(entity: Option<T>) -> status::Custom<Json<Response>> {
    if let Some(user) = entity {
        status::Custom(
            Status::from_code(Status::Ok.code).unwrap(),
            Json(Response {
                message: String::from("Ok"),
                data: serde_json::to_value(user).unwrap(),
            }),
        )
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from(""),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}

pub fn response_api_bool(action: bool) -> status::Custom<Json<Response>> {
    if action {
        status::Custom(
            Status::from_code(Status::Ok.code).unwrap(),
            Json(Response {
                message: String::from("Ok"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    } else {
        status::Custom(
            Status::from_code(Status::BadRequest.code).unwrap(),
            Json(Response {
                message: String::from("Bad"),
                data: serde_json::to_value("").unwrap(),
            }),
        )
    }
}
