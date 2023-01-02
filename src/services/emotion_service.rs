use align_mind_server::establish_connection;
use align_mind_server::models::emotion_model::*;
use align_mind_server::schema::emotions;

use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub fn get_emotion(uuid_emotion: Uuid) -> Option<Emotion> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_emotion: Result<Emotion, Error> = emotions::table
        .filter(emotions::emotion_id.eq(uuid_emotion))
        .first(connection);

    if let Ok(emotion) = result_emotion {
        Some(emotion)
    } else {
        None
    }
}

pub fn create_emotion(payload: NewEmotion) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    diesel::insert_into(emotions::table)
        .values(&payload)
        .execute(connection)
        .is_ok()
}

pub fn update_emotion(uuid_emotion: Uuid, mut payload: UpdateEmotion) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_emotion: Option<Emotion> = get_emotion(uuid_emotion);

    if let Some(emotion) = result_emotion {
        payload.updated_at = Some(Utc::now().naive_utc());

        return diesel::update(&emotion)
            .set(&payload)
            .execute(connection)
            .is_ok();
    }

    false
}

pub fn delete_emotion(uuid_emotion: Uuid) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_emotion: Option<Emotion> = get_emotion(uuid_emotion);

    if let Some(emotion) = result_emotion {
        return diesel::delete(&emotion).execute(connection).is_ok();
    }

    false
}
