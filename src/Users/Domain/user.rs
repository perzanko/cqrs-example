#[derive(Debug)]
pub struct User {
    id: String;
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

    pub fn first_name(&self) -> &String {
        &self.first_name
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
