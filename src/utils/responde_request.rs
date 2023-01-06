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

pub fn response_api_data<T: Serialize>(
    entity: Result<T, ResponseError>,
) -> status::Custom<Json<Response>> {
    match entity {
        Ok(response) => status::Custom(
            Status::from_code(Status::Ok.code).unwrap(),
            Json(Response {
                message: "Ok".to_string(),
                data: serde_json::to_value(response).unwrap(),
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
