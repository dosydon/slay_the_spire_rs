use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;

/// Lantern - Gain 1 Energy on the first turn of each combat
pub struct LanternRelic {
    used: bool,
    owner: Entity,
}

impl LanternRelic {
    pub fn new(owner: Entity) -> Self {
        Self {
            used: false,
            owner,
        }
    }
}

impl EventListener for LanternRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::StartOfPlayerTurn if !self.used => {
                self.used = true;
                vec![BattleEffect::GainEnergy { amount: 1 }]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lantern_creation() {
        let lantern = LanternRelic::new(Entity::Player);
        assert!(!lantern.used);
    }

    #[test]
    fn test_lantern_trigger_first_turn() {
        let mut lantern = LanternRelic::new(Entity::Player);

        // First turn start - should trigger
        let effects = lantern.on_event(&BattleEvent::StartOfPlayerTurn);

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::GainEnergy { amount: 1 }));
        assert!(lantern.used);
    }

    #[test]
    fn test_lantern_only_triggers_once() {
        let mut lantern = LanternRelic::new(Entity::Player);

        // First turn start - should trigger
        let effects1 = lantern.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects1.len(), 1);
        assert!(lantern.used);

        // Second turn start - should not trigger
        let effects2 = lantern.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects2.len(), 0);
    }
}
