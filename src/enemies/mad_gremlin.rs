use crate::game::{effect::BattleEffect, enemy::EnemyTrait, global_info::GlobalInfo};
use crate::battle::{battle_events::{BattleEvent, EventListener}, target::Entity};

#[derive(Clone, Debug)]
pub struct MadGremlin {
    hp: u32,
    angry_stacks: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MadGremlinMove {
    Scratch,
}

impl MadGremlin {
    pub fn new(hp: u32, angry_stacks: u32) -> Self {
        MadGremlin { hp, angry_stacks }
    }

    /// Calculate HP range based on ascension
    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 7 {
            (21, 25)
        } else {
            (20, 24)
        }
    }

    /// Calculate Scratch damage based on ascension
    pub fn calculate_scratch_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            5
        } else {
            4
        }
    }

    /// Calculate Angry stacks based on ascension
    pub fn calculate_angry_stacks(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 17 {
            2
        } else {
            1
        }
    }

    pub fn get_move_effects(&self, _move_type: MadGremlinMove, global_info: &GlobalInfo) -> Vec<BattleEffect> {
        let damage = Self::calculate_scratch_damage(global_info);
        vec![BattleEffect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 }]
    }

    pub fn get_angry_stacks(&self) -> u32 {
        self.angry_stacks
    }
}

impl EnemyTrait for MadGremlin {
    type MoveType = MadGremlinMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));
        let angry_stacks = Self::calculate_angry_stacks(global_info);

        MadGremlin::new(hp, angry_stacks)
    }

    fn get_name() -> String {
        "Mad Gremlin".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, _rng: &mut impl rand::Rng) -> (MadGremlinMove, Vec<BattleEffect>) {
        // Mad Gremlin always uses Scratch
        let move_type = MadGremlinMove::Scratch;
        let effects = self.get_move_effects(move_type, global_info);

        (move_type, effects)
    }
}

// AngryListener implementation for MadGremlin
// Whenever this creature takes damage, it gains Strength
#[derive(Debug)]
pub struct AngryListener {
    angry_amount: u32,
    owner: Entity,
}

impl AngryListener {
    pub fn new(owner: Entity, angry_amount: u32) -> Self {
        AngryListener {
            angry_amount,
            owner,
        }
    }
}

impl EventListener for AngryListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::DamageTaken { target, amount, source: _ } if *target == self.owner && *amount > 0 => {
                // When the Mad Gremlin takes damage, it gains Strength
                vec![BattleEffect::GainStrength { amount: self.angry_amount }]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true // Angry is always active
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
    use crate::game::global_info::GlobalInfo;

    #[test]
    fn test_mad_gremlin_creation() {
        let gremlin = MadGremlin::new(22, 1);
        assert_eq!(gremlin.hp, 22);
        assert_eq!(gremlin.angry_stacks, 1);
    }

    #[test]
    fn test_mad_gremlin_hp_range() {
        let global_info_low = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_high = GlobalInfo { ascention: 7, current_floor: 1 };

        assert_eq!(MadGremlin::calculate_hp_range(&global_info_low), (20, 24));
        assert_eq!(MadGremlin::calculate_hp_range(&global_info_high), (21, 25));
    }

    #[test]
    fn test_mad_gremlin_damage_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };

        assert_eq!(MadGremlin::calculate_scratch_damage(&global_info_asc0), 4);
        assert_eq!(MadGremlin::calculate_scratch_damage(&global_info_asc2), 5);
    }

    #[test]
    fn test_mad_gremlin_angry_scaling() {
        let global_info_asc16 = GlobalInfo { ascention: 16, current_floor: 1 };
        let global_info_asc17 = GlobalInfo { ascention: 17, current_floor: 1 };

        assert_eq!(MadGremlin::calculate_angry_stacks(&global_info_asc16), 1);
        assert_eq!(MadGremlin::calculate_angry_stacks(&global_info_asc17), 2);
    }

    #[test]
    fn test_mad_gremlin_move_effects() {
        let gremlin = MadGremlin::new(22, 1);
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };

        let effects_asc0 = gremlin.get_move_effects(MadGremlinMove::Scratch, &global_info_asc0);
        assert_eq!(effects_asc0.len(), 1);
        assert_eq!(effects_asc0[0], BattleEffect::AttackToTarget { amount: 4, num_attacks: 1, strength_multiplier: 1 });

        let effects_asc2 = gremlin.get_move_effects(MadGremlinMove::Scratch, &global_info_asc2);
        assert_eq!(effects_asc2.len(), 1);
        assert_eq!(effects_asc2[0], BattleEffect::AttackToTarget { amount: 5, num_attacks: 1, strength_multiplier: 1 });
    }

    #[test]
    fn test_mad_gremlin_always_uses_scratch() {
        let mut gremlin = MadGremlin::new(22, 1);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();

        // Test multiple turns to ensure it always uses Scratch
        for _ in 0..10 {
            let (move_type, _effects) = gremlin.choose_move_and_effects(&global_info, &mut rng);
            assert_eq!(move_type, MadGremlinMove::Scratch);
        }
    }

    #[test]
    fn test_mad_gremlin_instantiate() {
        let mut rng = rand::rng();
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };
        let global_info_asc17 = GlobalInfo { ascention: 17, current_floor: 1 };

        // Test multiple instantiations to ensure HP is in correct range
        for _ in 0..10 {
            let gremlin_asc0 = MadGremlin::instantiate(&mut rng, &global_info_asc0);
            assert!(gremlin_asc0.hp >= 20 && gremlin_asc0.hp <= 24);
            assert_eq!(gremlin_asc0.angry_stacks, 1);

            let gremlin_asc7 = MadGremlin::instantiate(&mut rng, &global_info_asc7);
            assert!(gremlin_asc7.hp >= 21 && gremlin_asc7.hp <= 25);
            assert_eq!(gremlin_asc7.angry_stacks, 1);

            let gremlin_asc17 = MadGremlin::instantiate(&mut rng, &global_info_asc17);
            assert!(gremlin_asc17.hp >= 21 && gremlin_asc17.hp <= 25);
            assert_eq!(gremlin_asc17.angry_stacks, 2);
        }
    }

    #[test]
    fn test_mad_gremlin_name() {
        assert_eq!(MadGremlin::get_name(), "Mad Gremlin");
    }

    #[test]
    fn test_angry_listener_creation() {
        let listener = AngryListener::new(Entity::Enemy(0), 1);
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Enemy(0));
        assert_eq!(listener.angry_amount, 1);
    }

    #[test]
    fn test_angry_triggers_on_damage() {
        let mut listener = AngryListener::new(Entity::Enemy(0), 1);

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 6,
            source: Entity::Player,
        };

        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], BattleEffect::GainStrength { amount: 1 });
        assert!(listener.is_active());
    }

    #[test]
    fn test_angry_does_not_trigger_on_zero_damage() {
        let mut listener = AngryListener::new(Entity::Enemy(0), 1);

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 0,
            source: Entity::Player,
        };

        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_angry_does_not_trigger_for_other_enemies() {
        let mut listener = AngryListener::new(Entity::Enemy(0), 1);

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(1), // Different enemy
            amount: 6,
            source: Entity::Player,
        };

        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_angry_triggers_multiple_times() {
        let mut listener = AngryListener::new(Entity::Enemy(0), 2);

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 5,
            source: Entity::Player,
        };

        // First damage
        let effects1 = listener.on_event(&damage_event);
        assert_eq!(effects1.len(), 1);
        assert_eq!(effects1[0], BattleEffect::GainStrength { amount: 2 });

        // Second damage should also trigger
        let effects2 = listener.on_event(&damage_event);
        assert_eq!(effects2.len(), 1);
        assert_eq!(effects2[0], BattleEffect::GainStrength { amount: 2 });

        assert!(listener.is_active());
    }

    #[test]
    fn test_angry_different_amounts() {
        let mut listener_1 = AngryListener::new(Entity::Enemy(0), 1);
        let mut listener_2 = AngryListener::new(Entity::Enemy(1), 2);

        let damage_event_0 = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 5,
            source: Entity::Player,
        };

        let damage_event_1 = BattleEvent::DamageTaken {
            target: Entity::Enemy(1),
            amount: 5,
            source: Entity::Player,
        };

        let effects_1 = listener_1.on_event(&damage_event_0);
        assert_eq!(effects_1, vec![BattleEffect::GainStrength { amount: 1 }]);

        let effects_2 = listener_2.on_event(&damage_event_1);
        assert_eq!(effects_2, vec![BattleEffect::GainStrength { amount: 2 }]);
    }
}
