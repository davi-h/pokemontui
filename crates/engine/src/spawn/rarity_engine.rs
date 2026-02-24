use crate::spawn::environment::{Environment, Weather};

const BASE: f32 = 0.02;
const STORM_BONUS: f32 = 0.03;
const FOG_BONUS: f32 = 0.015;
const NIGHT_BONUS: f32 = 0.01;
pub struct RarityEngine;

impl RarityEngine {
    /// Calcula chance de shiny baseada no ambiente
    pub fn shiny_chance(&self, env: &Environment) -> f32 {
        let base = BASE;

        let weather_bonus = match env.weather {
            Weather::Storm => STORM_BONUS,
            Weather::Fog => FOG_BONUS,
            _ => 0.0,
        };

        let night_bonus = if (20..=23).contains(&env.hour) || (0..=5).contains(&env.hour) {
            NIGHT_BONUS
        } else {
            0.0
        };

        // clamp garante que nunca passe de 100%
        (base + weather_bonus + night_bonus).clamp(0.0, 1.0)
    }
}