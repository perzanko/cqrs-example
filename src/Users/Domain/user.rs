pub struct User {
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    created_at: u32,
}

impl User {
    pub fn new(id: String, first_name: String, last_name: String, email: String, created_at: u32) -> User {
        User {
            id: id,
            first_name: first_name,
            last_name: last_name,
            email: email,
            created_at: created_at,
        }
    }
}
