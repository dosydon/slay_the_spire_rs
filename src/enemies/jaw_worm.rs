use crate::{enemies::{enemy_enum::EnemyEnum, jaw_worm}, game::{effect::Effect, enemy::EnemyTrait, global_info::GlobalInfo}, utils::CategoricalDistribution};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct JawWorm {
    last_move: Option<JawWormMove>,
    consecutive_thrash_count: u32,
    hp: u32,
    is_act3: bool,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum JawWormMove {
    Chomp,
    Bellow,
    Thrash,
}

impl JawWorm {
    pub fn new(hp: u32, is_act3: bool) -> Self {
        JawWorm { 
            last_move: None,
            consecutive_thrash_count: 0,
            hp,
            is_act3,
        }
    }

    /// Calculate Chomp damage based on ascension
    pub fn calculate_chomp_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 { 12 } else { 11 }
    }

    /// Calculate Bellow strength gain based on ascension
    pub fn calculate_bellow_strength(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 17 {
            5
        } else if global_info.ascention >= 2 {
            4
        } else {
            3
        }
    }

    /// Calculate Bellow block gain based on ascension
    pub fn calculate_bellow_block(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 17 { 9 } else { 6 }
    }

    fn get_move_effects(&self, move_type: JawWormMove, global_info: &GlobalInfo) -> Vec<Effect> {
        match move_type {
            JawWormMove::Chomp => {
                vec![Effect::AttackToTarget {
                    amount: Self::calculate_chomp_damage(global_info),
                    num_attacks: 1,
                    strength_multiplier: 1
                }]
            }
            JawWormMove::Bellow => {
                vec![
                    Effect::GainStrength(Self::calculate_bellow_strength(global_info)),
                    Effect::GainDefense(Self::calculate_bellow_block(global_info))
                ]
            }
            JawWormMove::Thrash => {
                vec![
                    Effect::AttackToTarget {
                        amount: 7,
                        num_attacks: 1,
                        strength_multiplier: 1
                    },
                    Effect::GainDefense(5)
                ]
            }
        }
    }

    fn record_move(&mut self, selected_move: JawWormMove) {
        // Track consecutive Thrash usage
        if selected_move == JawWormMove::Thrash {
            if let Some(JawWormMove::Thrash) = self.last_move {
                self.consecutive_thrash_count += 1;
            } else {
                self.consecutive_thrash_count = 1;
            }
        } else {
            self.consecutive_thrash_count = 0;
        }
        
        self.last_move = Some(selected_move);
    }

    fn get_move_probabilities(&self) -> (f64, f64, f64) {
        // Returns (chomp_prob, bellow_prob, thrash_prob)
        
        // Special handling for consecutive Thrash (2 in a row)
        if self.consecutive_thrash_count >= 2 {
            return (0.36, 0.64, 0.0); // Cannot use Thrash
        }
        
        match self.last_move {
            None => {
                // First move
                if self.is_act3 {
                    // Act 3: 25% Chomp, 45% Bellow, 30% Thrash
                    (0.25, 0.45, 0.30)
                } else {
                    // Act 1: Always starts with Chomp
                    (1.0, 0.0, 0.0)
                }
            }
            Some(JawWormMove::Chomp) => {
                // After Chomp: 0% Chomp, 59% Bellow, 41% Thrash
                (0.0, 0.59, 0.41)
            }
            Some(JawWormMove::Bellow) => {
                // After Bellow: 44% Chomp, 0% Bellow, 56% Thrash
                (0.44, 0.0, 0.56)
            }
            Some(JawWormMove::Thrash) => {
                // After Thrash: 25% Chomp, 45% Bellow, 30% Thrash
                (0.25, 0.45, 0.30)
            }
        }
    }



    /// Apply initial Bellow effects for Act 3 (called during instantiation)
    pub fn apply_initial_bellow_effects(&self, global_info: &GlobalInfo) -> Vec<Effect> {
        if self.is_act3 {
            vec![
                Effect::GainStrength(Self::calculate_bellow_strength(global_info)),
                Effect::GainDefense(Self::calculate_bellow_block(global_info))
            ]
        } else {
            Vec::new()
        }
    }

    fn choose_next_move(&self, _global_info: &GlobalInfo) -> CategoricalDistribution<JawWormMove> {
        let (chomp_prob, bellow_prob, thrash_prob) = self.get_move_probabilities();
        
        let mut outcomes_and_weights = Vec::new();
        
        if chomp_prob > 0.0 {
            outcomes_and_weights.push((JawWormMove::Chomp, chomp_prob));
        }
        if bellow_prob > 0.0 {
            outcomes_and_weights.push((JawWormMove::Bellow, bellow_prob));
        }
        if thrash_prob > 0.0 {
            outcomes_and_weights.push((JawWormMove::Thrash, thrash_prob));
        }
        
        // Ensure we have at least one valid move
        if outcomes_and_weights.is_empty() {
            outcomes_and_weights.push((JawWormMove::Chomp, 1.0));
        }
        
        CategoricalDistribution::new(outcomes_and_weights)
    }
}

impl EnemyTrait for JawWorm {
    type MoveType = JawWormMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        // Determine if this is Act 3 based on current floor
        let is_act3 = global_info.current_floor > 50; // Assuming Act 3 starts after floor 50
        
        // Calculate HP with ascension scaling
        let base_hp = if global_info.ascention >= 7 {
            42 + rng.random_range(0..=4) // 42-46
        } else {
            40 + rng.random_range(0..=4) // 40-44
        };
        
        JawWorm::new(base_hp, is_act3)
    }

    fn get_name() -> String {
        "Jaw Worm".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (JawWormMove, Vec<Effect>) {
        let move_distribution = self.choose_next_move(global_info);
        let selected_move = move_distribution.sample_owned(rng);
        
        println!("JawWorm selected move: {:?}", selected_move);
        
        // Record the move for move tracking
        self.record_move(selected_move);
        
        // Generate the effects for this move
        let effects = self.get_move_effects(selected_move, global_info);
        
        (selected_move, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::Battle;
    use crate::cards::ironclad::starter_deck::starter_deck;

    #[test]
    fn test_jaw_worm_creation() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        assert!(jaw_worm.hp >= 40 && jaw_worm.hp <= 44); // Act 1 HP range
        assert!(!jaw_worm.is_act3);
    }

    #[test]
    fn test_act3_jaw_worm_creation() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 51 }; // Act 3
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        assert!(jaw_worm.hp >= 40 && jaw_worm.hp <= 44); // Act 3 HP range
        assert!(jaw_worm.is_act3);
    }


    #[test]
    fn test_name() {
        assert_eq!(JawWorm::get_name(), "Jaw Worm");
    }

    #[test]
    fn test_ascension_hp_scaling() {
        let mut rng = rand::rng();
        
        // Test normal ascension HP (40-44)
        let normal_global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let normal_jaw_worm = JawWorm::instantiate(&mut rng, &normal_global_info);
        assert!(normal_jaw_worm.hp >= 40 && normal_jaw_worm.hp <= 44);
        
        // Test ascension 7+ HP (42-46)
        let high_global_info = GlobalInfo { ascention: 7, current_floor: 1 };
        let high_jaw_worm = JawWorm::instantiate(&mut rng, &high_global_info);
        assert!(high_jaw_worm.hp >= 42 && high_jaw_worm.hp <= 46);
    }

    #[test]
    fn test_chomp_damage_scaling() {
        let normal_global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let high_global_info = GlobalInfo { ascention: 2, current_floor: 1 };
        
        assert_eq!(JawWorm::calculate_chomp_damage(&normal_global_info), 11);
        assert_eq!(JawWorm::calculate_chomp_damage(&high_global_info), 12);
    }

    #[test]
    fn test_bellow_scaling() {
        let low_global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mid_global_info = GlobalInfo { ascention: 2, current_floor: 1 };
        let high_global_info = GlobalInfo { ascention: 17, current_floor: 1 };
        
        // Test strength scaling
        assert_eq!(JawWorm::calculate_bellow_strength(&low_global_info), 3);
        assert_eq!(JawWorm::calculate_bellow_strength(&mid_global_info), 4);
        assert_eq!(JawWorm::calculate_bellow_strength(&high_global_info), 5);
        
        // Test block scaling
        assert_eq!(JawWorm::calculate_bellow_block(&low_global_info), 6);
        assert_eq!(JawWorm::calculate_bellow_block(&mid_global_info), 6);
        assert_eq!(JawWorm::calculate_bellow_block(&high_global_info), 9);
    }

    #[test]
    fn test_first_move_act1() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        
        // Act 1 should always start with Chomp
        let move_distribution = jaw_worm.choose_next_move(&global_info);
        
        // Sample multiple times to ensure it's always Chomp
        for _ in 0..10 {
            let chosen_move = move_distribution.sample(&mut rng);
            assert_eq!(chosen_move, &JawWormMove::Chomp);
        }
    }

    #[test]
    fn test_first_move_act3() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 51 }; // Act 3
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        
        let move_distribution = jaw_worm.choose_next_move(&global_info);
        
        // Act 3 should have varied first moves, test that we can get different moves
        let mut moves_seen = std::collections::HashSet::new();
        for _ in 0..50 {
            let chosen_move = move_distribution.sample(&mut rng);
            moves_seen.insert(*chosen_move);
        }
        
        // Should see at least 2 different moves with enough samples
        assert!(moves_seen.len() >= 2);
    }

    #[test]
    fn test_move_effects_chomp() {
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::new(42, false);
        let effects = jaw_worm.get_move_effects(JawWormMove::Chomp, &global_info);
        
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier: 1 } => {
                assert_eq!(*amount, 11);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_move_effects_bellow() {
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::new(42, false);
        let effects = jaw_worm.get_move_effects(JawWormMove::Bellow, &global_info);
        
        assert_eq!(effects.len(), 2);
        
        let mut found_strength = false;
        let mut found_defense = false;
        
        for effect in &effects {
            match effect {
                Effect::GainStrength(amount) => {
                    assert_eq!(*amount, 3);
                    found_strength = true;
                }
                Effect::GainDefense(amount) => {
                    assert_eq!(*amount, 6);
                    found_defense = true;
                }
                _ => panic!("Unexpected effect type for Bellow"),
            }
        }
        
        assert!(found_strength && found_defense);
    }

    #[test]
    fn test_move_effects_thrash() {
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::new(42, false);
        let effects = jaw_worm.get_move_effects(JawWormMove::Thrash, &global_info);
        
        assert_eq!(effects.len(), 2);
        
        let mut found_attack = false;
        let mut found_defense = false;
        
        for effect in &effects {
            match effect {
                Effect::AttackToTarget { amount, num_attacks, strength_multiplier: 1 } => {
                    assert_eq!(*amount, 7);
                    assert_eq!(*num_attacks, 1);
                    found_attack = true;
                }
                Effect::GainDefense(amount) => {
                    assert_eq!(*amount, 5);
                    found_defense = true;
                }
                _ => panic!("Unexpected effect type for Thrash"),
            }
        }
        
        assert!(found_attack && found_defense);
    }

    #[test]
    fn test_consecutive_thrash_prevention() {
        let mut jaw_worm = JawWorm::new(42, false);
        
        // Manually set up two consecutive Thrash moves
        jaw_worm.record_move(JawWormMove::Thrash);
        jaw_worm.record_move(JawWormMove::Thrash);
        
        // Now consecutive_thrash_count should be 2
        assert_eq!(jaw_worm.consecutive_thrash_count, 2);
        
        // Get probabilities - Thrash should be 0%
        let (chomp_prob, bellow_prob, thrash_prob) = jaw_worm.get_move_probabilities();
        assert_eq!(thrash_prob, 0.0);
        assert!(chomp_prob > 0.0 || bellow_prob > 0.0);
    }

    #[test]
    fn test_move_probability_transitions() {
        let jaw_worm = JawWorm::new(42, false);
        
        // Test transitions from each move type
        let mut jaw_worm_after_chomp = jaw_worm.clone();
        jaw_worm_after_chomp.record_move(JawWormMove::Chomp);
        let (chomp_prob, bellow_prob, thrash_prob) = jaw_worm_after_chomp.get_move_probabilities();
        assert_eq!(chomp_prob, 0.0);
        assert_eq!(bellow_prob, 0.59);
        assert_eq!(thrash_prob, 0.41);
        
        let mut jaw_worm_after_bellow = jaw_worm.clone();
        jaw_worm_after_bellow.record_move(JawWormMove::Bellow);
        let (chomp_prob, bellow_prob, thrash_prob) = jaw_worm_after_bellow.get_move_probabilities();
        assert_eq!(chomp_prob, 0.44);
        assert_eq!(bellow_prob, 0.0);
        assert_eq!(thrash_prob, 0.56);
    }

    #[test]
    fn test_choose_move_and_effects_records_moves() {
        let mut jaw_worm = JawWorm::new(42, false);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        assert!(jaw_worm.last_move.is_none());
        let (_move, _effects) = jaw_worm.choose_move_and_effects(&global_info, &mut rng);
        
        // Should have recorded a move
        assert!(jaw_worm.last_move.is_some());
    }

    #[test]
    fn test_act3_initial_bellow_effects() {
        let global_info = GlobalInfo { ascention: 0, current_floor: 51 }; // Act 3
        let jaw_worm = JawWorm::new(42, true);
        
        let initial_effects = jaw_worm.apply_initial_bellow_effects(&global_info);
        assert_eq!(initial_effects.len(), 2);
        
        let mut found_strength = false;
        let mut found_defense = false;
        
        for effect in &initial_effects {
            match effect {
                Effect::GainStrength(amount) => {
                    assert_eq!(*amount, 3);
                    found_strength = true;
                }
                Effect::GainDefense(amount) => {
                    assert_eq!(*amount, 6);
                    found_defense = true;
                }
                _ => panic!("Unexpected effect type for initial bellow"),
            }
        }
        
        assert!(found_strength && found_defense);
    }

    #[test]
    fn test_act1_no_initial_bellow_effects() {
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 }; // Act 1
        let jaw_worm = JawWorm::new(42, false);
        
        let initial_effects = jaw_worm.apply_initial_bellow_effects(&global_info);
        assert_eq!(initial_effects.len(), 0);
    }

    // Integration tests
    #[test]
    fn test_jaw_worm_battle_integration() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create a Jaw Worm enemy
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        println!("JawWorm is_act3: {}, last_move: {:?}", jaw_worm.is_act3, jaw_worm.last_move);
        let (chomp_prob, bellow_prob, thrash_prob) = jaw_worm.get_move_probabilities();
        println!("Move probabilities: Chomp={}, Bellow={}, Thrash={}", chomp_prob, bellow_prob, thrash_prob);
        let enemies = vec![crate::battle::enemy_in_battle::EnemyInBattle::new(crate::enemies::EnemyEnum::JawWorm(jaw_worm))];
        
        // Create battle with Jaw Worm
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Verify Jaw Worm was created correctly
        assert_eq!(battle.get_enemies().len(), 1);
        assert!(battle.get_enemies()[0].battle_info.is_alive());
        
        // Verify player starts with correct stats
        assert_eq!(battle.get_player().battle_info.get_hp(), 80);
        assert_eq!(battle.get_player().get_energy(), 3);
        
        // Simulate enemy turn - Jaw Worm should use Chomp first in Act 1
        let initial_player_hp = battle.get_player().battle_info.get_hp();
        // Note: sample_enemy_actions is already called by Battle::new -> start_turn
        
        // Debug: Check what move was sampled
        if let Some(enemy_move) = battle.get_enemy_move(0) {
            println!("Enemy move: {:?}", enemy_move);
        }
        
        battle.process_enemy_effects(&mut rng, &global_info);
        battle.at_end_of_enemy_turn();
        
        let final_player_hp = battle.get_player().battle_info.get_hp();
        let actual_damage = initial_player_hp - final_player_hp;
        
        // Player should take damage from Chomp (11 damage)
        let expected_damage = JawWorm::calculate_chomp_damage(&global_info);
        println!("Expected damage: {}, Actual damage: {}, Player HP: {} -> {}", 
                expected_damage, actual_damage, initial_player_hp, final_player_hp);
        
        assert_eq!(battle.get_player().battle_info.get_hp(), initial_player_hp - expected_damage);
        
        println!("Jaw Worm battle integration test passed!");
        println!("Player HP after Chomp: {} (took {} damage)", 
                battle.get_player().battle_info.get_hp(), expected_damage);
    }

    #[test] 
    fn test_jaw_worm_act3_battle_integration() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 51 }; // Act 3
        
        // Create a Jaw Worm enemy (Act 3)
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![crate::battle::enemy_in_battle::EnemyInBattle::new(crate::enemies::EnemyEnum::JawWorm(jaw_worm))];
        
        // Create battle with Act 3 Jaw Worm
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Act 3 Jaw Worm should start with initial Bellow effects
        // This would typically be applied during enemy creation, but for this test
        // we'll verify the enemy exists and can perform moves
        
        assert!(battle.get_enemies()[0].battle_info.is_alive());
        
        // Simulate enemy turn 
        let initial_player_hp = battle.get_player().battle_info.get_hp();
        // Note: sample_enemy_actions is already called by Battle::new -> start_turn
        battle.process_enemy_effects(&mut rng, &global_info);
        battle.at_end_of_enemy_turn();
        
        // In Act 3, first move probabilities are different, so we just verify
        // that some action was taken (either damage or no damage if Bellow was used)
        let final_player_hp = battle.get_player().battle_info.get_hp();
        
        // Should either take damage or stay the same (if Bellow was used)
        assert!(final_player_hp <= initial_player_hp);
        
        println!("Act 3 Jaw Worm battle integration test passed!");
        println!("Player HP after enemy turn: {} (started at {})", 
                final_player_hp, initial_player_hp);
    }

    #[test]
    fn test_jaw_worm_multiple_turns() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create a Jaw Worm enemy
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![crate::battle::enemy_in_battle::EnemyInBattle::new(crate::enemies::EnemyEnum::JawWorm(jaw_worm))];
        
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Simulate multiple enemy turns to test move sequence
        let mut player_damage_taken = 0;
        let mut enemy_strength_gained = 0;
        
        for turn in 0..5 {
            let player_hp_before = battle.get_player().battle_info.get_hp();
            let enemy_strength_before = battle.get_enemies()[0].battle_info.get_strength();
            
            // Only sample actions for turns after the first one
            // (First turn actions are already sampled by Battle::new -> start_turn)
            if turn > 0 {
                battle.start_of_player_turn(&mut rng);
            }
            battle.process_enemy_effects(&mut rng, &global_info);
        battle.at_end_of_enemy_turn();
            
            let player_hp_after = battle.get_player().battle_info.get_hp();
            let enemy_strength_after = battle.get_enemies()[0].battle_info.get_strength();
            
            if player_hp_after < player_hp_before {
                player_damage_taken += player_hp_before - player_hp_after;
                println!("Turn {}: Player took {} damage", turn + 1, player_hp_before - player_hp_after);
            }
            
            if enemy_strength_after > enemy_strength_before {
                enemy_strength_gained += enemy_strength_after - enemy_strength_before;
                println!("Turn {}: Enemy gained {} strength", turn + 1, enemy_strength_after - enemy_strength_before);
            }
        }
        
        // Over 5 turns, either damage should be dealt or strength should be gained
        assert!(player_damage_taken > 0 || enemy_strength_gained > 0);
        
        println!("Multi-turn Jaw Worm test passed!");
        println!("Total damage dealt: {}, Total strength gained: {}", 
                player_damage_taken, enemy_strength_gained);
    }
}