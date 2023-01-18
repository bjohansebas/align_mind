use align_mind_server::{
    models::{
        color_model::Color, emotion_model::Emotion, response_model::ResponseError,
        statistics_model::AllEmotions, think_emotion_model::ThinkEmotion, think_model::Think,
    },
    schema::emotions,
};

use diesel::prelude::*;
use rocket::http::Status;
use std::collections::HashMap;
use uuid::Uuid;

use super::{color_service::get_color, think_service::get_thinks_with_user_uuid};

pub fn get_postive_and_negative(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<u32>, ResponseError> {
    let thinks: Vec<Think> = get_thinks_with_user_uuid(uuid_user, conn)?;
    let mut positive: u32 = 0;
    let mut negative: u32 = 0;

    let emotions = ThinkEmotion::belonging_to(&thinks)
        .inner_join(emotions::table)
        .select(emotions::all_columns)
        .load::<Emotion>(conn);

    if emotions.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }

    for emotion in emotions.unwrap() {
        if emotion.type_emotion.to_lowercase().eq("positive") {
            positive += 1;
        } else {
            negative += 1;
        }
    }

    Ok(vec![positive, negative])
}

pub fn get_emotion_filter(
    uuid_user: Uuid,
    type_emotion: &str,
    conn: &mut PgConnection,
) -> Result<Vec<(String, u32, String)>, ResponseError> {
    let thinks: Vec<Think> = get_thinks_with_user_uuid(uuid_user, conn)?;

    let emotions: Result<Vec<(String, String, Uuid)>, _> = ThinkEmotion::belonging_to(&thinks)
        .inner_join(emotions::table)
        .select((
            emotions::name_emotion,
            emotions::type_emotion,
            emotions::color_id,
        ))
        .filter(emotions::type_emotion.eq(type_emotion))
        .load::<(String, String, Uuid)>(conn);

    if emotions.is_err() {
        return Err(ResponseError {
            code: Status::BadRequest.code,
            message: "Unknown error".to_string(),
        });
    }
    let mut frequency: HashMap<_, u32> = HashMap::new();
    let mut colors: HashMap<_, String> = HashMap::new();

    let emotions: Vec<(String, String, Uuid)> = emotions.unwrap();

    for emotion in emotions {
        *frequency.entry(emotion.0.to_owned()).or_insert(0) += 1;

        let color: Color = get_color(emotion.2, conn)?;
        colors.insert(emotion.0, color.code_color);
    }

    let all_emotions: Vec<(String, u32)> = frequency
        .into_iter()
        .map(|(x, y)| (x, y))
        .collect::<Vec<(String, u32)>>();

    let mut result: Vec<(String, u32, String)> = Vec::new();

    for emotion in all_emotions {
        let color: String = colors.get(&emotion.0).unwrap().to_owned();
        result.push((emotion.0, emotion.1, color))
    }

    Ok(result)
}

pub fn get_postive_and_negative_filtre(
    uuid_user: Uuid,
    conn: &mut PgConnection,
) -> Result<AllEmotions, ResponseError> {
    let positive: Vec<(String, u32, String)> = get_emotion_filter(uuid_user, "Positive", conn)?;
    let negative: Vec<(String, u32, String)> = get_emotion_filter(uuid_user, "Negative", conn)?;

    Ok(AllEmotions { positive, negative })
}
