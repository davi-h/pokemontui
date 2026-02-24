use std::sync::{Mutex, OnceLock};

use contracts::rng::GameRng;

static RNG: OnceLock<Mutex<Box<dyn GameRng + Send>>> = OnceLock::new();

pub struct RngProvider;

impl RngProvider {
    pub fn init(rng: Box<dyn GameRng + Send>) {
        let _ = RNG.set(Mutex::new(rng));
    }

    pub fn with<T>(f: impl FnOnce(&mut dyn GameRng) -> T) -> T {
        let mut guard = RNG
            .get()
            .expect("RNG not initialized")
            .lock()
            .expect("RNG lock poisoned");

        f(guard.as_mut())
    }
}
