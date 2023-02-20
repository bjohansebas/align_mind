use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    pub message: String,
}

#[derive(Debug)]
pub struct ResponseValue<T: Serialize> {
    pub code: u16,
    pub value: T,
}
