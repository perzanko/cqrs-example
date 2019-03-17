use crate::users::domain::user::User;
use crate::users::infrastructure::models::write::new_user::NewUser;

pub trait UserRepository {    
    fn new() -> Self;
    fn add(&self, new_user: NewUser) -> bool;
}
