mod bootstrap;
mod router;
mod commands;
mod runtime;
mod container;
mod lifecycle;

use lifecycle::app::Application;

#[tokio::main]
async fn main() {
    let app = Application::build();
    app.run();
}