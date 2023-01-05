use align_mind_server::establish_connection;
use align_mind_server::models::auth_model::{Login, LoginDTO, LoginInfo};
use align_mind_server::models::response_model::{ResponseError, ResponseSuccess};
use align_mind_server::models::user_model::{NewUser, NewUserDTO, User};
use align_mind_server::schema::users;

use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use rocket::http::Status;

use crate::jwt::generate_token;

use super::users_service::{
    exist_email, exist_email_with_db, exist_username, exist_username_with_db, get_user_by_email,
};

pub fn create_account(
    data_user: NewUserDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    if exist_email_with_db(data_user.email.to_owned().unwrap(), conn) {
        return Err(ResponseError {
            code: Status::Conflict.code,
            message: "Email taken for other user".to_string(),
        });
    }

    if exist_username_with_db(data_user.username.to_owned().unwrap(), conn) {
        return Err(ResponseError {
            code: Status::Conflict.code,
            message: "Username taken for othe user".to_string(),
        });
    }

    let hash_password: String =
        hash(&data_user.password.to_owned().unwrap(), DEFAULT_COST).unwrap();

    let user: NewUser = NewUser {
        username: data_user.username.to_owned().unwrap(),
        password: hash_password,
        email: data_user.email.to_owned().unwrap(),
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

    let user_to_login: Result<User, ResponseError> = get_user_by_email(&login.email, conn);

    if let Err(e) = user_to_login {
        return Err(e);
    }

    if let Ok(user) = user_to_login {
        if !verify_passwords(&login.password, &user.password) {
            return Err(ResponseError {
                code: Status::BadRequest.code,
                message: "Wrong email or password, please try again".to_string(),
            });
        }
    }

    Ok(ResponseSuccess {
        data: serde_json::to_value(LoginInfo {
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
