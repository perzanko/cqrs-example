#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket;

mod users;

use users::infrastructure::api::user_routes;

fn main() {
    rocket::ignite()
        .mount("/users/command/", user_routes())
        .launch();
}
