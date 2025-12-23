use crate::game::{effect::BattleEffect, enemy::EnemyTrait, global_info::GlobalInfo};

#[derive(Clone, Debug)]
pub struct SneakyGremlin {
    hp: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SneakyGremlinMove {
    Puncture,
}

impl SneakyGremlin {
    pub fn new(hp: u32) -> Self {
        SneakyGremlin { hp }
    }

    /// Calculate HP range based on ascension
    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 7 {
            (11, 15)
        } else {
            (10, 14)
        }
    }

    /// Calculate Puncture damage based on ascension
    pub fn calculate_puncture_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            10
        } else {
            9
        }
    }

    pub fn get_move_effects(&self, _move_type: SneakyGremlinMove, global_info: &GlobalInfo) -> Vec<BattleEffect> {
        let damage = Self::calculate_puncture_damage(global_info);
        vec![BattleEffect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 }]
    }
}

impl EnemyTrait for SneakyGremlin {
    type MoveType = SneakyGremlinMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));

        SneakyGremlin::new(hp)
    }

    fn get_name() -> String {
        "Sneaky Gremlin".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, _rng: &mut impl rand::Rng) -> (SneakyGremlinMove, Vec<BattleEffect>) {
        // Sneaky Gremlin always uses Puncture
        let move_type = SneakyGremlinMove::Puncture;
        let effects = self.get_move_effects(move_type, global_info);

        (move_type, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::global_info::GlobalInfo;

    #[test]
    fn test_sneaky_gremlin_creation() {
        let gremlin = SneakyGremlin::new(12);
        assert_eq!(gremlin.hp, 12);
    }

    #[test]
    fn test_sneaky_gremlin_hp_range() {
        let global_info_low = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_high = GlobalInfo { ascention: 7, current_floor: 1 };

        assert_eq!(SneakyGremlin::calculate_hp_range(&global_info_low), (10, 14));
        assert_eq!(SneakyGremlin::calculate_hp_range(&global_info_high), (11, 15));
    }

    #[test]
    fn test_sneaky_gremlin_damage_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };

        assert_eq!(SneakyGremlin::calculate_puncture_damage(&global_info_asc0), 9);
        assert_eq!(SneakyGremlin::calculate_puncture_damage(&global_info_asc2), 10);
    }

    #[test]
    fn test_sneaky_gremlin_move_effects() {
        let gremlin = SneakyGremlin::new(12);
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };

        let effects_asc0 = gremlin.get_move_effects(SneakyGremlinMove::Puncture, &global_info_asc0);
        assert_eq!(effects_asc0.len(), 1);
        assert_eq!(effects_asc0[0], BattleEffect::AttackToTarget { amount: 9, num_attacks: 1, strength_multiplier: 1 });

        let effects_asc2 = gremlin.get_move_effects(SneakyGremlinMove::Puncture, &global_info_asc2);
        assert_eq!(effects_asc2.len(), 1);
        assert_eq!(effects_asc2[0], BattleEffect::AttackToTarget { amount: 10, num_attacks: 1, strength_multiplier: 1 });
    }

    #[test]
    fn test_sneaky_gremlin_always_uses_puncture() {
        let mut gremlin = SneakyGremlin::new(12);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();

        // Test multiple turns to ensure it always uses Puncture
        for _ in 0..10 {
            let (move_type, _effects) = gremlin.choose_move_and_effects(&global_info, &mut rng);
            assert_eq!(move_type, SneakyGremlinMove::Puncture);
        }
    }

    #[test]
    fn test_sneaky_gremlin_instantiate() {
        let mut rng = rand::rng();
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test multiple instantiations to ensure HP is in correct range
        for _ in 0..10 {
            let gremlin_asc0 = SneakyGremlin::instantiate(&mut rng, &global_info_asc0);
            assert!(gremlin_asc0.hp >= 10 && gremlin_asc0.hp <= 14);

            let gremlin_asc7 = SneakyGremlin::instantiate(&mut rng, &global_info_asc7);
            assert!(gremlin_asc7.hp >= 11 && gremlin_asc7.hp <= 15);
        }
    }

    #[test]
    fn test_sneaky_gremlin_name() {
        assert_eq!(SneakyGremlin::get_name(), "Sneaky Gremlin");
    }
}
