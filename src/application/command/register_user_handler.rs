use crate::application::command::register_user_command::RegisterUserCommand;
use crate::domain::user_repository::UserRepository;
use crate::infrastructure::models::write::new_user::NewUser;
use crate::infrastructure::repository::user_repository::ORMUserRepository;

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
        let added_user = self.user_repository.add(user);
        added_user.register_user();
    }
}
