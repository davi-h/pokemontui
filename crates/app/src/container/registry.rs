use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct ServiceRegistry {
    services: HashMap<TypeId, Box<dyn Any>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }

    pub fn resolve<T: 'static>(&self) -> Option<&T> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|s| s.downcast_ref())
    }

    pub fn resolve_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.services
            .get_mut(&TypeId::of::<T>())
            .and_then(|s| s.downcast_mut())
    }
}