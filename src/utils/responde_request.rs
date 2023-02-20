use align_mind_server::models::response_model::{ResponseMessage, ResponseValue};
use rocket::{http::Status, response::status, serde::json::Json};
use serde::Serialize;
use serde_json::Value;

pub fn response_message_api(entity: ResponseMessage) -> status::Custom<Json<ResponseMessage>> {
    match entity.code {
        Some(code) => status::Custom(
            Status::from_code(code).unwrap(),
            Json(ResponseMessage {
                message: entity.message,
                code: Some(code),
            }),
        ),
        None => status::Custom(
            Status::from_code(Status::Accepted.code).unwrap(),
            Json(ResponseMessage {
                message: entity.message,
                code: None,
            }),
        ),
    }
}

pub fn response_value_api<T: Serialize>(
    entity: Result<ResponseValue<T>, ResponseMessage>,
) -> status::Custom<Json<Value>> {
    match entity {
        Ok(response) => status::Custom(
            Status::from_code(response.code).unwrap(),
            Json(serde_json::to_value(response.value).unwrap()),
        ),
        Err(response) => status::Custom(
            Status::from_code(response.code.unwrap()).unwrap(),
            Json(
                serde_json::to_value(ResponseMessage {
                    code: response.code,
                    message: response.message,
                })
                .unwrap(),
            ),
        ),
    }
}
