use crate::game::game_event::{GameEvent, GameEventListener};
use crate::game::effect::BattleEffect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BurningBloodRelic {
    used: bool,
}

impl BurningBloodRelic {
    pub fn new() -> Self {
        BurningBloodRelic {
            used: false,
        }
    }
}

impl GameEventListener for BurningBloodRelic {
    fn on_game_event(&mut self, event: &GameEvent) -> Vec<BattleEffect> {
        match event {
            GameEvent::CombatVictory if !self.used => {
                self.used = true;
                vec![BattleEffect::Heal(6)]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        !self.used
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burning_blood_relic_creation() {
        let relic = BurningBloodRelic::new();
        assert!(relic.is_active());
    }

    #[test]
    fn test_burning_blood_triggers_on_combat_victory() {
        let mut relic = BurningBloodRelic::new();

        let combat_victory_event = GameEvent::CombatVictory;
        let effects = relic.on_game_event(&combat_victory_event);

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], BattleEffect::Heal(6));
        assert!(!relic.is_active()); // Used up
    }

    #[test]
    fn test_burning_blood_only_triggers_once() {
        let mut relic = BurningBloodRelic::new();

        let combat_victory_event = GameEvent::CombatVictory;

        // First combat victory triggers healing
        relic.on_game_event(&combat_victory_event);

        // Second combat victory should not trigger
        let effects = relic.on_game_event(&combat_victory_event);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_burning_blood_ignores_other_events() {
        let mut relic = BurningBloodRelic::new();

        let other_events = vec![
            GameEvent::CombatStart,
            GameEvent::CardObtained,
            GameEvent::RelicObtained,
        ];

        for event in other_events {
            let effects = relic.on_game_event(&event);
            assert_eq!(effects.len(), 0);
            assert!(relic.is_active()); // Still active
        }
    }
}