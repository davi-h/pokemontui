use application::commands::start_battle::StartBattle;
use crate::container::registry::ServiceRegistry;
use engine::battle::engine::BattleEngine;

pub struct UseCases<'a> {
    pub start_battle: StartBattle<'a>,
}

pub fn build_usecases<'a>(registry: &'a mut ServiceRegistry) -> UseCases<'a> {
    let engine = registry.resolve_mut::<BattleEngine>().unwrap();
    UseCases {
        start_battle: StartBattle::new(engine),
    }
}