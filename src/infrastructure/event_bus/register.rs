use super::EVENT_BUS;
use crate::application::event::registered_user_event::RegisteredUserEvent;
use crate::application::event::registered_user_listener;


pub fn register_events() {
    register_hook!(&EVENT_BUS, 0, RegisteredUserEvent, registered_user_listener::execute);
}