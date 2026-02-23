impl Default for TurnManager {
    fn default() -> Self {
        Self::new()
    }
}
pub struct TurnManager {
    turn: u32,
}

impl TurnManager {
    pub fn new() -> Self {
        Self { turn: 0 }
    }

    pub fn next_turn(&mut self) {
        self.turn += 1;
    }

    pub fn current(&self) -> u32 {
        self.turn
    }
}