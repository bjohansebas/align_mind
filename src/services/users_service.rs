use align_mind_server::models::response_model::{ResponseMessage, ResponseValue};
use align_mind_server::models::user_model::*;
use align_mind_server::schema::{profile_users, users};

use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use rocket::http::Status;
use uuid::Uuid;

pub fn get_user_account(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseValue<User>, ResponseMessage> {
    users::table
        .filter(users::user_id.eq(uuid_user))
        .first::<User>(conn)
        .map(|user| ResponseValue {
            code: Status::Accepted.code,
            value: user,
        })
        .map_err(|_| ResponseMessage {
            code: Some(Status::NotFound.code),
            message: "The id wasn't found".to_string(),
        })
}

pub fn get_user_account_by_email(
    email_user: String,
    conn: &mut PgConnection,
) -> Result<ResponseValue<User>, ResponseMessage> {
    users::table
        .filter(users::email.eq(email_user))
        .first::<User>(conn)
        .map(|user| ResponseValue {
            code: Status::Accepted.code,
            value: user,
        })
        .map_err(|_| ResponseMessage {
            code: Some(Status::NotFound.code),
            message: "The email was not register".to_string(),
        })
}

pub fn get_user_profile(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<ResponseValue<ProfileUser>, ResponseMessage> {
    let result_user: ResponseValue<User> = get_user_account(uuid_user, conn)?;

    ProfileUser::belonging_to(&result_user.value)
        .first::<ProfileUser>(conn)
        .map(|profile| ResponseValue {
            code: Status::Accepted.code,
            value: profile,
        })
        .map_err(|_| ResponseMessage {
            code: Some(Status::NotFound.code),
            message: "The profile hasn't been created".to_string(),
        })
}

pub fn create_profile(
    uuid_user: Uuid,
    data_profile: NewProfileUserDTO,
    conn: &mut PgConnection,
) -> ResponseMessage {
    if let Err(e) = get_user_account(uuid_user, conn) {
        return e;
    };

    if get_user_profile(uuid_user, conn).is_ok() {
        return ResponseMessage {
            code: Some(Status::Conflict.code),
            message: "The profile was created".to_string(),
        };
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
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Unknown Error".to_string(),
        };
    }

    ResponseMessage {
        code: Some(Status::Created.code),
        message: "The profile has been created".to_string(),
    }
}

pub fn update_user(
    uuid_user: Uuid,
    payload: UpdateUserDTO,
    conn: &mut PgConnection,
) -> ResponseMessage {
    let result_user = get_user_account(uuid_user, conn);

    if result_user.is_err() {
        return ResponseMessage {
            code: Some(Status::NotFound.code),
            message: "The id wasn't found".to_string(),
        };
    }

    if get_user_account_by_email(payload.email.to_owned().unwrap(), conn).is_ok() {
        return ResponseMessage {
            code: Some(Status::Conflict.code),
            message: "The email is used".to_string(),
        };
    }

    let data_user: UpdateUser = UpdateUser {
        email: payload.email,
        password: None,
        updated_at: Some(Utc::now().naive_utc()),
        changed_password_at: None,
    };

    let update_action: bool = diesel::update(&result_user.unwrap().value)
        .set(&data_user)
        .execute(conn)
        .is_ok();

    if !update_action {
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Unknown Error".to_string(),
        };
    }

    ResponseMessage {
        code: Some(Status::Ok.code),
        message: "Account has been updated".to_string(),
    }
}

pub fn update_profile(
    uuid_user: Uuid,
    payload: UpdateProfileUserDTO,
    conn: &mut PgConnection,
) -> ResponseMessage {
    let result_profile_user: Result<ResponseValue<ProfileUser>, ResponseMessage> =
        get_user_profile(uuid_user, conn);

    if let Err(e) = result_profile_user {
        return e;
    }

    let data_profile: UpdateProfileUser = UpdateProfileUser {
        first_name: payload.first_name,
        gender: payload.gender,
        last_name: payload.last_name,
        photo_url: payload.photo_url,
        preference_lang: payload.preference_lang,
        updated_at: Some(Utc::now().naive_utc()),
        years_old: payload.years_old,
    };

    let update_action = diesel::update(&result_profile_user.unwrap().value)
        .set(&data_profile)
        .execute(conn)
        .is_ok();

    if !update_action {
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Unknown error".to_string(),
        };
    }

    ResponseMessage {
        code: Some(Status::Ok.code),
        message: "successful profile update".to_string(),
    }
}

pub fn update_password(
    uuid_user: Uuid,
    payload: UpdatePasswordDTO,
    conn: &mut PgConnection,
) -> ResponseMessage {
    let result_user: Result<ResponseValue<User>, ResponseMessage> =
        get_user_account(uuid_user, conn);

    if let Err(e) = result_user {
        return e;
    }

    let user: User = result_user.unwrap().value;

    let valid_password: bool = verify(
        payload.password.to_owned().unwrap(),
        &user.password.to_owned(),
    )
    .unwrap();

    if !valid_password {
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "The password not match".to_string(),
        };
    }

    let hash_password: String = hash(payload.new_password.unwrap(), DEFAULT_COST).unwrap();

    let data_user: UpdateUser = UpdateUser {
        email: None,
        password: Some(hash_password),
        updated_at: Some(Utc::now().naive_utc()),
        changed_password_at: Some(Utc::now().naive_utc()),
    };

    let update_action: bool = diesel::update(&user).set(&data_user).execute(conn).is_ok();

    if !update_action {
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Unknown Error".to_string(),
        };
    }

    ResponseMessage {
        code: Some(Status::Ok.code),
        message: "The password has been updated".to_string(),
    }
}

pub fn delete_user_with_profile(uuid_user: Uuid, conn: &mut PgConnection) -> ResponseMessage {
    let result_user: Result<ResponseValue<User>, ResponseMessage> =
        get_user_account(uuid_user, conn);

    if let Err(e) = result_user {
        return e;
    }

    let user: User = result_user.unwrap().value;

    let result_delete: bool = diesel::delete(&user).execute(conn).is_ok();

    if !result_delete
        || get_user_account(uuid_user, conn).is_ok()
        || get_user_profile(uuid_user, conn).is_ok()
    {
        return ResponseMessage {
            code: Some(Status::BadRequest.code),
            message: "Fail user account and profile are not delete".to_string(),
        };
    };

    ResponseMessage {
        code: Some(Status::Ok.code),
        message: "Success user account and profile deleted".to_string(),
    }
}
