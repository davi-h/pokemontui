use super::{cache::DiskCache, client::fetch_species};

pub struct CachedSpeciesProvider {
    cache: DiskCache<String>,
    path: String,
}

impl CachedSpeciesProvider {
    pub fn new(path: &str) -> Self {
        Self {
            cache: DiskCache::load(path),
            path: path.into(),
        }
    }

    pub async fn get(&mut self, name: &str) -> anyhow::Result<String> {
        if let Some(val) = self.cache.get(name) {
            if !self.cache.is_stale(name) {
                return Ok(val);
            }
        }

        let fresh = fetch_species(name).await?;
        self.cache.set(name.into(), fresh.clone());
        self.cache.save(&self.path);

        Ok(fresh)
    }
}