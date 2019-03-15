use super::super::super::application::command::register_user_command::RegisterUserCommand;
use super::super::super::application::handler::register_user_handler::RegisterUserCommandHandler;
use super::super::repository::user_repository::ORMUserRepository;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/register", format = "application/json", data = "<data>")]
pub fn register(data: Json<RegisterUserCommand>) -> Status {
    let handler = RegisterUserCommandHandler::new(ORMUserRepository);
    let command = RegisterUserCommand::new(
        data.first_name().clone(),
        data.last_name().clone(),
        data.email().clone(),
    );
    handler.handle(command);
    Status::Ok
}
