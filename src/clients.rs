use crate::init_capnp::init;
use crate::proxy_capnp::thread;
use std::sync::RwLock;

pub struct Clients {
    pub init_client: RwLock<Option<init::Client>>,
    pub thread_client: RwLock<Option<thread::Client>>,
}

impl Clients {
    pub(crate) fn new() -> Self {
        Self {
            init_client: RwLock::new(None),
            thread_client: RwLock::new(None),
        }
    }
}
