pub struct Scheduler;

impl Scheduler {
    pub fn new() -> Self {
        Self
    }

    pub fn tick(&self) {
        println!("tick");
    }
}