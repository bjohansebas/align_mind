use align_mind_server::establish_connection;
use align_mind_server::models::auth_model::{Login, LoginInfo};
use align_mind_server::models::user_model::{NewUser, User};
use align_mind_server::schema::users;

use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;

use crate::jwt::generate_token;

use super::users_service::{exist_email, get_user_by_email};

pub fn create_account(mut data_user: NewUser) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let hash_password: String = hash(&data_user.password, DEFAULT_COST).unwrap();
    data_user.password = hash_password;

    if exist_email(data_user.email.to_owned()) {
        return false;
    }

    diesel::insert_into(users::table)
        .values(&data_user)
        .execute(connection)
        .is_ok()
}

pub fn sign_in(login: Login) -> Option<LoginInfo> {
    let user_to_login: Option<User> = get_user_by_email(&login.email);
    if let Some(user) = user_to_login {
        if !login.password.is_empty() && verify_passwords(&login.password, &user.password) {
            Some(LoginInfo {
                email: user.email,
                login_session: generate_token(login),
            })
        } else {
            None
        }
    } else {
        None
    }
}

fn verify_passwords(login_password: &String, hash_password: &str) -> bool {
    verify(login_password, hash_password).unwrap()
}
