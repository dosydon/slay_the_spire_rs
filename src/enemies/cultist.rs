use crate::{game::{effect::Effect, enemy::EnemyTrait, global_info::GlobalInfo}, utils::CategoricalDistribution};

#[derive(Clone)]
pub struct Cultist {
    base_damage: u32,
    hp: u32,
    ritual_amount: u32,
    has_used_incantation: bool,
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum CultistMove {
    Incantation,
    DarkStrike,
}

impl Cultist {
    pub fn new(hp: u32, ritual_amount: u32) -> Self {
        Cultist {
            base_damage: 6, // Dark Strike always does 6 damage
            hp,
            ritual_amount,
            has_used_incantation: false,
        }
    }

    pub fn calculate_ritual_amount(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 17 {
            5
        } else if global_info.ascention >= 2 {
            4
        } else {
            3
        }
    }

    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 7 {
            (50, 56)
        } else {
            (48, 54)
        }
    }

    fn get_valid_moves(&self) -> Vec<CultistMove> {
        if !self.has_used_incantation {
            // Must use Incantation first
            vec![CultistMove::Incantation]
        } else {
            // Only Dark Strike after Incantation
            vec![CultistMove::DarkStrike]
        }
    }

    pub fn record_move(&mut self, move_type: CultistMove) {
        match move_type {
            CultistMove::Incantation => {
                self.has_used_incantation = true;
            }
            CultistMove::DarkStrike => {
                // No special tracking needed for Dark Strike
            }
        }
    }

    pub fn get_move_effects(&self, move_type: CultistMove) -> Vec<Effect> {
        match move_type {
            CultistMove::Incantation => {
                // Gain Ritual (which grants Strength every turn)
                vec![Effect::GainRitual(self.ritual_amount)]
            }
            CultistMove::DarkStrike => {
                // Deal 6 damage
                vec![Effect::AttackToTarget { amount: self.base_damage, num_attacks: 1 }]
            }
        }
    }

    /// Choose a move and return both the move and its effects
    pub fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (CultistMove, Vec<Effect>) {
        let move_distribution = self.choose_next_move(global_info);
        let selected_move = move_distribution.sample_owned(rng);
        
        // Record the move for move tracking
        self.record_move(selected_move);
        
        // Generate the effects for this move
        let effects = self.get_move_effects(selected_move);
        
        (selected_move, effects)
    }

    /// Choose effects directly, sampling a move and recording it
    pub fn choose_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> Vec<Effect> {
        let (_move, effects) = self.choose_move_and_effects(global_info, rng);
        effects
    }
}

impl EnemyTrait for Cultist {
    type MoveType = CultistMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));
        let ritual_amount = Self::calculate_ritual_amount(global_info);
        
        Cultist::new(hp, ritual_amount)
    }

    fn hp_lb() -> u32 {
        48
    }

    fn hp_ub() -> u32 {
        54
    }

    fn choose_next_move(&self, _global_info: &GlobalInfo) -> CategoricalDistribution<Self::MoveType> {
        let valid_moves = self.get_valid_moves();
        
        // Cultist has deterministic behavior - no randomness needed
        let outcomes_and_weights: Vec<(CultistMove, f64)> = valid_moves
            .into_iter()
            .map(|move_type| (move_type, 1.0))
            .collect();
        
        CategoricalDistribution::new(outcomes_and_weights)
    }

    fn get_name() -> String {
        "Cultist".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::global_info::GlobalInfo;

    #[test]
    fn test_cultist_creation() {
        let cultist = Cultist::new(50, 3);
        assert_eq!(cultist.hp, 50);
        assert_eq!(cultist.ritual_amount, 3);
        assert!(!cultist.has_used_incantation);
    }

    #[test]
    fn test_cultist_move_pattern() {
        let mut cultist = Cultist::new(50, 3);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();

        // First move should be Incantation
        let (first_move, _) = cultist.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(first_move, CultistMove::Incantation);

        // All subsequent moves should be Dark Strike
        for _ in 0..5 {
            let (move_type, _) = cultist.choose_move_and_effects(&global_info, &mut rng);
            assert_eq!(move_type, CultistMove::DarkStrike);
        }
    }

    #[test]
    fn test_cultist_ascension_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };
        let global_info_asc17 = GlobalInfo { ascention: 17, current_floor: 1 };

        // Test ritual amount scaling
        assert_eq!(Cultist::calculate_ritual_amount(&global_info_asc0), 3);
        assert_eq!(Cultist::calculate_ritual_amount(&global_info_asc2), 4);
        assert_eq!(Cultist::calculate_ritual_amount(&global_info_asc17), 5);

        // Test HP scaling
        assert_eq!(Cultist::calculate_hp_range(&global_info_asc0), (48, 54));
        assert_eq!(Cultist::calculate_hp_range(&global_info_asc7), (50, 56));
    }

    #[test]
    fn test_cultist_effects() {
        let cultist = Cultist::new(50, 3);

        // Test Incantation effects
        let incantation_effects = cultist.get_move_effects(CultistMove::Incantation);
        assert_eq!(incantation_effects, vec![Effect::GainRitual(3)]);

        // Test Dark Strike effects
        let dark_strike_effects = cultist.get_move_effects(CultistMove::DarkStrike);
        assert_eq!(dark_strike_effects, vec![Effect::AttackToTarget { amount: 6, num_attacks: 1 }]);
    }
}