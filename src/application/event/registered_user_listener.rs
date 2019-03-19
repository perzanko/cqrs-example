use crate::application::event::registered_user_event::RegisteredUserEvent;

pub fn execute(event: &mut RegisteredUserEvent) -> () {
    println!("event -> {:?}", event);
}
