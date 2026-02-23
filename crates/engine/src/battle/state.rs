use domain::pokemon::entity::Pokemon;

pub struct BattleState {
    pub player: Pokemon,
    pub enemy: Pokemon,
    pub turn: u32,
    pub finished: bool,
}

impl BattleState {
    pub fn new(player: Pokemon, enemy: Pokemon) -> Self {
        Self {
            player,
            enemy,
            turn: 0,
            finished: false,
        }
    }
}