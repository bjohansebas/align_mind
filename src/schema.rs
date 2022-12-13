// @generated automatically by Diesel CLI.

diesel::table! {
    colors (color_id) {
        color_id -> Uuid,
        name_color -> Varchar,
        code_color -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    emotions (emotion_id) {
        emotion_id -> Uuid,
        name_emotion -> Varchar,
        color_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    places (place_id) {
        place_id -> Uuid,
        name_place -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        color_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    profile_users (profile_id) {
        profile_id -> Uuid,
        photo_url -> Nullable<Varchar>,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        years_old -> Nullable<Date>,
        preference_lang -> Varchar,
        gender -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    think_emotions (think_emotion_id) {
        think_emotion_id -> Uuid,
        think_id -> Uuid,
        emotion_id -> Uuid,
    }
}

diesel::table! {
    thinks (think_id) {
        think_id -> Uuid,
        user_id -> Uuid,
        place_id -> Uuid,
        is_archive -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    trash_thinks (trash_th_id) {
        trash_th_id -> Uuid,
        think_id -> Uuid,
        date_start -> Date,
        date_end -> Date,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        changed_password_at -> Nullable<Timestamp>,
        email -> Varchar,
        profile_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(emotions -> colors (color_id));
diesel::joinable!(places -> colors (color_id));
diesel::joinable!(places -> users (user_id));
diesel::joinable!(think_emotions -> emotions (emotion_id));
diesel::joinable!(think_emotions -> thinks (think_id));
diesel::joinable!(thinks -> places (place_id));
diesel::joinable!(thinks -> users (user_id));
diesel::joinable!(trash_thinks -> thinks (think_id));

diesel::allow_tables_to_appear_in_same_query!(
    colors,
    emotions,
    places,
    profile_users,
    think_emotions,
    thinks,
    trash_thinks,
    users,
);
