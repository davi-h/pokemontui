use std::sync::Arc;

use tokio::sync::Semaphore;
use futures::stream::{self, StreamExt};

use engine::factory::species_registry::SpeciesRegistry;
use contracts::api::species_source::SpeciesSource;
use contracts::sprites::sprite_gateway::SpriteGateway;

const CONCURRENCY_LIMIT: usize = 8;

pub struct SpeciesRegistryBuilder;

impl SpeciesRegistryBuilder {
    pub async fn build(
        provider: &impl SpeciesSource,
        sprites: &impl SpriteGateway,
    ) -> anyhow::Result<SpeciesRegistry> {
        // 1 — carrega lista de espécies
        let list = provider.all_species()?;

        // 2 — cria limitador de concorrência
        let semaphore = Arc::new(Semaphore::new(CONCURRENCY_LIMIT));

        // 3 — filtra espécies com sprite válido (paralelo limitado)
        let valid: Vec<String> = stream::iter(list)
            .map(|name| {
                let semaphore = semaphore.clone();
                async move {
                    let _permit = semaphore.acquire().await.unwrap();

                    if sprites.ensure(&name).await.is_ok() {
                        Some(name)
                    } else {
                        None
                    }
                }
            })
            .buffer_unordered(CONCURRENCY_LIMIT)
            .filter_map(async move |x| x)
            .collect()
            .await;

        // 4 — constrói registry imediatamente
        let registry = SpeciesRegistry::new(valid.clone());

        // 5 — preload background completo
        let bg_list = valid.clone();
        let sprites_ref = sprites.clone_box();

        tokio::spawn(async move {
            let semaphore = Arc::new(Semaphore::new(CONCURRENCY_LIMIT));

            stream::iter(bg_list)
                .for_each_concurrent(CONCURRENCY_LIMIT, |name| {
                    let semaphore = semaphore.clone();
                    let sprites = sprites_ref.clone_box();

                    async move {
                        let _permit = semaphore.acquire().await.unwrap();
                        let _ = sprites.ensure(&name).await;
                    }
                })
                .await;
        });

        Ok(registry)
    }
}