use crate::game::effect::Effect;

#[derive(Debug, Clone, PartialEq)]
pub enum GameEvent {
    CombatVictory,
    CombatStart,
    CardObtained,
    RelicObtained,
}

pub trait GameEventListener {
    fn on_game_event(&mut self, event: &GameEvent) -> Vec<Effect>;
    fn is_active(&self) -> bool;
}