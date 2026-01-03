use crate::{game::{effect::BattleEffect, enemy::EnemyTrait, global_info::GlobalInfo}, utils::CategoricalDistribution};
use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::battle::target::Entity;
use std::any::Any;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AcidSlimeL {
    last_moves: Vec<AcidSlimeLMove>,
    hp: u32,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AcidSlimeLMove {
    CorrosiveSpit,
    Tackle,
}

impl AcidSlimeL {
    pub fn new(hp: u32) -> Self {
        AcidSlimeL {
            hp,
            last_moves: Vec::new(),
        }
    }

    pub fn calculate_tackle_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            16
        } else {
            12
        }
    }

    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 7 {
            (68, 72)
        } else {
            (65, 69)
        }
    }

    fn get_valid_moves(&self) -> Vec<AcidSlimeLMove> {
        let mut valid_moves = Vec::new();

        if !self.would_violate_consecutive_rule(AcidSlimeLMove::CorrosiveSpit) {
            valid_moves.push(AcidSlimeLMove::CorrosiveSpit);
        }

        if !self.would_violate_consecutive_rule(AcidSlimeLMove::Tackle) {
            valid_moves.push(AcidSlimeLMove::Tackle);
        }

        // Fallback if all moves are blocked (should not happen normally)
        if valid_moves.is_empty() {
            valid_moves.push(AcidSlimeLMove::Tackle);
        }

        valid_moves
    }

    fn would_violate_consecutive_rule(&self, move_to_check: AcidSlimeLMove) -> bool {
        // Cannot use the same move three times in a row
        if self.last_moves.len() >= 2 {
            let last_two = &self.last_moves[self.last_moves.len() - 2..];
            return last_two.iter().all(|&m| m == move_to_check);
        }

        false
    }

    fn get_move_weights(&self, moves: &[AcidSlimeLMove]) -> Vec<f64> {
        moves.iter().map(|&move_type| match move_type {
            AcidSlimeLMove::CorrosiveSpit => 0.7, // 70% chance
            AcidSlimeLMove::Tackle => 0.3,        // 30% chance
        }).collect()
    }

    fn record_move(&mut self, selected_move: AcidSlimeLMove) {
        self.last_moves.push(selected_move);
        // Keep only the last 3 moves to prevent unbounded growth
        if self.last_moves.len() > 3 {
            self.last_moves.remove(0);
        }
    }

    pub fn get_move_effects(&self, move_type: AcidSlimeLMove, global_info: &GlobalInfo) -> Vec<BattleEffect> {
        match move_type {
            AcidSlimeLMove::CorrosiveSpit => {
                // Large Acid Slime applies 2 Weak instead of 1
                vec![BattleEffect::ApplyWeak { duration: 2 }]
            }
            AcidSlimeLMove::Tackle => {
                vec![
                    BattleEffect::AttackToTarget {
                        amount: Self::calculate_tackle_damage(global_info),
                        num_attacks: 1,
                        strength_multiplier: 1
                    },
                    // Large Acid Slime adds 2 Slimed
                    BattleEffect::AddSlimed(2)
                ]
            }
        }
    }

    /// Get the on-death effects (split into 2 Acid Slime M)
    pub fn get_on_death_effects() -> Vec<BattleEffect> {
        vec![BattleEffect::SplitIntoMediumSlimes]
    }

    fn choose_next_move(&self) -> CategoricalDistribution<AcidSlimeLMove> {
        let possible_moves = self.get_valid_moves();
        let weights = self.get_move_weights(&possible_moves);

        let outcomes_and_weights: Vec<(AcidSlimeLMove, f64)> = possible_moves
            .into_iter()
            .zip(weights)
            .collect();

        CategoricalDistribution::new(outcomes_and_weights)
    }
}

impl EnemyTrait for AcidSlimeL {
    type MoveType = AcidSlimeLMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));

        AcidSlimeL::new(hp)
    }

    fn get_name() -> String {
        "Acid Slime (L)".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (AcidSlimeLMove, Vec<BattleEffect>) {
        let move_distribution = self.choose_next_move();
        let selected_move = move_distribution.sample_owned(rng);

        // Record the move for consecutive move tracking
        self.record_move(selected_move);

        // Generate the effects for this move
        let effects = self.get_move_effects(selected_move, global_info);

        (selected_move, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::global_info::GlobalInfo;

    #[test]
    fn test_acid_slime_l_creation() {
        let acid_slime = AcidSlimeL::new(65);
        assert_eq!(acid_slime.hp, 65);
        assert!(acid_slime.last_moves.is_empty());
    }

    #[test]
    fn test_acid_slime_l_ascension_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test damage scaling
        assert_eq!(AcidSlimeL::calculate_tackle_damage(&global_info_asc0), 12);
        assert_eq!(AcidSlimeL::calculate_tackle_damage(&global_info_asc2), 16);

        // Test HP scaling
        assert_eq!(AcidSlimeL::calculate_hp_range(&global_info_asc0), (65, 69));
        assert_eq!(AcidSlimeL::calculate_hp_range(&global_info_asc7), (68, 72));
    }

    #[test]
    fn test_acid_slime_l_instantiation() {
        let mut rng = rand::rng();
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test normal ascension instantiation
        let acid_slime_asc0 = AcidSlimeL::instantiate(&mut rng, &global_info_asc0);
        assert!(acid_slime_asc0.hp >= 65 && acid_slime_asc0.hp <= 69);

        // Test high ascension instantiation
        let acid_slime_asc7 = AcidSlimeL::instantiate(&mut rng, &global_info_asc7);
        assert!(acid_slime_asc7.hp >= 68 && acid_slime_asc7.hp <= 72);
    }

    #[test]
    fn test_acid_slime_l_effects() {
        let acid_slime = AcidSlimeL::new(65);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Test Corrosive Spit effects (Large applies 2 Weak)
        let corrosive_spit_effects = acid_slime.get_move_effects(AcidSlimeLMove::CorrosiveSpit, &global_info);
        assert_eq!(corrosive_spit_effects, vec![BattleEffect::ApplyWeak { duration: 2 }]);

        // Test Tackle effects (Large deals more damage and adds 2 Slimed)
        let tackle_effects = acid_slime.get_move_effects(AcidSlimeLMove::Tackle, &global_info);
        assert_eq!(tackle_effects, vec![
            BattleEffect::AttackToTarget { amount: 12, num_attacks: 1, strength_multiplier: 1 },
            BattleEffect::AddSlimed(2)
        ]);
    }

    #[test]
    fn test_consecutive_move_prevention() {
        let mut acid_slime = AcidSlimeL::new(65);

        // Fill up with 2 consecutive Tackles
        acid_slime.record_move(AcidSlimeLMove::Tackle);
        acid_slime.record_move(AcidSlimeLMove::Tackle);

        // Third Tackle should be prevented
        assert!(acid_slime.would_violate_consecutive_rule(AcidSlimeLMove::Tackle));
        assert!(!acid_slime.would_violate_consecutive_rule(AcidSlimeLMove::CorrosiveSpit));

        let valid_moves = acid_slime.get_valid_moves();
        assert!(!valid_moves.contains(&AcidSlimeLMove::Tackle));
        assert!(valid_moves.contains(&AcidSlimeLMove::CorrosiveSpit));
    }

    #[test]
    fn test_on_death_effects() {
        let effects = AcidSlimeL::get_on_death_effects();

        // Should have one effect: SplitIntoMediumSlimes
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], crate::game::effect::BattleEffect::SplitIntoMediumSlimes));
    }
}

/// Acid Slime L half-HP split listener - creates 2 Acid Slime M when Acid Slime L drops below half HP
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcidSlimeLSplitListener {
    owner: Entity,
    active: bool,
}

impl AcidSlimeLSplitListener {
    pub fn new(enemy_index: usize) -> Self {
        AcidSlimeLSplitListener {
            owner: Entity::Enemy(enemy_index),
            active: true,
        }
    }
}

impl EventListener for AcidSlimeLSplitListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        if !self.active {
            return vec![];
        }

        match event {
            BattleEvent::DamageTaken { target, amount: _, source: _ } => {
                if target == &self.owner {
                    // This Acid Slime L took damage - split is handled by damage system
                    self.active = false; // Prevent further triggers
                    vec![]
                } else {
                    vec![]
                }
            }
            _ => vec![],
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn hash_to(&self, state: &mut std::collections::hash_map::DefaultHasher) {
        use std::hash::Hash;
        self.hash(state);
    }
}
