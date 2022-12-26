use align_mind_server::establish_connection;
use align_mind_server::models::user_model::*;
use align_mind_server::schema::{profile_users, users};

use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use regex::Regex;
use uuid::Uuid;

pub fn sign_up(mut data_profile: NewProfileUser, mut data_user: NewUser) -> (ProfileUser, User) {
    let mut connection: PgConnection = establish_connection();

    connection
        .transaction::<(ProfileUser, User), DieselError, _>(|conn: &mut PgConnection| {
            let hash_password: String = hash(&data_user.password, DEFAULT_COST).unwrap();
            data_user.password = hash_password;

            let user: User = diesel::insert_into(users::table)
                .values(&data_user)
                .get_result(conn)
                .unwrap();

            data_profile.user_id = Some(user.user_id);

            let profile_user: ProfileUser = diesel::insert_into(profile_users::table)
                .values(&data_profile)
                .get_result(conn)
                .unwrap();

            Ok((profile_user, user))
        })
        .unwrap()
}

pub fn get_user(uuid_user: Uuid) -> User {
    let connection: &mut PgConnection = &mut establish_connection();

    users::table
        .filter(users::user_id.eq(uuid_user))
        .first(connection)
        .unwrap()
}

pub fn get_user_profile(uuid_user: Uuid) -> ProfileUser {
    let connection: &mut PgConnection = &mut establish_connection();

    let user: User = get_user(uuid_user);

    ProfileUser::belonging_to(&user).first(connection).unwrap()
}

pub fn verify_new_email(email: String) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let exist_user: bool = users::table
        .filter(users::email.eq(&email))
        .first::<User>(connection)
        .is_ok();

    let regex_email: Regex = Regex::new(r"^[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$").unwrap();
    let test_regex: bool = regex_email.is_match(&email);
    test_regex && !exist_user
}

pub fn delete_user_with_profile(uuid_user: Uuid) {
    let connection: &mut PgConnection = &mut establish_connection();

    let user: User = get_user(uuid_user);

    let profile_user: ProfileUser = get_user_profile(uuid_user);

    diesel::delete(&user).execute(connection).unwrap();

    diesel::delete(&profile_user).execute(connection).unwrap();
}

pub fn update_user(uuid_user: Uuid, mut payload: UpdateUser) {
    let connection: &mut PgConnection = &mut establish_connection();

    let user: User = get_user(uuid_user);

    let is_password: &bool = &payload.password.as_ref().unwrap().is_empty();
    if !is_password {
        let password: &String = &payload.password.to_owned().unwrap();
        let hash_password: String = hash(password, DEFAULT_COST).unwrap();

        payload.password = Some(hash_password);
        payload.changed_password_at = Some(Utc::now().naive_utc())
    }

    payload.updated_at = Some(Utc::now().naive_utc());

    diesel::update(&user)
        .set(&payload)
        .execute(connection)
        .unwrap();
}

pub fn update_profile(uuid_user: Uuid, mut payload: UpdateProfileUser) {
    let connection: &mut PgConnection = &mut establish_connection();

    let profile_user: ProfileUser = get_user_profile(uuid_user);

    payload.updated_at = Some(Utc::now().naive_utc());

    diesel::update(&profile_user)
        .set(&payload)
        .execute(connection)
        .unwrap();
}
