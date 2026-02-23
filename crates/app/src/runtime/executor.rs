use crate::container::registry::ServiceRegistry;

pub struct RuntimeExecutor {
    container: ServiceRegistry,
}

impl RuntimeExecutor {
    pub fn new(container: ServiceRegistry) -> Self {
        Self { container }
    }

    pub fn start(self) {
        println!("runtime started");
    }
}