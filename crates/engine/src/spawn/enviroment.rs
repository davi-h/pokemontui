pub struct Environment {
    pub hour: u8,
    pub weather: Weather,
}

pub enum Weather {
    Clear,
    Rain,
    Storm,
    Fog,
}

pub trait EnvironmentProvider {
    fn current(&self) -> Environment;
}