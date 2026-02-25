#[derive(Clone, Debug)]
pub struct SpawnEntry {
    /// Nome da espécie
    pub species: String,

    /// Peso base antes de modificadores
    pub base_weight: u32,

    /// Intervalo de level possível
    pub min_level: u8,
    pub max_level: u8,

    /// Condições opcionais de spawn
    pub conditions: SpawnConditions,
}

#[derive(Clone, Debug, Default)]
pub struct SpawnConditions {
    /// Biomas permitidos (vazio = qualquer)
    pub allowed_biomes: Vec<String>,

    /// Climas permitidos (vazio = qualquer)
    pub allowed_weather: Vec<String>,

    /// Eventos globais necessários
    pub required_events: Vec<String>,

    /// Modificador percentual de peso
    /// 1.0 = neutro
    pub weight_multiplier: f32,

    /// Permite spawnar shiny diretamente
    pub force_shiny: bool,
}

impl SpawnEntry {
    /// Peso efetivo após modificadores
    pub fn effective_weight(&self) -> u32 {
        ((self.base_weight as f32) * self.conditions.weight_multiplier)
            .max(0.0) as u32
    }

    /// Validação estrutural
    pub fn is_valid(&self) -> bool {
        self.base_weight > 0 && self.min_level <= self.max_level
    }
}