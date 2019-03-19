mod user_controller;

pub fn user_routes() -> Vec<rocket::Route> {
    routes![
        user_controller::register
    ]
}