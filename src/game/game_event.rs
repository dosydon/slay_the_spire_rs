use crate::game::effect::BattleEffect;
use crate::battle::battle_events::BattleEvent;

#[derive(Debug, Clone, PartialEq)]
pub enum GameEvent {
    CombatVictory,
    CombatStart,
    CardObtained,
    RelicObtained,
    Battle(BattleEvent),
}

pub trait GameEventListener: Send + Sync {
    fn on_game_event(&mut self, event: &GameEvent) -> Vec<BattleEffect>;
    fn is_active(&self) -> bool;
}