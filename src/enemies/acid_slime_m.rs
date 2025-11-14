use crate::{game::{effect::Effect, enemy::EnemyTrait, global_info::GlobalInfo}, utils::CategoricalDistribution};

#[derive(Clone, Debug)]
pub struct AcidSlimeM {
    last_moves: Vec<AcidSlimeMMove>,
    hp: u32,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AcidSlimeMMove {
    CorrosiveSpit,
    Tackle,
}

impl AcidSlimeM {
    pub fn new(hp: u32) -> Self {
        AcidSlimeM {
            hp,
            last_moves: Vec::new(),
        }
    }

    pub fn calculate_tackle_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            10
        } else {
            7
        }
    }

    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 7 {
            (29, 34)
        } else {
            (28, 32)
        }
    }

    fn get_valid_moves(&self) -> Vec<AcidSlimeMMove> {
        let mut valid_moves = Vec::new();
        
        if !self.would_violate_consecutive_rule(AcidSlimeMMove::CorrosiveSpit) {
            valid_moves.push(AcidSlimeMMove::CorrosiveSpit);
        }
        
        if !self.would_violate_consecutive_rule(AcidSlimeMMove::Tackle) {
            valid_moves.push(AcidSlimeMMove::Tackle);
        }
        
        // Fallback if all moves are blocked (should not happen normally)
        if valid_moves.is_empty() {
            valid_moves.push(AcidSlimeMMove::Tackle);
        }
        
        valid_moves
    }

    fn would_violate_consecutive_rule(&self, move_to_check: AcidSlimeMMove) -> bool {
        // Cannot use the same move three times in a row
        if self.last_moves.len() >= 2 {
            let last_two = &self.last_moves[self.last_moves.len() - 2..];
            return last_two.iter().all(|&m| m == move_to_check);
        }
        
        false
    }

    fn get_move_weights(&self, moves: &[AcidSlimeMMove]) -> Vec<f64> {
        moves.iter().map(|&move_type| match move_type {
            AcidSlimeMMove::CorrosiveSpit => 0.7, // 70% chance
            AcidSlimeMMove::Tackle => 0.3,        // 30% chance
        }).collect()
    }

    fn record_move(&mut self, selected_move: AcidSlimeMMove) {
        self.last_moves.push(selected_move);
        // Keep only the last 3 moves to prevent unbounded growth
        if self.last_moves.len() > 3 {
            self.last_moves.remove(0);
        }
    }

    pub fn get_move_effects(&self, move_type: AcidSlimeMMove, global_info: &GlobalInfo) -> Vec<Effect> {
        match move_type {
            AcidSlimeMMove::CorrosiveSpit => {
                vec![Effect::ApplyWeak(2)]
            }
            AcidSlimeMMove::Tackle => {
                vec![Effect::AttackToTarget { 
                    amount: Self::calculate_tackle_damage(global_info), 
                    num_attacks: 1 
                }]
            }
        }
    }

    fn choose_next_move(&self) -> CategoricalDistribution<AcidSlimeMMove> {
        let possible_moves = self.get_valid_moves();
        let weights = self.get_move_weights(&possible_moves);
        
        let outcomes_and_weights: Vec<(AcidSlimeMMove, f64)> = possible_moves
            .into_iter()
            .zip(weights)
            .collect();

        CategoricalDistribution::new(outcomes_and_weights)
    }
}

impl EnemyTrait for AcidSlimeM {
    type MoveType = AcidSlimeMMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));
        
        AcidSlimeM::new(hp)
    }

    fn get_name() -> String {
        "Acid Slime (M)".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (AcidSlimeMMove, Vec<Effect>) {
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
    fn test_acid_slime_m_creation() {
        let acid_slime = AcidSlimeM::new(30);
        assert_eq!(acid_slime.hp, 30);
        assert!(acid_slime.last_moves.is_empty());
    }

    #[test]
    fn test_acid_slime_m_ascension_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test damage scaling
        assert_eq!(AcidSlimeM::calculate_tackle_damage(&global_info_asc0), 7);
        assert_eq!(AcidSlimeM::calculate_tackle_damage(&global_info_asc2), 10);

        // Test HP scaling
        assert_eq!(AcidSlimeM::calculate_hp_range(&global_info_asc0), (28, 32));
        assert_eq!(AcidSlimeM::calculate_hp_range(&global_info_asc7), (29, 34));
    }

    #[test]
    fn test_acid_slime_m_move_pattern() {
        let mut acid_slime = AcidSlimeM::new(30);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();

        // Test that we can get both moves
        let mut moves_seen = std::collections::HashSet::new();
        for _ in 0..50 {
            let (move_type, _effects) = acid_slime.choose_move_and_effects(&global_info, &mut rng);
            moves_seen.insert(move_type);
        }
        
        // Should see both moves with enough samples
        assert!(moves_seen.len() >= 2);
        assert!(moves_seen.contains(&AcidSlimeMMove::CorrosiveSpit));
        assert!(moves_seen.contains(&AcidSlimeMMove::Tackle));
    }

    #[test]
    fn test_acid_slime_m_instantiation() {
        let mut rng = rand::rng();
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test normal ascension instantiation
        let acid_slime_asc0 = AcidSlimeM::instantiate(&mut rng, &global_info_asc0);
        assert!(acid_slime_asc0.hp >= 28 && acid_slime_asc0.hp <= 32);

        // Test high ascension instantiation
        let acid_slime_asc7 = AcidSlimeM::instantiate(&mut rng, &global_info_asc7);
        assert!(acid_slime_asc7.hp >= 29 && acid_slime_asc7.hp <= 34);
    }

    #[test]
    fn test_acid_slime_m_effects() {
        let acid_slime = AcidSlimeM::new(30);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Test Corrosive Spit effects
        let corrosive_effects = acid_slime.get_move_effects(AcidSlimeMMove::CorrosiveSpit, &global_info);
        assert_eq!(corrosive_effects, vec![Effect::ApplyWeak(2)]);

        // Test Tackle effects
        let tackle_effects = acid_slime.get_move_effects(AcidSlimeMMove::Tackle, &global_info);
        assert_eq!(tackle_effects, vec![
            Effect::AttackToTarget { amount: 7, num_attacks: 1 }
        ]);

        // Test ascension damage scaling
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };
        let tackle_effects_asc2 = acid_slime.get_move_effects(AcidSlimeMMove::Tackle, &global_info_asc2);
        assert_eq!(tackle_effects_asc2, vec![
            Effect::AttackToTarget { amount: 10, num_attacks: 1 }
        ]);
    }

    #[test]
    fn test_consecutive_move_prevention() {
        let mut acid_slime = AcidSlimeM::new(30);
        
        // Fill up with 2 consecutive CorrosiveSpits
        acid_slime.record_move(AcidSlimeMMove::CorrosiveSpit);
        acid_slime.record_move(AcidSlimeMMove::CorrosiveSpit);
        
        // Third CorrosiveSpit should be prevented
        assert!(acid_slime.would_violate_consecutive_rule(AcidSlimeMMove::CorrosiveSpit));
        assert!(!acid_slime.would_violate_consecutive_rule(AcidSlimeMMove::Tackle));
        
        let valid_moves = acid_slime.get_valid_moves();
        assert!(!valid_moves.contains(&AcidSlimeMMove::CorrosiveSpit));
        assert!(valid_moves.contains(&AcidSlimeMMove::Tackle));
    }

    #[test]
    fn test_move_weights() {
        let acid_slime = AcidSlimeM::new(30);
        let moves = vec![AcidSlimeMMove::CorrosiveSpit, AcidSlimeMMove::Tackle];
        let weights = acid_slime.get_move_weights(&moves);
        
        assert_eq!(weights, vec![0.7, 0.3]); // CorrosiveSpit 70%, Tackle 30%
    }

    #[test]
    fn test_choose_move_and_effects_records_moves() {
        let mut acid_slime = AcidSlimeM::new(30);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        assert!(acid_slime.last_moves.is_empty());
        let (_move, _effects) = acid_slime.choose_move_and_effects(&global_info, &mut rng);
        
        // Should have recorded one move
        assert_eq!(acid_slime.last_moves.len(), 1);
    }

    #[test]
    fn test_acid_slime_m_name() {
        assert_eq!(AcidSlimeM::get_name(), "Acid Slime (M)");
    }

    #[test]
    fn test_acid_slime_m_battle_integration() {
        use crate::battle::{Battle, enemy_in_battle::EnemyInBattle};
        use crate::enemies::EnemyEnum;
        use crate::cards::ironclad::starter_deck::starter_deck;
        
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let acid_slime = AcidSlimeM::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::AcidSlimeM(acid_slime))];
        let battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Test that the enemy is properly set up
        assert_eq!(battle.get_enemies().len(), 1);
        assert!(battle.get_enemies()[0].battle_info.is_alive());
        let enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert!(enemy_hp >= 28 && enemy_hp <= 32);
        
        // Test move generation using a separate AcidSlimeM instance
        let mut test_slime = AcidSlimeM::instantiate(&mut rng, &global_info);
        let (enemy_move, effects) = test_slime.choose_move_and_effects(&global_info, &mut rng);
        
        // Should get either CorrosiveSpit or Tackle
        match enemy_move {
            AcidSlimeMMove::CorrosiveSpit => {
                assert_eq!(effects, vec![Effect::ApplyWeak(2)]);
            }
            AcidSlimeMMove::Tackle => {
                assert_eq!(effects, vec![Effect::AttackToTarget { amount: 7, num_attacks: 1 }]);
            }
        }
    }

    #[test]
    fn test_acid_slime_m_applies_weak_to_player() {
        use crate::battle::{Battle, enemy_in_battle::EnemyInBattle};
        use crate::enemies::EnemyEnum;
        use crate::cards::ironclad::starter_deck::starter_deck;
        
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create AcidSlimeM enemy
        let acid_slime = AcidSlimeM::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::AcidSlimeM(acid_slime))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Verify player starts without Weak
        assert!(!battle.get_player().battle_info.is_weak());
        assert_eq!(battle.get_player().battle_info.get_weak_turns(), 0);
        
        // Force the AcidSlimeM to use CorrosiveSpit by manually setting up the action
        // We need to create a separate AcidSlimeM to generate the move
        let mut test_slime = AcidSlimeM::new(30);
        let effects = test_slime.get_move_effects(AcidSlimeMMove::CorrosiveSpit, &global_info);
        
        // Verify the effect is ApplyWeak(2)
        assert_eq!(effects, vec![Effect::ApplyWeak(2)]);
        
        // Apply the Weak effect to the player through the battle system
        use crate::game::effect::BaseEffect;
        use crate::battle::target::Entity;
        
        let weak_effect = BaseEffect::ApplyWeak {
            target: Entity::Player,
            duration: 2,
        };
        
        battle.eval_base_effect(&weak_effect);
        
        // Verify player now has Weak status for 2 turns
        assert!(battle.get_player().battle_info.is_weak());
        assert_eq!(battle.get_player().battle_info.get_weak_turns(), 2);
    }

    #[test]
    fn test_weak_status_reduces_player_damage() {
        use crate::battle::{Battle, enemy_in_battle::EnemyInBattle};
        use crate::enemies::EnemyEnum;
        use crate::cards::ironclad::starter_deck::starter_deck;
        use crate::game::effect::BaseEffect;
        use crate::battle::target::Entity;
        
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create a battle with AcidSlimeM enemy
        let acid_slime = AcidSlimeM::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::AcidSlimeM(acid_slime))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Test damage calculation without Weak
        let base_damage = 10;
        let normal_damage = battle.get_player().battle_info.calculate_damage(base_damage);
        assert_eq!(normal_damage, 10, "Normal damage should equal base damage");
        
        // Apply Weak status to the player
        let weak_effect = BaseEffect::ApplyWeak {
            target: Entity::Player,
            duration: 2,
        };
        battle.eval_base_effect(&weak_effect);
        
        // Verify Weak is applied
        assert!(battle.get_player().battle_info.is_weak());
        assert_eq!(battle.get_player().battle_info.get_weak_turns(), 2);
        
        // Test damage calculation with Weak (should be 25% reduced)
        let weak_damage = battle.get_player().battle_info.calculate_damage(base_damage);
        let expected_weak_damage = (base_damage as f32 * 0.75) as u32;
        assert_eq!(weak_damage, expected_weak_damage, "Weak should reduce damage by 25%");
        assert_eq!(weak_damage, 7, "10 damage with Weak should become 7");
        
        // Test with different base damage values
        assert_eq!(battle.get_player().battle_info.calculate_damage(12), 9); // 12 * 0.75 = 9
        assert_eq!(battle.get_player().battle_info.calculate_damage(8), 6);  // 8 * 0.75 = 6
        assert_eq!(battle.get_player().battle_info.calculate_damage(20), 15); // 20 * 0.75 = 15
        
        // Test edge case with 1 damage
        assert_eq!(battle.get_player().battle_info.calculate_damage(1), 0);  // 1 * 0.75 = 0.75 -> 0
        
        // Simulate turn passage to test Weak duration
        battle.get_player_mut().battle_info.at_end_of_turn();
        assert!(battle.get_player().battle_info.is_weak());
        assert_eq!(battle.get_player().battle_info.get_weak_turns(), 1);
        
        // Damage should still be reduced
        assert_eq!(battle.get_player().battle_info.calculate_damage(base_damage), 7);
        
        // After second turn, Weak should expire
        battle.get_player_mut().battle_info.at_end_of_turn();
        assert!(!battle.get_player().battle_info.is_weak());
        assert_eq!(battle.get_player().battle_info.get_weak_turns(), 0);
        
        // Damage should return to normal
        assert_eq!(battle.get_player().battle_info.calculate_damage(base_damage), 10);
    }
}