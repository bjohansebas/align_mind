use align_mind_server::establish_connection;
use align_mind_server::models::color_model::Color;
use align_mind_server::models::emotion_model::*;
use align_mind_server::schema::emotions;

use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

use super::color_service::get_color;

pub fn get_emotion(uuid_emotion: Uuid) -> Option<Emotion> {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_emotion: Result<Emotion, Error> = emotions::table
        .filter(emotions::emotion_id.eq(uuid_emotion))
        .first::<Emotion>(connection);

    if let Ok(emotion) = result_emotion {
        Some(emotion)
    } else {
        None
    }
}

pub fn create_emotion(payload: NewEmotionDTO) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let uuid_color: Result<Uuid, uuid::Error> = Uuid::parse_str(payload.color_id.unwrap().as_str());

    if let Ok(uuid) = uuid_color {
        let result_color: Option<Color> = get_color(uuid);

        if let Some(color) = result_color {
            let emotion: NewEmotion = NewEmotion {
                color_id: color.color_id,
                name_emotion: payload.name_emotion.unwrap(),
            };

            return diesel::insert_into(emotions::table)
                .values(&emotion)
                .execute(connection)
                .is_ok();
        }
    }
    false
}

pub fn update_emotion(uuid_emotion: Uuid, payload: UpdateEmotionDTO) -> bool {
    let connection: &mut PgConnection = &mut establish_connection();

    let result_emotion: Option<Emotion> = get_emotion(uuid_emotion);

    if let Some(emotion) = result_emotion {
        let mut data_emotion: UpdateEmotion = UpdateEmotion {
            name_emotion: payload.name_emotion,
            color_id: None,
            updated_at: Some(Utc::now().naive_utc()),
        };

        if let Some(color_id) = payload.color_id {
            let uuid_color: Result<Uuid, uuid::Error> = Uuid::parse_str(color_id.as_str());
            if let Ok(uuid) = uuid_color {
                let result_color: Option<Color> = get_color(uuid);
                if let Some(color) = result_color {
                    data_emotion.color_id = Some(color.color_id);
                }
            } else {
                return false;
            }
        }

        return diesel::update(&emotion)
            .set(&data_emotion)
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
