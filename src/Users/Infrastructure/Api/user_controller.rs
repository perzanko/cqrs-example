use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::users::application::command::register_user_command::RegisterUserCommand;
use crate::users::application::handler::register_user_handler::RegisterUserCommandHandler;

#[post("/register", format = "application/json", data = "<data>")]
pub fn register(data: Json<RegisterUserCommand>) -> Status {
    let handler = RegisterUserCommandHandler::new();
    let command = RegisterUserCommand::new(
        data.first_name().clone(),
        data.last_name().clone(),
        data.email().clone(),
    );
    handler.handle(command);
    Status::Ok
}
