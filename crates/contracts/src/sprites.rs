pub trait SpriteLoader {
    fn fetch(&self, name: &str);
}

pub trait SpriteGateway: Send + Sync {
    fn has(&self, name: &str) -> bool;
    fn request(&self, name: &str);
    fn request_batch(&self, names: &[String]);
}