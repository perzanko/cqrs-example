#[derive(Debug)]
pub struct User {
    first_name: String,
    last_name: String,
    email: String,
}

impl User {
    pub fn new(first_name: String, last_name: String, email: String) -> User {
        User {
            first_name: first_name,
            last_name: last_name,
            email: email,
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
