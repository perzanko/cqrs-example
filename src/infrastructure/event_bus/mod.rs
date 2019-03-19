use eventbus::EventBus as EB;

pub mod register;

lazy_static! {
    pub static ref EVENT_BUS: EB = EB::new();
}
