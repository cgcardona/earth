use tokio_core::reactor::{Core, Handle};

pub struct EventLoop {
    core: Core,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            core: Core::new().unwrap(),
        }
    }

    // pub fn create() -> Core {
    //     let mut event_loop: Core = Core::new().unwrap();
    // }
}
