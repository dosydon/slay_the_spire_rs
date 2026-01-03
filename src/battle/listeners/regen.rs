use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;
use serde::{Serialize, Deserialize};

/// Regeneration - heals HP at the end of each turn, decreasing by 1 each turn
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegenListener {
    current_regen: u32,
    owner: Entity,
}

impl RegenListener {
    pub fn new(amount: u32, owner: Entity) -> Self {
        Self {
            current_regen: amount,
            owner,
        }
    }
}

impl EventListener for RegenListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::EndOfTurn { entity } if self.current_regen > 0 && *entity == self.owner => {
                let heal_amount = self.current_regen;
                self.current_regen = self.current_regen.saturating_sub(1);
                vec![BattleEffect::Heal(heal_amount)]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        // Listener remains active until regen reaches 0
        self.current_regen > 0
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn hash_to(&self, state: &mut std::collections::hash_map::DefaultHasher) {
        use std::hash::Hash;
        self.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regen_creation() {
        let regen = RegenListener::new(5, Entity::Player);
        assert_eq!(regen.current_regen, 5);
        assert!(regen.is_active());
    }

    #[test]
    fn test_regen_heals_and_decreases() {
        let mut regen = RegenListener::new(3, Entity::Player);

        // First end of turn - heal 3
        let effects1 = regen.on_event(&BattleEvent::EndOfTurn { entity: Entity::Player });
        assert_eq!(effects1.len(), 1);
        assert!(matches!(effects1[0], BattleEffect::Heal(3)));
        assert_eq!(regen.current_regen, 2);
        assert!(regen.is_active());

        // Second end of turn - heal 2
        let effects2 = regen.on_event(&BattleEvent::EndOfTurn { entity: Entity::Player });
        assert_eq!(effects2.len(), 1);
        assert!(matches!(effects2[0], BattleEffect::Heal(2)));
        assert_eq!(regen.current_regen, 1);
        assert!(regen.is_active());

        // Third end of turn - heal 1
        let effects3 = regen.on_event(&BattleEvent::EndOfTurn { entity: Entity::Player });
        assert_eq!(effects3.len(), 1);
        assert!(matches!(effects3[0], BattleEffect::Heal(1)));
        assert_eq!(regen.current_regen, 0);
        assert!(!regen.is_active()); // Inactive after reaching 0
    }

    #[test]
    fn test_regn_ignores_other_events() {
        let mut regen = RegenListener::new(5, Entity::Player);

        let other_events = vec![
            BattleEvent::StartOfPlayerTurn,
            BattleEvent::EndOfTurn { entity: Entity::Enemy(0) }, // Enemy end of turn
        ];

        for event in other_events {
            let effects = regen.on_event(&event);
            assert_eq!(effects.len(), 0);
            assert_eq!(regen.current_regen, 5); // Unchanged
        }
    }

    #[test]
    fn test_regen_inactive_at_zero() {
        let mut regen = RegenListener::new(1, Entity::Player);

        // First end of turn triggers
        let effects1 = regen.on_event(&BattleEvent::EndOfTurn { entity: Entity::Player });
        assert_eq!(effects1.len(), 1);
        assert!(!regen.is_active());

        // Second end of turn should not trigger (regen is 0)
        let effects2 = regen.on_event(&BattleEvent::EndOfTurn { entity: Entity::Player });
        assert_eq!(effects2.len(), 0);
    }
}
