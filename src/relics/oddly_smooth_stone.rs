use serde::{Serialize, Deserialize};
use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;

/// Oddly Smooth Stone - At the start of each combat, gain 1 Dexterity
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct OddlySmoothStoneRelic {
    owner: Entity,
}

impl OddlySmoothStoneRelic {
    pub fn new(owner: Entity) -> Self {
        Self { owner }
    }
}

impl EventListener for OddlySmoothStoneRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CombatStart { player } if *player == self.owner => {
                vec![BattleEffect::GainDexterity { amount: 1 }]
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

    fn hash_to(&self, state: &mut std::collections::hash_map::DefaultHasher) {
        use std::hash::Hash;
        self.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oddly_smooth_stone_creation() {
        let stone = OddlySmoothStoneRelic::new(Entity::Player);
        assert_eq!(stone.owner, Entity::Player);
    }

    #[test]
    fn test_oddly_smooth_stone_gains_dexterity_on_combat_start() {
        let mut stone = OddlySmoothStoneRelic::new(Entity::Player);

        let effects = stone.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::GainDexterity { amount: 1 }));
    }

    #[test]
    fn test_oddly_smooth_stone_no_trigger_for_enemy() {
        let mut stone = OddlySmoothStoneRelic::new(Entity::Player);

        let effects = stone.on_event(&BattleEvent::CombatStart {
            player: Entity::Enemy(0),
        });

        assert_eq!(effects.len(), 0);
    }
}
