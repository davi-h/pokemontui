use std::path::{PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use tokio::{fs, io::AsyncWriteExt};
use reqwest::Client;

use contracts::sprites::sprite_gateway::SpriteGateway;

const RETRIES: usize = 3;

#[derive(Clone)]
pub struct SpriteGatewayImpl {
    cache_dir: Arc<PathBuf>,
    http: Client,
}

impl SpriteGatewayImpl {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            cache_dir: Arc::new(cache_dir),
            http: Client::new(),
        }
    }

    fn sprite_path(&self, name: &str) -> PathBuf {
        self.cache_dir.join(format!("{name}.png"))
    }

    fn sprite_url(name: &str) -> String {
        format!(
            "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png",
            name
        )
    }

    async fn download(&self, name: &str, path: &PathBuf) -> anyhow::Result<()> {
        let url = Self::sprite_url(name);

        for _ in 0..RETRIES {
            match self.http.get(&url).send().await {
                Ok(resp) if resp.status().is_success() => {
                    let bytes = resp.bytes().await?;
                    let mut file = fs::File::create(path).await?;
                    file.write_all(&bytes).await?;
                    return Ok(());
                }
                _ => continue,
            }
        }

        anyhow::bail!("failed to download sprite: {name}")
    }
}

#[async_trait]
impl SpriteGateway for SpriteGatewayImpl {
    async fn ensure(&self, name: &str) -> anyhow::Result<()> {
        let path = self.sprite_path(name);

        // cache hit
        if path.exists() {
            return Ok(());
        }

        // garante diretório
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // download
        self.download(name, &path).await
    }

    fn clone_box(&self) -> Box<dyn SpriteGateway> {
        Box::new(self.clone())
    }
}