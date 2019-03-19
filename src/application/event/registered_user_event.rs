use eventbus::{Event};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct RegisteredUserEvent {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl Event for RegisteredUserEvent {
}
