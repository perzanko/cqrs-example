use super::super::super::domain::user::User;
use super::super::super::domain::user_repository::UserRepository;

#[derive(Debug)]
pub struct ORMUserRepository; // TODO: implement real db connection...

impl UserRepository for ORMUserRepository {
    fn add(&self, user: User) -> User {
        user
    }
}
