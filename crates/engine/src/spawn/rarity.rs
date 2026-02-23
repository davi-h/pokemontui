pub trait SpawnModifier {
    fn modify(&self, chance: f32) -> f32;
}

pub struct ShinyModifier;

impl SpawnModifier for ShinyModifier {
    fn modify(&self, chance: f32) -> f32 {
        chance * (1.0 / 4096.0)
    }
}

pub struct LegendaryModifier;

impl SpawnModifier for LegendaryModifier {
    fn modify(&self, chance: f32) -> f32 {
        chance * 0.01
    }
}