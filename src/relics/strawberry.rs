use crate::game::game_event::{GameEvent, GameEventListener};
use crate::game::effect::BattleEffect;
use serde::{Deserialize, Serialize};

/// Strawberry - Raise your Max HP by 7 (one-time effect on pickup)
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrawberryRelic {
    applied: bool,
}

impl StrawberryRelic {
    pub fn new() -> Self {
        Self { applied: false }
    }
}

impl GameEventListener for StrawberryRelic {
    fn on_game_event(&mut self, event: &GameEvent) -> Vec<BattleEffect> {
        match event {
            GameEvent::RelicObtained if !self.applied => {
                self.applied = true;
                // Apply Max HP increase
                vec![BattleEffect::HealAndIncreaseMaxHp(7)]
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
    fn test_strawberry_creation() {
        let strawberry = StrawberryRelic::new();
        assert!(!strawberry.applied);
    }

    #[test]
    fn test_strawberry_applies_on_pickup() {
        let mut strawberry = StrawberryRelic::new();

        let effects = strawberry.on_game_event(&GameEvent::RelicObtained);

        assert_eq!(effects.len(), 1);
        assert!(matches!(
            effects[0],
            BattleEffect::HealAndIncreaseMaxHp(7)
        ));
        assert!(strawberry.applied);
    }

    #[test]
    fn test_strawberry_only_applies_once() {
        let mut strawberry = StrawberryRelic::new();

        // First pickup
        let effects1 = strawberry.on_game_event(&GameEvent::RelicObtained);
        assert_eq!(effects1.len(), 1);
        assert!(strawberry.applied);

        // Second pickup (shouldn't happen in game, but test safety)
        let effects2 = strawberry.on_game_event(&GameEvent::RelicObtained);
        assert_eq!(effects2.len(), 0);
    }
}
