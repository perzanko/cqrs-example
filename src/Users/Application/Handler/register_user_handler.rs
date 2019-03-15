use super::super::super::domain::user::User;
use super::super::super::domain::user_repository::UserRepository;
use super::super::super::infrastructure::repository::user_repository::ORMUserRepository;
use super::super::command::register_user_command::RegisterUserCommand;

pub struct RegisterUserCommandHandler {
    user_repository: ORMUserRepository,
}

impl RegisterUserCommandHandler {
    pub fn new(repository: ORMUserRepository) -> RegisterUserCommandHandler {
        RegisterUserCommandHandler {
            user_repository: repository,
        }
    }

    pub fn handle(&self, command: RegisterUserCommand) -> () {
        let user = User::new(
            command.first_name().clone(),
            command.last_name().clone(),
            command.email().clone(),
        );
        self.user_repository.add(user);
    }
}
