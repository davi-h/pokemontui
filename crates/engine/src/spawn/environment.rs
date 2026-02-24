#[derive(Debug, Clone, Copy)]
pub struct Environment {
    pub hour: u8,
    pub weather: Weather,
}

impl Environment {
    pub fn new(hour: u8, weather: Weather) -> Self {
        Self {
            hour: hour % 24, // garante intervalo válido
            weather,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Weather {
    Clear,
    Rain,
    Storm,
    Fog,
}

pub trait EnvironmentProvider {
    fn current(&self) -> Environment;
}