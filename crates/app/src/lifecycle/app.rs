use crate::router::router::Router;
use crate::container::command_registry::build_registry;

pub struct Application {
    router: Router,
}

impl Application {
    pub fn build() -> Self {
        let registry = build_registry();
        let router = Router::new(registry);

        Self { router }
    }

    pub fn run(self) {
        let args: Vec<String> = std::env::args().collect();

        if let Err(err) = self.router.run(args) {
            eprintln!("Error: {:?}", err);
        }
    }
}