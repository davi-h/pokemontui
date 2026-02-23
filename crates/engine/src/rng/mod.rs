
use rand::Rng;

pub struct GameRng;

impl GameRng {
	pub fn chance(percent: u8) -> bool {
		let roll: u8 = rand::thread_rng().gen_range(0..=100);
		roll <= percent
	}

	pub fn range(min: u32, max: u32) -> u32 {
		rand::thread_rng().gen_range(min..=max)
	}
}
