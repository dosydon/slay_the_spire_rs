use crate::{game::{effect::Effect, enemy::EnemyTrait, global_info::GlobalInfo}, utils::CategoricalDistribution};

#[derive(Clone, Debug)]
pub struct SpikeSlimeM {
    last_moves: Vec<SpikeSlimeMMove>,
    hp: u32,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpikeSlimeMMove {
    Lick,
    FlameTackle,
}

impl SpikeSlimeM {
    pub fn new(hp: u32) -> Self {
        SpikeSlimeM {
            hp,
            last_moves: Vec::new(),
        }
    }

    pub fn calculate_flame_tackle_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            10
        } else {
            8
        }
    }

    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 7 {
            (29, 34)
        } else {
            (28, 32)
        }
    }

    fn get_valid_moves(&self, global_info: &GlobalInfo) -> Vec<SpikeSlimeMMove> {
        let mut valid_moves = Vec::new();
        
        if !self.would_violate_consecutive_rule(SpikeSlimeMMove::Lick, global_info) {
            valid_moves.push(SpikeSlimeMMove::Lick);
        }
        
        if !self.would_violate_consecutive_rule(SpikeSlimeMMove::FlameTackle, global_info) {
            valid_moves.push(SpikeSlimeMMove::FlameTackle);
        }
        
        // Fallback if all moves are blocked (should not happen normally)
        if valid_moves.is_empty() {
            valid_moves.push(SpikeSlimeMMove::FlameTackle);
        }
        
        valid_moves
    }

    fn would_violate_consecutive_rule(&self, move_to_check: SpikeSlimeMMove, global_info: &GlobalInfo) -> bool {
        if global_info.ascention >= 17 {
            // On Ascension 17, cannot use Lick twice in a row
            if move_to_check == SpikeSlimeMMove::Lick && self.last_moves.len() >= 1 {
                return self.last_moves.last() == Some(&SpikeSlimeMMove::Lick);
            }
        }
        
        // Cannot use the same move three times in a row
        if self.last_moves.len() >= 2 {
            let last_two = &self.last_moves[self.last_moves.len() - 2..];
            return last_two.iter().all(|&m| m == move_to_check);
        }
        
        false
    }

    fn get_move_weights(&self, moves: &[SpikeSlimeMMove]) -> Vec<f64> {
        moves.iter().map(|&move_type| match move_type {
            SpikeSlimeMMove::FlameTackle => 0.3, // 30% chance
            SpikeSlimeMMove::Lick => 0.7,        // 70% chance
        }).collect()
    }

    fn record_move(&mut self, selected_move: SpikeSlimeMMove) {
        self.last_moves.push(selected_move);
        // Keep only the last 3 moves to prevent unbounded growth
        if self.last_moves.len() > 3 {
            self.last_moves.remove(0);
        }
    }

    pub fn get_move_effects(&self, move_type: SpikeSlimeMMove, global_info: &GlobalInfo) -> Vec<Effect> {
        match move_type {
            SpikeSlimeMMove::Lick => {
                vec![Effect::ApplyFrail { duration: 1 }]
            }
            SpikeSlimeMMove::FlameTackle => {
                vec![
                    Effect::AttackToTarget {
                        amount: Self::calculate_flame_tackle_damage(global_info),
                        num_attacks: 1,
                        strength_multiplier: 1
                    },
                    Effect::AddSlimed(1)
                ]
            }
        }
    }

    fn choose_next_move(&self, global_info: &GlobalInfo) -> CategoricalDistribution<SpikeSlimeMMove> {
        let possible_moves = self.get_valid_moves(global_info);
        let weights = self.get_move_weights(&possible_moves);
        
        let outcomes_and_weights: Vec<(SpikeSlimeMMove, f64)> = possible_moves
            .into_iter()
            .zip(weights)
            .collect();

        CategoricalDistribution::new(outcomes_and_weights)
    }
}

impl EnemyTrait for SpikeSlimeM {
    type MoveType = SpikeSlimeMMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));
        
        SpikeSlimeM::new(hp)
    }

    fn get_name() -> String {
        "Spike Slime (M)".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (SpikeSlimeMMove, Vec<Effect>) {
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
    fn test_spike_slime_m_creation() {
        let spike_slime = SpikeSlimeM::new(30);
        assert_eq!(spike_slime.hp, 30);
        assert!(spike_slime.last_moves.is_empty());
    }

    #[test]
    fn test_spike_slime_m_ascension_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test damage scaling
        assert_eq!(SpikeSlimeM::calculate_flame_tackle_damage(&global_info_asc0), 8);
        assert_eq!(SpikeSlimeM::calculate_flame_tackle_damage(&global_info_asc2), 10);

        // Test HP scaling
        assert_eq!(SpikeSlimeM::calculate_hp_range(&global_info_asc0), (28, 32));
        assert_eq!(SpikeSlimeM::calculate_hp_range(&global_info_asc7), (29, 34));
    }

    #[test]
    fn test_spike_slime_m_move_pattern() {
        let mut spike_slime = SpikeSlimeM::new(30);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();

        // Test that we can get both moves
        let mut moves_seen = std::collections::HashSet::new();
        for _ in 0..50 {
            let (move_type, _effects) = spike_slime.choose_move_and_effects(&global_info, &mut rng);
            moves_seen.insert(move_type);
        }
        
        // Should see both moves with enough samples
        assert!(moves_seen.len() >= 2);
        assert!(moves_seen.contains(&SpikeSlimeMMove::Lick));
        assert!(moves_seen.contains(&SpikeSlimeMMove::FlameTackle));
    }

    #[test]
    fn test_spike_slime_m_instantiation() {
        let mut rng = rand::rng();
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test normal ascension instantiation
        let spike_slime_asc0 = SpikeSlimeM::instantiate(&mut rng, &global_info_asc0);
        assert!(spike_slime_asc0.hp >= 28 && spike_slime_asc0.hp <= 32);

        // Test high ascension instantiation
        let spike_slime_asc7 = SpikeSlimeM::instantiate(&mut rng, &global_info_asc7);
        assert!(spike_slime_asc7.hp >= 29 && spike_slime_asc7.hp <= 34);
    }

    #[test]
    fn test_spike_slime_m_effects() {
        let spike_slime = SpikeSlimeM::new(30);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Test Lick effects
        let lick_effects = spike_slime.get_move_effects(SpikeSlimeMMove::Lick, &global_info);
        assert_eq!(lick_effects, vec![Effect::ApplyFrail { duration: 1 }]);

        // Test Flame Tackle effects
        let flame_tackle_effects = spike_slime.get_move_effects(SpikeSlimeMMove::FlameTackle, &global_info);
        assert_eq!(flame_tackle_effects, vec![
            Effect::AttackToTarget { amount: 8, num_attacks: 1, strength_multiplier: 1 },
            Effect::AddSlimed(1)
        ]);
    }

    #[test]
    fn test_consecutive_move_prevention() {
        let mut spike_slime = SpikeSlimeM::new(30);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Fill up with 2 consecutive Licks
        spike_slime.record_move(SpikeSlimeMMove::Lick);
        spike_slime.record_move(SpikeSlimeMMove::Lick);
        
        // Third Lick should be prevented
        assert!(spike_slime.would_violate_consecutive_rule(SpikeSlimeMMove::Lick, &global_info));
        assert!(!spike_slime.would_violate_consecutive_rule(SpikeSlimeMMove::FlameTackle, &global_info));
        
        let valid_moves = spike_slime.get_valid_moves(&global_info);
        assert!(!valid_moves.contains(&SpikeSlimeMMove::Lick));
        assert!(valid_moves.contains(&SpikeSlimeMMove::FlameTackle));
    }

    #[test]
    fn test_ascension_17_lick_restriction() {
        let mut spike_slime = SpikeSlimeM::new(30);
        let global_info = GlobalInfo { ascention: 17, current_floor: 1 };
        
        // Use one Lick
        spike_slime.record_move(SpikeSlimeMMove::Lick);
        
        // Second Lick should be prevented on Ascension 17
        assert!(spike_slime.would_violate_consecutive_rule(SpikeSlimeMMove::Lick, &global_info));
        
        let valid_moves = spike_slime.get_valid_moves(&global_info);
        assert!(!valid_moves.contains(&SpikeSlimeMMove::Lick));
        assert!(valid_moves.contains(&SpikeSlimeMMove::FlameTackle));
    }

    #[test]
    fn test_move_weights() {
        let spike_slime = SpikeSlimeM::new(30);
        let moves = vec![SpikeSlimeMMove::Lick, SpikeSlimeMMove::FlameTackle];
        let weights = spike_slime.get_move_weights(&moves);
        
        assert_eq!(weights, vec![0.7, 0.3]); // Lick 70%, FlameTackle 30%
    }

    #[test]
    fn test_choose_move_and_effects_records_moves() {
        let mut spike_slime = SpikeSlimeM::new(30);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        assert!(spike_slime.last_moves.is_empty());
        let (_move, _effects) = spike_slime.choose_move_and_effects(&global_info, &mut rng);
        
        // Should have recorded one move
        assert_eq!(spike_slime.last_moves.len(), 1);
    }
}