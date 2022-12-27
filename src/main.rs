#[macro_use]
extern crate rocket;
use rocket::launch;
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
                sign_up,
                delete_account,
                updating_user,
                update_profile_user
            ],
        )
        .mount("/colors", routes![])
}
