use super::user::User;

pub trait UserRepository {
    fn add(&self, user: User) -> User;
}
