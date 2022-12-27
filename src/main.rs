#[macro_use]
extern crate rocket;
use rocket::launch;
use routes::color_route::*;
use routes::emotion_route::*;
use routes::place_route::*;
use routes::think_route::*;
use routes::trash_route::*;
use routes::users_route::*;

mod routes;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/users",
            routes![
                getting_user,
                getting_profile,
                getting_places_of_user,
                getting_thinks_of_user,
                getting_trash_of_user,
                sign_up,
                delete_account,
                updating_user,
                update_profile_user
            ],
        )
        .mount(
            "/colors",
            routes![getting_color, updating_color, deleting_color, save_color],
        )
        .mount(
            "/places",
            routes![getting_place, save_place, deleting_place, updating_place],
        )
        .mount(
            "/thinks",
            routes![
                getting_think,
                save_think,
                deleting_think,
                updating_think,
                move_to_trash
            ],
        )
        .mount("/trash", routes![getting_trash, restore_think])
        .mount(
            "/emotions",
            routes![
                getting_emotion,
                save_emotion,
                deleting_emotion,
                updating_emotion
            ],
        )
}
