use align_mind_server::models::auth_model::{Login, LoginDTO, LoginInfo};
use align_mind_server::models::response_model::{ResponseError, ResponseSuccess};
use align_mind_server::models::user_model::{NewUser, NewUserDTO, User};
use align_mind_server::schema::users;

use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use rocket::http::Status;

use crate::jwt::generate_token;

use super::users_service::{get_user_by_email, get_user_by_username};

pub fn create_account(
    data_user: NewUserDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    if get_user_by_email(data_user.email.to_owned().unwrap(), conn).is_ok() {
        return Err(ResponseError {
            code: Status::Conflict.code,
            message: "Email taken for other user".to_string(),
        });
    }

    if get_user_by_username(data_user.username.to_owned().unwrap(), conn).is_ok() {
        return Err(ResponseError {
            code: Status::Conflict.code,
            message: "Username taken for othe user".to_string(),
        });
    }

    let hash_password: String = hash(data_user.password.to_owned().unwrap(), DEFAULT_COST).unwrap();

    let user: NewUser = NewUser {
        username: data_user.username.to_owned().unwrap(),
        password: hash_password,
        email: data_user.email.unwrap(),
    };

    let result_action: bool = diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)
        .is_ok();

    if result_action {
        Ok(ResponseSuccess {
            message: "signup successfully".to_string(),
            data: serde_json::to_value("").unwrap(),
        })
    } else {
        Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Error not found".to_string(),
        })
    }
}

pub fn sign_in(
    payload: LoginDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let login: Login = Login {
        email: payload.email.unwrap(),
        password: payload.password.unwrap(),
    };

    let user_to_login: User = get_user_by_email(login.email.to_owned(), conn)?;

    if !verify_passwords(&login.password, &user_to_login.password) {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Wrong email or password, please try again".to_string(),
        });
    }

    Ok(ResponseSuccess {
        data: serde_json::to_value(LoginInfo {
            id: user_to_login.user_id.to_string(),
            email: login.email.to_owned(),
            login_session: generate_token(login),
        })
        .unwrap(),
        message: "login successfully".to_string(),
    })
}

fn verify_passwords(login_password: &String, hash_password: &str) -> bool {
    verify(login_password, hash_password).unwrap()
}
