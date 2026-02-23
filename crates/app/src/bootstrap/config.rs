#[derive(Clone)]
pub struct AppConfig {
    pub app_name: String,
}

impl AppConfig {
    pub fn load() -> Self {
        Self {
            app_name: "poke-engine".into(),
        }
    }
}