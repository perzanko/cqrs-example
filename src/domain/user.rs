use uuid::Uuid;
use std::time::SystemTime;
use crate::application::event::registered_user_event::RegisteredUserEvent;
use crate::application::event::Event;
use crate::infrastructure::event_bus::EVENT_BUS;

#[derive(Queryable)]
pub struct User {
    id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    created_at: SystemTime,
}

impl User {
    pub fn new(
        id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        created_at: SystemTime,
    ) -> Self {
        Self {
            id: id,
            first_name: first_name,
            last_name: last_name,
            email: email,
            created_at: created_at,
        }
    }

    pub fn register_user(&self) -> () {
        self.apply(Event::RegisteredUser);
    }

    fn apply(&self, event: Event) {
        match event {
            Event::RegisteredUser => {
                let mut data = RegisteredUserEvent {
                    first_name: self.first_name.clone().unwrap(),
                    last_name: self.last_name.clone().unwrap(),
                    email: self.email.clone().unwrap(),
                };
                post_event!(&EVENT_BUS, &mut data, RegisteredUserEvent);
            }
        }
    }
}
