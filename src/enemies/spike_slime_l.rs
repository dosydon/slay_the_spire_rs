use crate::{game::{effect::Effect, enemy::EnemyTrait, global_info::GlobalInfo}, utils::CategoricalDistribution};
use crate::battle::events::{BattleEvent, EventListener};
use crate::battle::target::Entity;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct SpikeSlimeL {
    last_moves: Vec<SpikeSlimeLMove>,
    hp: u32,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpikeSlimeLMove {
    Lick,
    FlameTackle,
}

impl SpikeSlimeL {
    pub fn new(hp: u32) -> Self {
        SpikeSlimeL {
            hp,
            last_moves: Vec::new(),
        }
    }

    pub fn calculate_flame_tackle_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            18
        } else {
            16
        }
    }

    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 7 {
            (65, 70)
        } else {
            (64, 70)
        }
    }

    fn get_valid_moves(&self, global_info: &GlobalInfo) -> Vec<SpikeSlimeLMove> {
        let mut valid_moves = Vec::new();

        if !self.would_violate_consecutive_rule(SpikeSlimeLMove::Lick, global_info) {
            valid_moves.push(SpikeSlimeLMove::Lick);
        }

        if !self.would_violate_consecutive_rule(SpikeSlimeLMove::FlameTackle, global_info) {
            valid_moves.push(SpikeSlimeLMove::FlameTackle);
        }

        // Fallback if all moves are blocked (should not happen normally)
        if valid_moves.is_empty() {
            valid_moves.push(SpikeSlimeLMove::FlameTackle);
        }

        valid_moves
    }

    fn would_violate_consecutive_rule(&self, move_to_check: SpikeSlimeLMove, global_info: &GlobalInfo) -> bool {
        if global_info.ascention >= 17 {
            // On Ascension 17, cannot use Lick twice in a row
            if move_to_check == SpikeSlimeLMove::Lick && self.last_moves.len() >= 1 {
                return self.last_moves.last() == Some(&SpikeSlimeLMove::Lick);
            }
        }

        // Cannot use the same move three times in a row
        if self.last_moves.len() >= 2 {
            let last_two = &self.last_moves[self.last_moves.len() - 2..];
            return last_two.iter().all(|&m| m == move_to_check);
        }

        false
    }

    fn get_move_weights(&self, moves: &[SpikeSlimeLMove]) -> Vec<f64> {
        moves.iter().map(|&move_type| match move_type {
            SpikeSlimeLMove::FlameTackle => 0.3, // 30% chance
            SpikeSlimeLMove::Lick => 0.7,        // 70% chance
        }).collect()
    }

    fn record_move(&mut self, selected_move: SpikeSlimeLMove) {
        self.last_moves.push(selected_move);
        // Keep only the last 3 moves to prevent unbounded growth
        if self.last_moves.len() > 3 {
            self.last_moves.remove(0);
        }
    }

    pub fn get_move_effects(&self, move_type: SpikeSlimeLMove, global_info: &GlobalInfo) -> Vec<Effect> {
        match move_type {
            SpikeSlimeLMove::Lick => {
                vec![Effect::ApplyFrail { duration: 2 }]
            }
            SpikeSlimeLMove::FlameTackle => {
                vec![
                    Effect::AttackToTarget {
                        amount: Self::calculate_flame_tackle_damage(global_info),
                        num_attacks: 1,
                        strength_multiplier: 1
                    },
                    Effect::AddSlimed(2)
                ]
            }
        }
    }

    /// Get the on-death effects (split into 2 Spike Slime M)
    pub fn get_on_death_effects() -> Vec<Effect> {
        vec![Effect::SplitIntoMediumSlimes]
    }

    fn choose_next_move(&self, global_info: &GlobalInfo) -> CategoricalDistribution<SpikeSlimeLMove> {
        let possible_moves = self.get_valid_moves(global_info);
        let weights = self.get_move_weights(&possible_moves);

        let outcomes_and_weights: Vec<(SpikeSlimeLMove, f64)> = possible_moves
            .into_iter()
            .zip(weights)
            .collect();

        CategoricalDistribution::new(outcomes_and_weights)
    }
}

impl EnemyTrait for SpikeSlimeL {
    type MoveType = SpikeSlimeLMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));

        SpikeSlimeL::new(hp)
    }

    fn get_name() -> String {
        "Spike Slime (L)".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (SpikeSlimeLMove, Vec<Effect>) {
        let move_distribution = self.choose_next_move(global_info);
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
    fn test_spike_slime_l_creation() {
        let spike_slime = SpikeSlimeL::new(65);
        assert_eq!(spike_slime.hp, 65);
        assert!(spike_slime.last_moves.is_empty());
    }

    #[test]
    fn test_spike_slime_l_ascension_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test damage scaling
        assert_eq!(SpikeSlimeL::calculate_flame_tackle_damage(&global_info_asc0), 16);
        assert_eq!(SpikeSlimeL::calculate_flame_tackle_damage(&global_info_asc2), 18);

        // Test HP scaling
        assert_eq!(SpikeSlimeL::calculate_hp_range(&global_info_asc0), (64, 70));
        assert_eq!(SpikeSlimeL::calculate_hp_range(&global_info_asc7), (65, 70));
    }

    #[test]
    fn test_spike_slime_l_instantiation() {
        let mut rng = rand::rng();
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test normal ascension instantiation
        let spike_slime_asc0 = SpikeSlimeL::instantiate(&mut rng, &global_info_asc0);
        assert!(spike_slime_asc0.hp >= 64 && spike_slime_asc0.hp <= 70);

        // Test high ascension instantiation
        let spike_slime_asc7 = SpikeSlimeL::instantiate(&mut rng, &global_info_asc7);
        assert!(spike_slime_asc7.hp >= 65 && spike_slime_asc7.hp <= 70);
    }

    #[test]
    fn test_spike_slime_l_effects() {
        let spike_slime = SpikeSlimeL::new(65);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Test Lick effects (Large applies 2 Frail instead of 1)
        let lick_effects = spike_slime.get_move_effects(SpikeSlimeLMove::Lick, &global_info);
        assert_eq!(lick_effects, vec![Effect::ApplyFrail { duration: 2 }]);

        // Test Flame Tackle effects (Large adds 2 Slimed instead of 1)
        let flame_tackle_effects = spike_slime.get_move_effects(SpikeSlimeLMove::FlameTackle, &global_info);
        assert_eq!(flame_tackle_effects, vec![
            Effect::AttackToTarget { amount: 16, num_attacks: 1, strength_multiplier: 1 },
            Effect::AddSlimed(2)
        ]);
    }

    #[test]
    fn test_consecutive_move_prevention() {
        let mut spike_slime = SpikeSlimeL::new(65);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Fill up with 2 consecutive Licks
        spike_slime.record_move(SpikeSlimeLMove::Lick);
        spike_slime.record_move(SpikeSlimeLMove::Lick);

        // Third Lick should be prevented
        assert!(spike_slime.would_violate_consecutive_rule(SpikeSlimeLMove::Lick, &global_info));
        assert!(!spike_slime.would_violate_consecutive_rule(SpikeSlimeLMove::FlameTackle, &global_info));

        let valid_moves = spike_slime.get_valid_moves(&global_info);
        assert!(!valid_moves.contains(&SpikeSlimeLMove::Lick));
        assert!(valid_moves.contains(&SpikeSlimeLMove::FlameTackle));
    }

    #[test]
    fn test_ascension_17_lick_restriction() {
        let mut spike_slime = SpikeSlimeL::new(65);
        let global_info = GlobalInfo { ascention: 17, current_floor: 1 };

        // Use one Lick
        spike_slime.record_move(SpikeSlimeLMove::Lick);

        // Second Lick should be prevented on Ascension 17
        assert!(spike_slime.would_violate_consecutive_rule(SpikeSlimeLMove::Lick, &global_info));

        let valid_moves = spike_slime.get_valid_moves(&global_info);
        assert!(!valid_moves.contains(&SpikeSlimeLMove::Lick));
        assert!(valid_moves.contains(&SpikeSlimeLMove::FlameTackle));
    }
}

/// Spike Slime L half-HP split listener - creates 2 Spike Slime M when Spike Slime L drops below half HP
pub struct SpikeSlimeLSplitListener {
    owner: Entity,
    active: bool,
}

impl SpikeSlimeLSplitListener {
    pub fn new(enemy_index: usize) -> Self {
        SpikeSlimeLSplitListener {
            owner: Entity::Enemy(enemy_index),
            active: true,
        }
    }
}

impl EventListener for SpikeSlimeLSplitListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        if !self.active {
            return vec![];
        }

        match event {
            BattleEvent::DamageTaken { target, amount: _, source: _ } => {
                if target == &self.owner {
                    // This Spike Slime L took damage - split is handled by damage system
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
}
