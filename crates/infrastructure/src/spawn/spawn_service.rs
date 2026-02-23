use contracts::spawn::SpawnService;

pub struct DefaultSpawnService;

impl SpawnService for DefaultSpawnService {
    fn spawn(&self) {
        println!("a wild pokemon appeared");
    }
}