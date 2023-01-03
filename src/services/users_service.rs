use align_mind_server::establish_connection;
use align_mind_server::models::user_model::*;
use align_mind_server::schema::{profile_users, users};

use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_user(uuid_user: Uuid) -> Option<User> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Result<User, diesel::result::Error> = users::table
        .filter(users::user_id.eq(uuid_user))
        .first::<User>(connection);

    if let Ok(user) = result_user {
        return Some(user);
    }
    None
}

pub fn get_user_by_email(email_user: &String) -> Option<User> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Result<User, diesel::result::Error> = users::table
        .filter(users::email.eq(email_user))
        .first::<User>(connection);

    if let Ok(user) = result_user {
        return Some(user);
    }
    None
}

pub fn get_user_profile(uuid_user: Uuid) -> Option<ProfileUser> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Option<User> = get_user(uuid_user);

    if let Some(user) = result_user {
        let result_profile: Result<ProfileUser, diesel::result::Error> =
            ProfileUser::belonging_to(&user).first::<ProfileUser>(connection);

        if let Ok(profile) = result_profile {
            return Some(profile);
        }
    }
    None
}

pub fn exist_email(email: String) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let exist_user: bool = users::table
        .filter(users::email.eq(&email))
        .first::<User>(connection)
        .is_ok();

    exist_user
}

pub fn exist_username(username: String) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let exist_user: bool = users::table
        .filter(users::username.eq(&username))
        .first::<User>(connection)
        .is_ok();

    exist_user
}

pub fn create_profile(uuid_user: Uuid, mut data_profile: NewProfileUser) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Option<User> = get_user(uuid_user);
    if let Some(user) = result_user {
        data_profile.user_id = user.user_id;

        let insert_profile: bool = diesel::insert_into(profile_users::table)
            .values(&data_profile)
            .execute(connection)
            .is_ok();

        return insert_profile;
    }
    false
}

pub fn update_user(uuid_user: Uuid, mut payload: UpdateUser) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Option<User> = get_user(uuid_user);
    if let Some(user) = result_user {
        if let Some(password) = &payload.password {
            let hash_password: String = hash(password, DEFAULT_COST).unwrap();
            payload.password = Some(hash_password);
            payload.changed_password_at = Some(Utc::now().naive_utc())
        }

        if let Some(email) = &payload.email {
            if exist_email(email.to_owned()) {
                return false;
            }
        }

        if let Some(username) = &payload.username {
            if exist_username(username.to_owned()) {
                return false;
            }
        }

        payload.updated_at = Some(Utc::now().naive_utc());

        return diesel::update(&user)
            .set(&payload)
            .execute(connection)
            .is_ok();
    }
    false
}

pub fn update_profile(uuid_user: Uuid, mut payload: UpdateProfileUser) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_profile_user: Option<ProfileUser> = get_user_profile(uuid_user);

    if let Some(profile) = result_profile_user {
        payload.updated_at = Some(Utc::now().naive_utc());

        return diesel::update(&profile)
            .set(&payload)
            .execute(connection)
            .is_ok();
    }
    false
}

pub fn delete_user_with_profile(uuid_user: Uuid) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_user: Option<User> = get_user(uuid_user);
    let result_profile_user: Option<ProfileUser> = get_user_profile(uuid_user);

    if let Some(user) = result_user {
        let result_delete: bool = diesel::delete(&user).execute(connection).is_ok();

        if let Some(profile) = result_profile_user {
            return diesel::delete(&profile).execute(connection).is_ok();
        }
        return result_delete;
    }
    false
}
