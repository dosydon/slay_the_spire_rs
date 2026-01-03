use serde::{Serialize, Deserialize};
use crate::game::{card::Card, effect::{BattleEffect, Condition}, card_type::CardType, card_enum::CardEnum, card::{Rarity, CardClass}};
use crate::battle::{battle_events::{BattleEvent, EventListener}, target::Entity};

/// Flame Barrier Listener
/// When player takes damage from enemy attacks this turn, deal damage back
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlameBarrierListener {
    owner: Entity,
    damage_to_deal: u32,
    is_active: bool,
}

impl FlameBarrierListener {
    pub fn new(owner: Entity, damage_to_deal: u32) -> Self {
        FlameBarrierListener {
            owner,
            damage_to_deal,
            is_active: true,
        }
    }
}

impl EventListener for FlameBarrierListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::DamageTaken { target, source, amount: _ }
                if *target == self.owner && self.is_active && matches!(source, Entity::Enemy(_)) => {
                // When player takes damage from enemy, retaliate with damage
                vec![BattleEffect::AttackAllEnemies {
                    amount: self.damage_to_deal,
                    num_attacks: 1,
                }]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        self.is_active
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

/// Flame Barrier - Skill Card
/// Cost: 2 (2 when upgraded)
/// Effect: Gain 12 Block. This turn, when attacked, deal 4 damage to the attacker.
pub fn flame_barrier() -> Card {
    Card::new(CardEnum::FlameBarrier, 2, CardClass::IronClad(Rarity::Uncommon, CardType::Skill), vec![
        BattleEffect::GainDefense { amount: 12 },
        BattleEffect::ActivateFlameBarrier { damage: 4 },
    ])
        .set_play_condition(Condition::True)
}

pub fn flame_barrier_upgraded() -> Card {
    Card::new(CardEnum::FlameBarrier, 2, CardClass::IronClad(Rarity::Uncommon, CardType::Skill), vec![
        BattleEffect::GainDefense { amount: 16 },
        BattleEffect::ActivateFlameBarrier { damage: 6 },
    ])
        .set_upgraded(true)
        .set_play_condition(Condition::True)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flame_barrier_creation() {
        let card = flame_barrier();

        assert_eq!(card.get_name(), "Flame Barrier");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_flame_barrier_upgraded_creation() {
        let card = flame_barrier_upgraded();

        assert_eq!(card.get_name(), "Flame Barrier+");
        assert_eq!(card.get_cost(), 2);  // Cost stays the same
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_flame_barrier_listener_creation() {
        let listener = FlameBarrierListener::new(Entity::Player, 4);
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Player);
    }

    #[test]
    fn test_flame_barrier_triggers_on_enemy_attack() {
        let mut listener = FlameBarrierListener::new(Entity::Player, 4);

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Player,
            source: Entity::Enemy(0),
            amount: 10,
        };

        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], BattleEffect::AttackAllEnemies { amount: 4, num_attacks: 1 });
        assert!(listener.is_active()); // Still active after triggering
    }

    #[test]
    fn test_flame_barrier_does_not_trigger_on_self_damage() {
        let mut listener = FlameBarrierListener::new(Entity::Player, 4);

        let self_damage_event = BattleEvent::DamageTaken {
            target: Entity::Player,
            source: Entity::Player,
            amount: 5,
        };

        let effects = listener.on_event(&self_damage_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active());
    }

    #[test]
    fn test_flame_barrier_does_not_trigger_on_enemy_damage() {
        let mut listener = FlameBarrierListener::new(Entity::Player, 4);

        let enemy_damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            source: Entity::Player,
            amount: 8,
        };

        let effects = listener.on_event(&enemy_damage_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active());
    }

    #[test]
    fn test_flame_barrier_damage_amounts() {
        let mut base_listener = FlameBarrierListener::new(Entity::Player, 4);
        let mut upgraded_listener = FlameBarrierListener::new(Entity::Player, 6);

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Player,
            source: Entity::Enemy(0),
            amount: 10,
        };

        let base_effects = base_listener.on_event(&damage_event);
        let upgraded_effects = upgraded_listener.on_event(&damage_event);

        assert_eq!(base_effects.len(), 1);
        assert_eq!(base_effects[0], BattleEffect::AttackAllEnemies { amount: 4, num_attacks: 1 });

        assert_eq!(upgraded_effects.len(), 1);
        assert_eq!(upgraded_effects[0], BattleEffect::AttackAllEnemies { amount: 6, num_attacks: 1 });
    }

    #[test]
    fn test_flame_barrier_only_triggers_for_owner() {
        let mut listener = FlameBarrierListener::new(Entity::Player, 4);

        // Enemy taking damage should not trigger
        let enemy_damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            source: Entity::Enemy(1),
            amount: 5,
        };

        let effects = listener.on_event(&enemy_damage_event);
        assert_eq!(effects.len(), 0);

        // Player taking damage from enemy should trigger
        let player_damage_event = BattleEvent::DamageTaken {
            target: Entity::Player,
            source: Entity::Enemy(0),
            amount: 8,
        };

        let effects = listener.on_event(&player_damage_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], BattleEffect::AttackAllEnemies { amount: 4, num_attacks: 1 });
    }

    #[test]
    fn test_flame_barrier_card_effects() {
        let card = flame_barrier();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0], BattleEffect::GainDefense { amount: 12 });
        assert_eq!(effects[1], BattleEffect::ActivateFlameBarrier { damage: 4 });
    }

    #[test]
    fn test_flame_barrier_upgraded_effects() {
        let card = flame_barrier_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0], BattleEffect::GainDefense { amount: 16 });
        assert_eq!(effects[1], BattleEffect::ActivateFlameBarrier { damage: 6 });
    }

    #[test]
    fn test_flame_barrier_cost_stays_same() {
        let flame_barrier_card = flame_barrier();
        assert_eq!(flame_barrier_card.get_cost(), 2, "Flame Barrier should cost 2 energy");

        let flame_barrier_plus = flame_barrier_upgraded();
        assert_eq!(flame_barrier_plus.get_cost(), 2, "Flame Barrier+ should also cost 2 energy");
    }

    #[test]
    fn test_flame_barrier_block_amounts() {
        let base_card = flame_barrier();
        let upgraded_card = flame_barrier_upgraded();

        let base_effects = base_card.get_effects();
        let upgraded_effects = upgraded_card.get_effects();

        // Check block amounts
        assert_eq!(base_effects[0], BattleEffect::GainDefense { amount: 12 });
        assert_eq!(upgraded_effects[0], BattleEffect::GainDefense { amount: 16 });

        // Check retaliation damage amounts
        assert_eq!(base_effects[1], BattleEffect::ActivateFlameBarrier { damage: 4 });
        assert_eq!(upgraded_effects[1], BattleEffect::ActivateFlameBarrier { damage: 6 });
    }
}