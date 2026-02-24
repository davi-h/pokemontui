use crate::api::error::ApiError;

pub trait SpeciesSource {
    fn all_species(&self) -> Result<Vec<String>, ApiError>;
}