mod bootstrap;
mod router;
mod commands;
mod runtime;
mod container;
mod lifecycle;
mod services;

use infrastructure::rng::default_rng::DefaultRng;
use lifecycle::app::Application;
use services::rng_provider::RngProvider;

fn main() {
    RngProvider::init(Box::new(DefaultRng));
    let app = Application::build();
    app.run();
}
