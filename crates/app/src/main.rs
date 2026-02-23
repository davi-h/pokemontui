mod bootstrap;
mod lifecycle;
mod runtime;
mod container;

use lifecycle::app::Application;

fn main() {
    let app = Application::build();
    app.run();
}