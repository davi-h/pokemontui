#[derive(Clone, Debug)]
pub struct Stats {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub speed: u16,
}

impl Stats {
    /// Cria stats vindos diretamente da PokéAPI
    pub fn from_base(
        hp: u16,
        attack: u16,
        defense: u16,
        special_attack: u16,
        special_defense: u16,
        speed: u16,
    ) -> Self {
        Self {
            hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
        }
    }

    /// Gera stats zerados (útil para builder default)
    pub fn zero() -> Self {
        Self {
            hp: 0,
            attack: 0,
            defense: 0,
            special_attack: 0,
            special_defense: 0,
            speed: 0,
        }
    }

    /// Escala stats baseado no level
    pub fn scale_with_level(&self, level: u8) -> Self {
        let lvl = level as u16;

        Self {
            hp: self.hp + lvl * 2,
            attack: self.attack + lvl,
            defense: self.defense + lvl,
            special_attack: self.special_attack + lvl,
            special_defense: self.special_defense + lvl,
            speed: self.speed + lvl,
        }
    }
    pub fn from_level(level: u8) -> Self {
        let lvl = level as u16;

        Self {
            hp: 10 + lvl * 2,
            attack: 5 + lvl,
            defense: 5 + lvl,
            special_attack: 5 + lvl,
            special_defense: 5 + lvl,
            speed: 5 + lvl,
        }
    }
}