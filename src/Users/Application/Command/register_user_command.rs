#[derive(Serialize, Deserialize)]
pub struct RegisterUserCommand {
    first_name: String,
    last_name: String,
    email: String,
}

impl RegisterUserCommand {
    pub fn new(first_name: String, last_name: String, email: String) -> RegisterUserCommand {
        RegisterUserCommand {
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
}
