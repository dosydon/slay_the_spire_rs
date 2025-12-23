use crate::game::{effect::BattleEffect, enemy::EnemyTrait, global_info::GlobalInfo};

#[derive(Clone, Debug)]
pub struct GremlinWizard {
    hp: u32,
    charge_count: u32,
    has_used_first_blast: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GremlinWizardMove {
    Charging,
    UltimateBlast,
}

impl GremlinWizard {
    pub fn new(hp: u32) -> Self {
        GremlinWizard {
            hp,
            charge_count: 0,
            has_used_first_blast: false,
        }
    }

    /// Calculate HP range based on ascension
    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 7 {
            (22, 26)
        } else {
            (21, 25)
        }
    }

    /// Calculate Ultimate Blast damage based on ascension
    pub fn calculate_blast_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            30
        } else {
            25
        }
    }

    /// Check if should skip charging after first blast (A17+)
    pub fn should_skip_charging_after_first_blast(global_info: &GlobalInfo) -> bool {
        global_info.ascention >= 17
    }

    pub fn get_move_effects(&self, move_type: GremlinWizardMove, global_info: &GlobalInfo) -> Vec<BattleEffect> {
        match move_type {
            GremlinWizardMove::Charging => {
                // Charging does nothing
                vec![]
            }
            GremlinWizardMove::UltimateBlast => {
                let damage = Self::calculate_blast_damage(global_info);
                vec![BattleEffect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 }]
            }
        }
    }
}

impl EnemyTrait for GremlinWizard {
    type MoveType = GremlinWizardMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));

        GremlinWizard::new(hp)
    }

    fn get_name() -> String {
        "Gremlin Wizard".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, _rng: &mut impl rand::Rng) -> (GremlinWizardMove, Vec<BattleEffect>) {
        // Pattern:
        // - First cycle: Charges twice (turns 1-2), then Ultimate Blast (turn 3)
        // - Subsequent cycles (normal): Charges 3 times, then blast (repeats)
        // - Ascension 17+: After first blast, blasts every turn without charging

        let move_type;
        let is_a17_plus = Self::should_skip_charging_after_first_blast(global_info);

        if !self.has_used_first_blast {
            // First cycle: charge twice, then blast
            if self.charge_count < 2 {
                move_type = GremlinWizardMove::Charging;
                self.charge_count += 1;
            } else {
                move_type = GremlinWizardMove::UltimateBlast;
                self.charge_count = 0;
                self.has_used_first_blast = true;
            }
        } else if is_a17_plus {
            // A17+: After first blast, always blast
            move_type = GremlinWizardMove::UltimateBlast;
        } else {
            // Subsequent cycles: charge 3 times, then blast
            if self.charge_count < 3 {
                move_type = GremlinWizardMove::Charging;
                self.charge_count += 1;
            } else {
                move_type = GremlinWizardMove::UltimateBlast;
                self.charge_count = 0;
            }
        }

        let effects = self.get_move_effects(move_type, global_info);
        (move_type, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::global_info::GlobalInfo;

    #[test]
    fn test_gremlin_wizard_creation() {
        let wizard = GremlinWizard::new(23);
        assert_eq!(wizard.hp, 23);
        assert_eq!(wizard.charge_count, 0);
        assert!(!wizard.has_used_first_blast);
    }

    #[test]
    fn test_gremlin_wizard_hp_range() {
        let global_info_low = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_high = GlobalInfo { ascention: 7, current_floor: 1 };

        assert_eq!(GremlinWizard::calculate_hp_range(&global_info_low), (21, 25));
        assert_eq!(GremlinWizard::calculate_hp_range(&global_info_high), (22, 26));
    }

    #[test]
    fn test_gremlin_wizard_damage_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };

        assert_eq!(GremlinWizard::calculate_blast_damage(&global_info_asc0), 25);
        assert_eq!(GremlinWizard::calculate_blast_damage(&global_info_asc2), 30);
    }

    #[test]
    fn test_gremlin_wizard_a17_behavior() {
        let global_info_asc16 = GlobalInfo { ascention: 16, current_floor: 1 };
        let global_info_asc17 = GlobalInfo { ascention: 17, current_floor: 1 };

        assert!(!GremlinWizard::should_skip_charging_after_first_blast(&global_info_asc16));
        assert!(GremlinWizard::should_skip_charging_after_first_blast(&global_info_asc17));
    }

    #[test]
    fn test_gremlin_wizard_first_cycle_pattern() {
        let mut wizard = GremlinWizard::new(23);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();

        // Turn 1: Charging
        let (move1, effects1) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move1, GremlinWizardMove::Charging);
        assert_eq!(effects1.len(), 0);
        assert_eq!(wizard.charge_count, 1);
        assert!(!wizard.has_used_first_blast);

        // Turn 2: Charging
        let (move2, effects2) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move2, GremlinWizardMove::Charging);
        assert_eq!(effects2.len(), 0);
        assert_eq!(wizard.charge_count, 2);
        assert!(!wizard.has_used_first_blast);

        // Turn 3: Ultimate Blast
        let (move3, effects3) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move3, GremlinWizardMove::UltimateBlast);
        assert_eq!(effects3.len(), 1);
        assert_eq!(effects3[0], BattleEffect::AttackToTarget { amount: 25, num_attacks: 1, strength_multiplier: 1 });
        assert_eq!(wizard.charge_count, 0);
        assert!(wizard.has_used_first_blast);
    }

    #[test]
    fn test_gremlin_wizard_subsequent_cycle_pattern() {
        let mut wizard = GremlinWizard::new(23);
        wizard.has_used_first_blast = true;
        wizard.charge_count = 0;

        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();

        // Turn 1: Charging
        let (move1, _) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move1, GremlinWizardMove::Charging);
        assert_eq!(wizard.charge_count, 1);

        // Turn 2: Charging
        let (move2, _) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move2, GremlinWizardMove::Charging);
        assert_eq!(wizard.charge_count, 2);

        // Turn 3: Charging
        let (move3, _) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move3, GremlinWizardMove::Charging);
        assert_eq!(wizard.charge_count, 3);

        // Turn 4: Ultimate Blast
        let (move4, effects4) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move4, GremlinWizardMove::UltimateBlast);
        assert_eq!(effects4.len(), 1);
        assert_eq!(wizard.charge_count, 0);
    }

    #[test]
    fn test_gremlin_wizard_a17_pattern() {
        let mut wizard = GremlinWizard::new(23);
        let global_info = GlobalInfo { ascention: 17, current_floor: 1 };
        let mut rng = rand::rng();

        // First cycle: Charge twice, then blast
        let (move1, _) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move1, GremlinWizardMove::Charging);

        let (move2, _) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move2, GremlinWizardMove::Charging);

        let (move3, _) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move3, GremlinWizardMove::UltimateBlast);
        assert!(wizard.has_used_first_blast);

        // After first blast at A17+: Always blast
        let (move4, _) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move4, GremlinWizardMove::UltimateBlast);

        let (move5, _) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move5, GremlinWizardMove::UltimateBlast);

        let (move6, _) = wizard.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move6, GremlinWizardMove::UltimateBlast);
    }

    #[test]
    fn test_gremlin_wizard_move_effects() {
        let wizard = GremlinWizard::new(23);
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };

        // Charging does nothing
        let charging_effects = wizard.get_move_effects(GremlinWizardMove::Charging, &global_info_asc0);
        assert_eq!(charging_effects.len(), 0);

        // Ultimate Blast damage
        let blast_effects_asc0 = wizard.get_move_effects(GremlinWizardMove::UltimateBlast, &global_info_asc0);
        assert_eq!(blast_effects_asc0.len(), 1);
        assert_eq!(blast_effects_asc0[0], BattleEffect::AttackToTarget { amount: 25, num_attacks: 1, strength_multiplier: 1 });

        let blast_effects_asc2 = wizard.get_move_effects(GremlinWizardMove::UltimateBlast, &global_info_asc2);
        assert_eq!(blast_effects_asc2.len(), 1);
        assert_eq!(blast_effects_asc2[0], BattleEffect::AttackToTarget { amount: 30, num_attacks: 1, strength_multiplier: 1 });
    }

    #[test]
    fn test_gremlin_wizard_instantiate() {
        let mut rng = rand::rng();
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test multiple instantiations to ensure HP is in correct range
        for _ in 0..10 {
            let wizard_asc0 = GremlinWizard::instantiate(&mut rng, &global_info_asc0);
            assert!(wizard_asc0.hp >= 21 && wizard_asc0.hp <= 25);

            let wizard_asc7 = GremlinWizard::instantiate(&mut rng, &global_info_asc7);
            assert!(wizard_asc7.hp >= 22 && wizard_asc7.hp <= 26);
        }
    }

    #[test]
    fn test_gremlin_wizard_name() {
        assert_eq!(GremlinWizard::get_name(), "Gremlin Wizard");
    }
}
