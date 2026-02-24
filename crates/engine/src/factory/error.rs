use std::fmt;

#[derive(Debug)]
pub enum FactoryError {
    InvalidLevel,
    EmptyRegistry,
    UnknownSpecies(String),
    Internal(String),
}

impl fmt::Display for FactoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FactoryError::InvalidLevel => write!(f, "invalid level"),
            FactoryError::EmptyRegistry => write!(f, "species registry is empty"),
            FactoryError::UnknownSpecies(name) => write!(f, "unknown species: {}", name),
            FactoryError::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl std::error::Error for FactoryError {}

impl From<crate::factory::pokemon_factory::FactoryError> for FactoryError {
    fn from(e: crate::factory::pokemon_factory::FactoryError) -> Self {
        FactoryError::Internal(format!("{:?}", e))
    }
}