use serde::{Serialize, Deserialize};
use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;

/// Bronze Scales - Whenever you take damage, deal 3 damage back (Thorns)
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BronzeScalesRelic {
    owner: Entity,
}

impl BronzeScalesRelic {
    pub fn new(owner: Entity) -> Self {
        Self { owner }
    }
}

impl EventListener for BronzeScalesRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::DamageTaken { target, amount, .. } if *target == self.owner && *amount > 0 => {
                // Deal 3 damage back to the source
                vec![BattleEffect::AttackToTarget {
                    amount: 3,
                    num_attacks: 1,
                    strength_multiplier: 0,
                }]
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
    fn test_bronze_scales_creation() {
        let scales = BronzeScalesRelic::new(Entity::Player);
        assert_eq!(scales.owner, Entity::Player);
    }

    #[test]
    fn test_bronze_scales_deals_damage_when_player_takes_damage() {
        let mut scales = BronzeScalesRelic::new(Entity::Player);

        let effects = scales.on_event(&BattleEvent::DamageTaken {
            target: Entity::Player,
            amount: 5,
            source: Entity::Enemy(0),
        });

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::AttackToTarget { amount: 3, num_attacks: 1, strength_multiplier: 0 }));
    }

    #[test]
    fn test_bronze_scales_no_trigger_when_zero_damage() {
        let mut scales = BronzeScalesRelic::new(Entity::Player);

        let effects = scales.on_event(&BattleEvent::DamageTaken {
            target: Entity::Player,
            amount: 0,
            source: Entity::Enemy(0),
        });

        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_bronze_scales_no_trigger_when_enemy_takes_damage() {
        let mut scales = BronzeScalesRelic::new(Entity::Player);

        let effects = scales.on_event(&BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 5,
            source: Entity::Player,
        });

        assert_eq!(effects.len(), 0);
    }
}
