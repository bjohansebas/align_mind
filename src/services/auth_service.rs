use align_mind_server::models::auth_model::{Login, LoginDTO, LoginInfo};
use align_mind_server::models::response_model::{ResponseMessage, ResponseValue};
use align_mind_server::models::user_model::{NewUser, NewUserDTO, User};
use align_mind_server::schema::users;

use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use rocket::http::Status;

use crate::jwt::generate_token;

use super::users_service::get_user_account_by_email;

pub fn create_account(data_user: NewUserDTO, conn: &mut PgConnection) -> ResponseMessage {
    if get_user_account_by_email(data_user.email.to_owned().unwrap(), conn).is_ok() {
        return ResponseMessage {
            code: Some(Status::Conflict.code),
            message: "Email taken for other user".to_string(),
        };
    }

    let hash_password: String = hash(data_user.password.to_owned().unwrap(), DEFAULT_COST).unwrap();

    let user: NewUser = NewUser {
        password: hash_password,
        email: data_user.email.unwrap(),
    };

    let result_action: bool = diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)
        .is_ok();

    if !result_action {
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Error not found".to_string(),
        };
    }

    ResponseMessage {
        code: None,
        message: "signup successfully".to_string(),
    }
}

pub fn sign_in(
    payload: LoginDTO,
    conn: &mut PgConnection,
) -> Result<ResponseValue<LoginInfo>, ResponseMessage> {
    let login: Login = Login {
        email: payload.email.unwrap(),
        password: payload.password.unwrap(),
    };

    let user_to_login: Result<ResponseValue<User>, ResponseMessage> =
        get_user_account_by_email(login.email.to_owned(), conn);

    if user_to_login.is_err() {
        return Err(ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Wrong email or password, please try again".to_string(),
        });
    }

    let user: User = user_to_login.unwrap().value;

    if !verify_passwords(&login.password, &user.password) {
        return Err(ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Wrong email or password, please try again".to_string(),
        });
    }

    Ok(ResponseValue {
        code: Status::Accepted.code,
        value: LoginInfo {
            id: user.user_id.to_owned().to_string(),
            email: login.email.to_owned(),
            login_session: generate_token(login),
        },
    })
}

fn verify_passwords(login_password: &String, hash_password: &str) -> bool {
    verify(login_password, hash_password).unwrap()
}
