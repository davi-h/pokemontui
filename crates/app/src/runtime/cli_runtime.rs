use crate::router::router::Router;
use crate::container::command_registry::build_registry;

pub struct CliRuntime;

impl CliRuntime {
    pub fn run() {
        let registry = build_registry();
        let router = Router::new(registry);

        if let Err(err) = router.run(std::env::args().collect()) {
            eprintln!("{:?}", err);
        }
    }
}