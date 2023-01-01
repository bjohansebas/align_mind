use align_mind_server::models::auth_model::LoginDTO;
use align_mind_server::models::response_model::Response;

use chrono::Utc;
use dotenvy::dotenv;
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use jsonwebtoken::{DecodingKey, EncodingKey};
use jsonwebtoken::{Header, Validation};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::env;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub sub: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserToken {
    type Error = status::Custom<Json<Response>>;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_header.len()].trim();
                if let Ok(token_data) = decode_token(token.to_string()) {
                    return Outcome::Success(token_data.claims);
                }
            }
        }
        Outcome::Failure((
            Status::BadRequest,
            status::Custom(
                Status::Unauthorized,
                Json(Response {
                    message: String::from("Invalid token, please login again"),
                    data: serde_json::to_value("").unwrap(),
                }),
            ),
        ))
    }
}

fn private_secret() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn generate_token(login: LoginDTO) -> String {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000;
    let payload = UserToken {
        iat: now,
        exp: now + ONE_WEEK,
        sub: login.email,
    };

    let key = private_secret();
    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(key.as_bytes()),
    )
    .unwrap()
}

pub fn decode_token(token: String) -> Result<TokenData<UserToken>> {
    let key = private_secret();
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::default(),
    )
}
