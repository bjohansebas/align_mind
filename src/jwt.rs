use align_mind_server::establish_connection;
use align_mind_server::models::auth_model::Login;
use align_mind_server::models::response_model::{ResponseMessage, ResponseValue};
use align_mind_server::models::user_model::User;

use chrono::Utc;
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

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

use crate::services::users_service::get_user_account_by_email;

static ONE_DAY: i64 = 60 * 60 * 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub sub: Uuid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserToken {
    type Error = status::Custom<Json<ResponseMessage>>;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str: String = authen_header.to_string();

            if authen_str.starts_with("Bearer") {
                let token: &str = authen_str[6..authen_header.len()].trim();

                if let Ok(token_data) = decode_token(token.to_string()) {
                    return Outcome::Success(token_data.claims);
                }
            }
        }

        Outcome::Failure((
            Status::BadRequest,
            status::Custom(
                Status::Unauthorized,
                Json(ResponseMessage {
                    code: Some(Status::Unauthorized.code),
                    message: String::from("Invalid token, please login again"),
                }),
            ),
        ))
    }
}

fn private_secret() -> String {
    dotenv().ok();
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn generate_token(login: Login) -> String {
    let conn: &mut PgConnection = &mut establish_connection();

    let user: ResponseValue<User> = get_user_account_by_email(login.email, conn).unwrap();

    let now: i64 = Utc::now().timestamp_nanos() / 1_000_000_000;
    let payload: UserToken = UserToken {
        iat: now,
        exp: now + ONE_DAY,
        sub: user.value.user_id,
    };

    let key: String = private_secret();

    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(key.as_bytes()),
    )
    .unwrap()
}

pub fn decode_token(token: String) -> Result<TokenData<UserToken>> {
    let key: String = private_secret();

    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::default(),
    )
}
