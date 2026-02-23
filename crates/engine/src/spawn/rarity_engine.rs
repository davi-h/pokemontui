pub struct RarityEngine;

impl RarityEngine {
    pub fn shiny_chance(&self, env: &Environment) -> f32 {
        let base = 0.02;

        let weather_bonus = match env.weather {
            Weather::Storm => 0.03,
            Weather::Fog => 0.015,
            _ => 0.0,
        };

        let night_bonus = if env.hour >= 20 || env.hour <= 5 {
            0.01
        } else {
            0.0
        };

        base + weather_bonus + night_bonus
    }
}