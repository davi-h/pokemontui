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

<<<<<<< HEAD
fn main() {
    RngProvider::init(Box::new(DefaultRng));
=======
#[tokio::main]
async fn main() {
>>>>>>> 694a416 (v0.0.4)
    let app = Application::build();
    app.run();
}
