use align_mind_server::establish_connection;
use diesel::prelude::*;
use chrono::Utc;
use diesel::result::Error;
use uuid::Uuid;

use align_mind_server::models::user_model::*;
use align_mind_server::schema::{profile_users,users};

pub fn create_user_with_profile(data_profile: NewProfileUser,mut data_user: NewUser ) -> (ProfileUser,User){
    let mut connection = establish_connection();

    connection.transaction::<(ProfileUser,User),Error,_>(|conn| {
        let profile_user:ProfileUser = diesel::insert_into(profile_users::table)
            .values(&data_profile)
            .get_result(conn).unwrap();
        data_user.profile_id=Some(profile_user.profile_id);
        let user= diesel::insert_into(users::table)
            .values(&data_user)
            .get_result(conn).unwrap();
        Ok((profile_user,user))
    }).unwrap()
}

pub fn get_user(user_id: String) -> User {
    let connection = &mut establish_connection();
    users::table
        .filter(users::email.eq(user_id))
        .first(connection)
        .unwrap()
}

pub fn get_user_profile(user_id: String) -> ProfileUser{
    let connection = &mut establish_connection();
    let user: User = users::table
        .filter(users::email.eq(user_id))
        .first(connection)
        .unwrap();
    profile_users::table
        .filter(profile_users::profile_id.eq(&user.profile_id))
        .first(connection)
        .unwrap()
}

pub fn delete_user_with_profile(user_id: Uuid){
    let connection = &mut establish_connection();
    let user: User = users::table
        .filter(users::user_id.eq(user_id))
        .first(connection)
        .unwrap();
    let profile_user:ProfileUser = profile_users::table
        .filter(profile_users::profile_id.eq(&user.profile_id))
        .first(connection)
        .unwrap();
    diesel::delete(&user)
        .execute(connection)
        .unwrap();
    diesel::delete(&profile_user)
        .execute(connection)
        .unwrap();
}

pub fn update_user(user_id: Uuid, mut payload: UpdateUser) {
    let connection = &mut establish_connection();
    let user: User = users::table
        .filter(users::user_id.eq(user_id))
        .first(connection)
        .unwrap();
    if !payload.password.eq(&Some("".to_string())) {
        payload.changed_password_at = Some(Utc::now().naive_utc())
    }
    payload.updated_at = Some(Utc::now().naive_utc());
    diesel::update(&user)
        .set(&payload)
        .execute(connection)
        .unwrap();
}

pub fn update_profile(profile_id: Uuid, mut payload: UpdateProfileUser){
    let connection = &mut establish_connection();
    let profile_user: ProfileUser = profile_users::table
    .filter(profile_users::profile_id.eq(&profile_id))
    .first(connection)
    .unwrap();
    payload.updated_at = Some(Utc::now().naive_utc());
    diesel::update(&profile_user)
        .set(&payload)
        .execute(connection)
        .unwrap();
}