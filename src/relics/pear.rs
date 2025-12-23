use crate::game::game_event::{GameEvent, GameEventListener};
use crate::game::effect::Effect;

/// Pear - Raise your Max HP by 10 (one-time effect on pickup)
pub struct PearRelic {
    applied: bool,
}

impl PearRelic {
    pub fn new() -> Self {
        Self { applied: false }
    }
}

impl GameEventListener for PearRelic {
    fn on_game_event(&mut self, event: &GameEvent) -> Vec<Effect> {
        match event {
            GameEvent::RelicObtained if !self.applied => {
                self.applied = true;
                // Apply Max HP increase
                vec![Effect::HealAndIncreaseMaxHp(10)]
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
    fn test_pear_creation() {
        let pear = PearRelic::new();
        assert!(!pear.applied);
    }

    #[test]
    fn test_pear_applies_on_pickup() {
        let mut pear = PearRelic::new();

        let effects = pear.on_game_event(&GameEvent::RelicObtained);

        assert_eq!(effects.len(), 1);
        assert!(matches!(
            effects[0],
            Effect::HealAndIncreaseMaxHp(10)
        ));
        assert!(pear.applied);
    }

    #[test]
    fn test_pear_only_applies_once() {
        let mut pear = PearRelic::new();

        // First pickup
        let effects1 = pear.on_game_event(&GameEvent::RelicObtained);
        assert_eq!(effects1.len(), 1);
        assert!(pear.applied);

        // Second pickup (shouldn't happen in game, but test safety)
        let effects2 = pear.on_game_event(&GameEvent::RelicObtained);
        assert_eq!(effects2.len(), 0);
    }
}
