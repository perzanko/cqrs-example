use crate::users::application::command::register_user_command::RegisterUserCommand;
use crate::users::domain::user_repository::UserRepository;
use crate::users::infrastructure::models::write::new_user::NewUser;
use crate::users::infrastructure::repository::user_repository::ORMUserRepository;

pub struct RegisterUserCommandHandler {
    user_repository: ORMUserRepository,
}

impl RegisterUserCommandHandler {
    pub fn new() -> RegisterUserCommandHandler {
        RegisterUserCommandHandler {
            user_repository: ORMUserRepository::new(),
        }
    }

    pub fn handle(&self, command: RegisterUserCommand) -> () {
        let user = NewUser {
            first_name: command.first_name().clone(),
            last_name: command.last_name().clone(),
            email: command.email().clone(),
        };
        self.user_repository.add(user);
    }
}
