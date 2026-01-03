use crate::game::{effect::BattleEffect, enemy::EnemyTrait, global_info::GlobalInfo};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FatGremlin {
    hp: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FatGremlinMove {
    Smash,
}

impl FatGremlin {
    pub fn new(hp: u32) -> Self {
        FatGremlin { hp }
    }

    /// Calculate HP range based on ascension
    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 7 {
            (14, 18)
        } else {
            (13, 17)
        }
    }

    /// Calculate Smash damage based on ascension
    pub fn calculate_smash_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            5
        } else {
            4
        }
    }

    /// Check if should apply Frail based on ascension
    pub fn should_apply_frail(global_info: &GlobalInfo) -> bool {
        global_info.ascention >= 17
    }

    pub fn get_move_effects(&self, _move_type: FatGremlinMove, global_info: &GlobalInfo) -> Vec<BattleEffect> {
        let damage = Self::calculate_smash_damage(global_info);
        let mut effects = vec![
            BattleEffect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 },
            BattleEffect::ApplyWeak { duration: 1 },
        ];

        if Self::should_apply_frail(global_info) {
            effects.push(BattleEffect::ApplyFrail { duration: 1 });
        }

        effects
    }
}

impl EnemyTrait for FatGremlin {
    type MoveType = FatGremlinMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));

        FatGremlin::new(hp)
    }

    fn get_name() -> String {
        "Fat Gremlin".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, _rng: &mut impl rand::Rng) -> (FatGremlinMove, Vec<BattleEffect>) {
        // Fat Gremlin always uses Smash
        let move_type = FatGremlinMove::Smash;
        let effects = self.get_move_effects(move_type, global_info);

        (move_type, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::global_info::GlobalInfo;

    #[test]
    fn test_fat_gremlin_creation() {
        let gremlin = FatGremlin::new(15);
        assert_eq!(gremlin.hp, 15);
    }

    #[test]
    fn test_fat_gremlin_hp_range() {
        let global_info_low = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_high = GlobalInfo { ascention: 7, current_floor: 1 };

        assert_eq!(FatGremlin::calculate_hp_range(&global_info_low), (13, 17));
        assert_eq!(FatGremlin::calculate_hp_range(&global_info_high), (14, 18));
    }

    #[test]
    fn test_fat_gremlin_damage_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };

        assert_eq!(FatGremlin::calculate_smash_damage(&global_info_asc0), 4);
        assert_eq!(FatGremlin::calculate_smash_damage(&global_info_asc2), 5);
    }

    #[test]
    fn test_fat_gremlin_frail_ascension() {
        let global_info_asc16 = GlobalInfo { ascention: 16, current_floor: 1 };
        let global_info_asc17 = GlobalInfo { ascention: 17, current_floor: 1 };

        assert!(!FatGremlin::should_apply_frail(&global_info_asc16));
        assert!(FatGremlin::should_apply_frail(&global_info_asc17));
    }

    #[test]
    fn test_fat_gremlin_move_effects() {
        let gremlin = FatGremlin::new(15);
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc17 = GlobalInfo { ascention: 17, current_floor: 1 };

        // Base effects
        let effects_asc0 = gremlin.get_move_effects(FatGremlinMove::Smash, &global_info_asc0);
        assert_eq!(effects_asc0.len(), 2);
        assert_eq!(effects_asc0[0], BattleEffect::AttackToTarget { amount: 4, num_attacks: 1, strength_multiplier: 1 });
        assert_eq!(effects_asc0[1], BattleEffect::ApplyWeak { duration: 1 });

        // With Frail at A17+
        let effects_asc17 = gremlin.get_move_effects(FatGremlinMove::Smash, &global_info_asc17);
        assert_eq!(effects_asc17.len(), 3);
        assert_eq!(effects_asc17[0], BattleEffect::AttackToTarget { amount: 5, num_attacks: 1, strength_multiplier: 1 });
        assert_eq!(effects_asc17[1], BattleEffect::ApplyWeak { duration: 1 });
        assert_eq!(effects_asc17[2], BattleEffect::ApplyFrail { duration: 1 });
    }

    #[test]
    fn test_fat_gremlin_always_uses_smash() {
        let mut gremlin = FatGremlin::new(15);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();

        // Test multiple turns to ensure it always uses Smash
        for _ in 0..10 {
            let (move_type, _effects) = gremlin.choose_move_and_effects(&global_info, &mut rng);
            assert_eq!(move_type, FatGremlinMove::Smash);
        }
    }

    #[test]
    fn test_fat_gremlin_instantiate() {
        let mut rng = rand::rng();
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test multiple instantiations to ensure HP is in correct range
        for _ in 0..10 {
            let gremlin_asc0 = FatGremlin::instantiate(&mut rng, &global_info_asc0);
            assert!(gremlin_asc0.hp >= 13 && gremlin_asc0.hp <= 17);

            let gremlin_asc7 = FatGremlin::instantiate(&mut rng, &global_info_asc7);
            assert!(gremlin_asc7.hp >= 14 && gremlin_asc7.hp <= 18);
        }
    }

    #[test]
    fn test_fat_gremlin_name() {
        assert_eq!(FatGremlin::get_name(), "Fat Gremlin");
    }
}
