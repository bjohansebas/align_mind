use align_mind_server::establish_connection;
use align_mind_server::models::emotion_model::*;
use align_mind_server::schema::emotions;

use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_emotion(uuid_emotion: Uuid) -> Emotion {
    let connection: &mut PgConnection = &mut establish_connection();

    emotions::table
        .filter(emotions::emotion_id.eq(uuid_emotion))
        .first(connection)
        .unwrap()
}

pub fn create_emotion(payload: NewEmotion) -> Emotion {
    let connection: &mut PgConnection = &mut establish_connection();

    diesel::insert_into(emotions::table)
        .values(&payload)
        .get_result(connection)
        .unwrap()
}

pub fn update_emotion(uuid_emotion: Uuid, mut payload: UpdateEmotion) {
    let connection: &mut PgConnection = &mut establish_connection();

    let emotion: Emotion = get_emotion(uuid_emotion);

    payload.updated_at = Some(Utc::now().naive_utc());

    diesel::update(&emotion)
        .set(&payload)
        .execute(connection)
        .unwrap();
}

pub fn delete_emotion(uuid_emotion: Uuid) {
    let connection: &mut PgConnection = &mut establish_connection();

    let emotion: Emotion = get_emotion(uuid_emotion);

    diesel::delete(&emotion).execute(connection).unwrap();
}
