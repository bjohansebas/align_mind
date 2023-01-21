use align_mind_server::establish_connection;
use align_mind_server::models::response_model::{ResponseError, ResponseSuccess};
use align_mind_server::models::user_model::*;
use align_mind_server::schema::{profile_users, users};

use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use rocket::http::Status;
use uuid::Uuid;

pub fn get_user(uuid_user: Uuid, conn: &mut PgConnection) -> Result<User, ResponseError> {
    users::table
        .filter(users::user_id.eq(uuid_user))
        .first::<User>(conn)
        .map_err(|_| ResponseError {
            code: Status::NotFound.code,
            message: "The id wasn't found".to_string(),
        })
}

pub fn get_user_by_email(
    email_user: String,
    conn: &mut PgConnection,
) -> Result<User, ResponseError> {
    users::table
        .filter(users::email.eq(email_user))
        .first::<User>(conn)
        .map_err(|_| ResponseError {
            code: Status::NotFound.code,
            message: "The email was not register".to_string(),
        })
}

pub fn get_user_by_username(
    username: String,
    conn: &mut PgConnection,
) -> Result<User, ResponseError> {
    users::table
        .filter(users::username.eq(username))
        .first::<User>(conn)
        .map_err(|_| ResponseError {
            code: Status::NotFound.code,
            message: "The username wasn't found".to_string(),
        })
}

pub fn get_user_profile(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<ProfileUser, ResponseError> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: User = get_user(uuid_user, conn)?;

    let result_profile: Result<ProfileUser, diesel::result::Error> =
        ProfileUser::belonging_to(&result_user).first::<ProfileUser>(connection);

    if result_profile.is_err() {
        return Err(ResponseError {
            code: Status::NotFound.code,
            message: "The profile hasn't been created".to_string(),
        });
    }

    Ok(result_profile.unwrap())
}

pub fn create_profile(
    uuid_user: Uuid,
    data_profile: NewProfileUserDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    get_user(uuid_user, conn)?;

    if get_user_profile(uuid_user, conn).is_ok() {
        return Err(ResponseError {
            code: Status::Conflict.code,
            message: "The profile was created".to_string(),
        });
    }

    let profile: NewProfileUser = NewProfileUser {
        first_name: data_profile.first_name.unwrap(),
        gender: data_profile.gender.unwrap(),
        preference_lang: data_profile.preference_lang.unwrap(),
        last_name: data_profile.last_name,
        photo_url: data_profile.photo_url,
        years_old: data_profile.years_old,
        user_id: uuid_user,
    };

    let insert_action: bool = diesel::insert_into(profile_users::table)
        .values(&profile)
        .execute(conn)
        .is_ok();

    if !insert_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown Error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The profile has been created".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn update_user(
    uuid_user: Uuid,
    payload: UpdateUserDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_user: User = get_user(uuid_user, conn)?;

    if let Some(email) = &payload.email {
        if get_user_by_email(email.to_owned(), conn).is_ok() {
            return Err(ResponseError {
                code: Status::Conflict.code,
                message: "The email is used".to_string(),
            });
        }
    }

    if let Some(username) = &payload.username {
        if get_user_by_username(username.to_owned(), conn).is_ok() {
            return Err(ResponseError {
                code: Status::Conflict.code,
                message: "The username is used".to_string(),
            });
        }
    }

    let data_user: UpdateUser = UpdateUser {
        username: payload.username,
        email: payload.email,
        password: None,
        updated_at: Some(Utc::now().naive_utc()),
        changed_password_at: None,
    };

    let update_action: bool = diesel::update(&result_user)
        .set(&data_user)
        .execute(conn)
        .is_ok();

    if !update_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown Error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "Account has been updated".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn update_profile(
    uuid_user: Uuid,
    payload: UpdateProfileUserDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_profile_user: ProfileUser = get_user_profile(uuid_user, conn)?;

    let data_profile: UpdateProfileUser = UpdateProfileUser {
        first_name: payload.first_name,
        gender: payload.gender,
        last_name: payload.last_name,
        photo_url: payload.photo_url,
        preference_lang: payload.preference_lang,
        updated_at: Some(Utc::now().naive_utc()),
        years_old: payload.years_old,
    };

    let update_action = diesel::update(&result_profile_user)
        .set(&data_profile)
        .execute(conn)
        .is_ok();

    if !update_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "successful profile update".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn update_password(
    uuid_user: Uuid,
    payload: UpdatePasswordDTO,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_user: User = get_user(uuid_user, conn)?;

    let valid_password: bool =
        verify(payload.password.to_owned().unwrap(), &result_user.password).unwrap();

    if !valid_password {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "The password not match".to_string(),
        });
    }

    let mut data_user: UpdateUser = UpdateUser {
        username: None,
        email: None,
        password: payload.new_password.to_owned(),
        updated_at: Some(Utc::now().naive_utc()),
        changed_password_at: None,
    };

    if let Some(password) = &payload.new_password {
        let hash_password: String = hash(password, DEFAULT_COST).unwrap();
        data_user.password = Some(hash_password);
        data_user.changed_password_at = Some(Utc::now().naive_utc())
    }

    let update_action: bool = diesel::update(&result_user)
        .set(&data_user)
        .execute(conn)
        .is_ok();

    if !update_action {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown Error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "The password has been updated".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}

pub fn delete_user_with_profile(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseSuccess, ResponseError> {
    let result_user: User = get_user(uuid_user, conn)?;
    let result_profile_user: Result<ProfileUser, ResponseError> = get_user_profile(uuid_user, conn);

    if let Ok(profile) = result_profile_user {
        let result_delete: bool = diesel::delete(&profile).execute(conn).is_ok();
        if result_delete && get_user(uuid_user, conn).is_err() {
            return Ok(ResponseSuccess {
                message: "Success user account and profile deleted".to_string(),
                data: serde_json::to_value("").unwrap(),
            });
        };
    }

    let result_delete: bool = diesel::delete(&result_user).execute(conn).is_ok();

    if !result_delete {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    Ok(ResponseSuccess {
        message: "Success user account deleted".to_string(),
        data: serde_json::to_value("").unwrap(),
    })
}
