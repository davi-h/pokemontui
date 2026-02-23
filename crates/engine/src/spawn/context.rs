#[derive(Clone)]
pub struct SpawnContext {
    pub time_of_day: TimeOfDay,
    pub weather: Weather,
    pub player_level: u8,
}

#[derive(Clone, PartialEq)]
pub enum TimeOfDay {
    Day,
    Night,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Weather {
    Clear,
    Rain,
    Storm,
    Fog,
}
