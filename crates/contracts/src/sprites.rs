pub trait SpriteLoader {
    fn fetch(&self, name: &str);
}