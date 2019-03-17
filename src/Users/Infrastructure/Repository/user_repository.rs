use diesel::RunQueryDsl;

use crate::users::infrastructure::repository::connection_manager::ConnectionManager;
use crate::write_schema::users;
use crate::users::domain::user_repository::UserRepository;
use crate::users::infrastructure::models::write::new_user::NewUser;

pub struct ORMUserRepository {
    connection_manager: ConnectionManager,
}

impl UserRepository for ORMUserRepository {
    fn new() -> Self {
        ORMUserRepository {
            connection_manager: ConnectionManager::new(),
        }
    }

    fn add(&self, new_user: NewUser) -> bool {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&self.connection_manager.connection_write)
            .expect("Error saving new post");
        true
    }
}
