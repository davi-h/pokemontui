use async_trait::async_trait;

#[async_trait]
pub trait SpriteGateway: Send + Sync {
    async fn ensure(&self, name: &str) -> anyhow::Result<()>;

    fn clone_box(&self) -> Box<dyn SpriteGateway>;
}