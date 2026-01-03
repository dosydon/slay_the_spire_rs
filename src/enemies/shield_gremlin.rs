use crate::game::{effect::BattleEffect, enemy::EnemyTrait, global_info::GlobalInfo};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShieldGremlin {
    hp: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShieldGremlinMove {
    Protect,
    ShieldBash,
}

impl ShieldGremlin {
    pub fn new(hp: u32) -> Self {
        ShieldGremlin { hp }
    }

    /// Calculate HP range based on ascension
    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 7 {
            (13, 17)
        } else {
            (12, 15)
        }
    }

    /// Calculate Protect block amount based on ascension
    pub fn calculate_protect_block(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 17 {
            11
        } else if global_info.ascention >= 2 {
            8
        } else {
            7
        }
    }

    /// Calculate Shield Bash damage based on ascension
    pub fn calculate_shield_bash_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            8
        } else {
            6
        }
    }

    pub fn get_move_effects(&self, move_type: ShieldGremlinMove, global_info: &GlobalInfo) -> Vec<BattleEffect> {
        match move_type {
            ShieldGremlinMove::Protect => {
                let block = Self::calculate_protect_block(global_info);
                // Grant block to a random ally (handled by battle system)
                vec![BattleEffect::GainDefenseRandomAlly { amount: block }]
            }
            ShieldGremlinMove::ShieldBash => {
                let damage = Self::calculate_shield_bash_damage(global_info);
                vec![BattleEffect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 }]
            }
        }
    }
}

impl EnemyTrait for ShieldGremlin {
    type MoveType = ShieldGremlinMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));

        ShieldGremlin::new(hp)
    }

    fn get_name() -> String {
        "Shield Gremlin".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, _rng: &mut impl rand::Rng) -> (ShieldGremlinMove, Vec<BattleEffect>) {
        // Shield Gremlin uses Protect when allies are alive, Shield Bash when alone
        // This logic needs to be handled externally by checking enemy count
        // For now, default to Protect (the battle system will need to override this based on ally count)
        let move_type = ShieldGremlinMove::Protect;
        let effects = self.get_move_effects(move_type, global_info);

        (move_type, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::global_info::GlobalInfo;

    #[test]
    fn test_shield_gremlin_creation() {
        let gremlin = ShieldGremlin::new(14);
        assert_eq!(gremlin.hp, 14);
    }

    #[test]
    fn test_shield_gremlin_hp_range() {
        let global_info_low = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_high = GlobalInfo { ascention: 7, current_floor: 1 };

        assert_eq!(ShieldGremlin::calculate_hp_range(&global_info_low), (12, 15));
        assert_eq!(ShieldGremlin::calculate_hp_range(&global_info_high), (13, 17));
    }

    #[test]
    fn test_shield_gremlin_protect_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };
        let global_info_asc17 = GlobalInfo { ascention: 17, current_floor: 1 };

        assert_eq!(ShieldGremlin::calculate_protect_block(&global_info_asc0), 7);
        assert_eq!(ShieldGremlin::calculate_protect_block(&global_info_asc2), 8);
        assert_eq!(ShieldGremlin::calculate_protect_block(&global_info_asc17), 11);
    }

    #[test]
    fn test_shield_gremlin_bash_damage_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };

        assert_eq!(ShieldGremlin::calculate_shield_bash_damage(&global_info_asc0), 6);
        assert_eq!(ShieldGremlin::calculate_shield_bash_damage(&global_info_asc2), 8);
    }

    #[test]
    fn test_shield_gremlin_protect_effects() {
        let gremlin = ShieldGremlin::new(14);
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc17 = GlobalInfo { ascention: 17, current_floor: 1 };

        let effects_asc0 = gremlin.get_move_effects(ShieldGremlinMove::Protect, &global_info_asc0);
        assert_eq!(effects_asc0.len(), 1);
        assert_eq!(effects_asc0[0], BattleEffect::GainDefenseRandomAlly { amount: 7 });

        let effects_asc17 = gremlin.get_move_effects(ShieldGremlinMove::Protect, &global_info_asc17);
        assert_eq!(effects_asc17.len(), 1);
        assert_eq!(effects_asc17[0], BattleEffect::GainDefenseRandomAlly { amount: 11 });
    }

    #[test]
    fn test_shield_gremlin_bash_effects() {
        let gremlin = ShieldGremlin::new(14);
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };

        let effects_asc0 = gremlin.get_move_effects(ShieldGremlinMove::ShieldBash, &global_info_asc0);
        assert_eq!(effects_asc0.len(), 1);
        assert_eq!(effects_asc0[0], BattleEffect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 });

        let effects_asc2 = gremlin.get_move_effects(ShieldGremlinMove::ShieldBash, &global_info_asc2);
        assert_eq!(effects_asc2.len(), 1);
        assert_eq!(effects_asc2[0], BattleEffect::AttackToTarget { amount: 8, num_attacks: 1, strength_multiplier: 1 });
    }

    #[test]
    fn test_shield_gremlin_instantiate() {
        let mut rng = rand::rng();
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test multiple instantiations to ensure HP is in correct range
        for _ in 0..10 {
            let gremlin_asc0 = ShieldGremlin::instantiate(&mut rng, &global_info_asc0);
            assert!(gremlin_asc0.hp >= 12 && gremlin_asc0.hp <= 15);

            let gremlin_asc7 = ShieldGremlin::instantiate(&mut rng, &global_info_asc7);
            assert!(gremlin_asc7.hp >= 13 && gremlin_asc7.hp <= 17);
        }
    }

    #[test]
    fn test_shield_gremlin_name() {
        assert_eq!(ShieldGremlin::get_name(), "Shield Gremlin");
    }
}
