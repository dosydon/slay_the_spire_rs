use crate::game::game_event::{GameEvent, GameEventListener};
use crate::game::effect::BattleEffect;
use serde::{Deserialize, Serialize};

/// Mango - Raise your Max HP by 14 (one-time effect on pickup)
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MangoRelic {
    applied: bool,
}

impl MangoRelic {
    pub fn new() -> Self {
        Self { applied: false }
    }
}

impl GameEventListener for MangoRelic {
    fn on_game_event(&mut self, event: &GameEvent) -> Vec<BattleEffect> {
        match event {
            GameEvent::RelicObtained if !self.applied => {
                self.applied = true;
                vec![BattleEffect::HealAndIncreaseMaxHp(14)]
            }
            _ => vec![],
        }
    }

    fn is_active(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mango_creation() {
        let mango = MangoRelic::new();
        assert!(!mango.applied);
    }

    #[test]
    fn test_mango_applies_on_pickup() {
        let mut mango = MangoRelic::new();

        let effects = mango.on_game_event(&GameEvent::RelicObtained);

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::HealAndIncreaseMaxHp(14)));
        assert!(mango.applied);
    }

    #[test]
    fn test_mango_only_applies_once() {
        let mut mango = MangoRelic::new();

        let effects1 = mango.on_game_event(&GameEvent::RelicObtained);
        assert_eq!(effects1.len(), 1);
        assert!(mango.applied);

        let effects2 = mango.on_game_event(&GameEvent::RelicObtained);
        assert_eq!(effects2.len(), 0);
    }
}
