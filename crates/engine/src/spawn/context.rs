#[derive(Clone, Debug)]
pub struct SpawnContext {
    pub time_of_day: TimeOfDay,
    pub weather: Weather,
    pub player_level: u8,

    /// Eventos globais ativos (ex: eclipse, blood_moon)
    pub active_events: Vec<String>,

    /// Multiplicador global de spawn
    /// usado para eventos mundiais
    pub global_spawn_rate: f32,
}

impl SpawnContext {
    pub fn new(time_of_day: TimeOfDay, weather: Weather, player_level: u8) -> Self {
        Self {
            time_of_day,
            weather,
            player_level,
            active_events: Vec::new(),
            global_spawn_rate: 1.0,
        }
    }

    /// Verifica se um evento está ativo
    pub fn has_event(&self, event: &str) -> bool {
        self.active_events.iter().any(|e| e == event)
    }

    /// Aplica multiplicador global
    pub fn spawn_rate(&self) -> f32 {
        self.global_spawn_rate
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TimeOfDay {
    Day,
    Night,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Weather {
    Clear,
    Rain,
    Storm,
    Fog,
}