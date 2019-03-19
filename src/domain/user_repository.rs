use crate::infrastructure::models::write::new_user::NewUser;
use crate::domain::user::User;

pub trait UserRepository {
    fn new() -> Self;
    fn add(&self, new_user: NewUser) -> User;
}
