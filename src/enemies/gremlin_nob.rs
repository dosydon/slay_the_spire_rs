use crate::{game::{effect::Effect, enemy::EnemyTrait, global_info::GlobalInfo}, utils::CategoricalDistribution};

#[derive(Clone, Debug)]
pub struct GremlinNob {
    hp: u32,
    enrage_stacks: u32,
    last_moves: Vec<GremlinNobMove>,
    turn_count: u32,
    has_used_first_move: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GremlinNobMove {
    Bellow,
    SkullBash,
    BullRush,
}

impl GremlinNob {
    pub fn new(hp: u32) -> Self {
        GremlinNob {
            hp,
            enrage_stacks: 0,
            last_moves: Vec::new(),
            turn_count: 0,
            has_used_first_move: false,
        }
    }

    /// Calculate HP range based on ascension
    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 8 {
            (85, 90)
        } else {
            (82, 86)
        }
    }

    /// Calculate Skull Bash damage based on ascension
    pub fn calculate_skull_bash_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 3 {
            8
        } else {
            6
        }
    }

    /// Calculate Bull Rush damage based on ascension
    pub fn calculate_bull_rush_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 3 {
            16
        } else {
            14
        }
    }

    /// Calculate Enrage amount gained from Bellow based on ascension
    pub fn calculate_enrage_amount(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 18 {
            3
        } else {
            2
        }
    }

    /// Check if we should use the special Ascension 18+ pattern
    fn is_ascension_18_plus(&self, global_info: &GlobalInfo) -> bool {
        global_info.ascention >= 18
    }

    fn get_valid_moves(&self, global_info: &GlobalInfo) -> Vec<GremlinNobMove> {
        // Always start with Bellow
        if !self.has_used_first_move {
            return vec![GremlinNobMove::Bellow];
        }

        if self.is_ascension_18_plus(global_info) {
            // Ascension 18+ pattern: Bellow -> Skull Bash -> Bull Rush -> Bull Rush -> repeat
            // Turn 1: Bellow (handled above)
            // Turn 2: Skull Bash 
            // Turn 3-4: Bull Rush
            // Turn 5: Skull Bash
            // Turn 6-7: Bull Rush
            // etc.
            // Note: turn_count represents completed turns, so for the next move we check turn_count + 1
            match self.turn_count + 1 {
                2 | 5 | 8 | 11 => vec![GremlinNobMove::SkullBash], // Every 3rd turn starting from turn 2
                _ => vec![GremlinNobMove::BullRush], // All other turns
            }
        } else {
            // Standard pattern: 33% Skull Bash, 67% Bull Rush
            // Cannot use Bull Rush 3 times in a row
            let consecutive_bull_rush = self.count_consecutive_bull_rush();
            
            if consecutive_bull_rush >= 2 {
                // Must use Skull Bash to break the sequence
                vec![GremlinNobMove::SkullBash]
            } else {
                // Both moves are valid
                vec![GremlinNobMove::SkullBash, GremlinNobMove::BullRush]
            }
        }
    }

    fn count_consecutive_bull_rush(&self) -> u32 {
        let mut count = 0;
        for &move_type in self.last_moves.iter().rev() {
            if move_type == GremlinNobMove::BullRush {
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    fn get_move_weights(&self, moves: &[GremlinNobMove]) -> Vec<f64> {
        if moves.len() == 1 {
            // Only one valid move
            vec![1.0]
        } else {
            // Standard pattern: 33% Skull Bash, 67% Bull Rush
            moves.iter().map(|&move_type| match move_type {
                GremlinNobMove::SkullBash => 0.33,
                GremlinNobMove::BullRush => 0.67,
                GremlinNobMove::Bellow => 1.0, // This should never happen in weight calculation
            }).collect()
        }
    }

    fn record_move(&mut self, selected_move: GremlinNobMove) {
        self.last_moves.push(selected_move);
        self.turn_count += 1;
        
        if !self.has_used_first_move {
            self.has_used_first_move = true;
        }
        
        // Keep only the last 3 moves to prevent unbounded growth
        if self.last_moves.len() > 3 {
            self.last_moves.remove(0);
        }
    }

    pub fn get_move_effects(&self, move_type: GremlinNobMove, global_info: &GlobalInfo) -> Vec<Effect> {
        match move_type {
            GremlinNobMove::Bellow => {
                // Bellow activates Enrage - no immediate strength gain
                let enrage_amount = Self::calculate_enrage_amount(global_info);
                vec![Effect::ActivateEnrage(enrage_amount)]
            }
            GremlinNobMove::SkullBash => {
                let damage = Self::calculate_skull_bash_damage(global_info);
                vec![
                    Effect::AttackToTarget { amount: damage, num_attacks: 1 },
                    Effect::ApplyVulnerable(2),
                ]
            }
            GremlinNobMove::BullRush => {
                let damage = Self::calculate_bull_rush_damage(global_info);
                vec![Effect::AttackToTarget { amount: damage, num_attacks: 1 }]
            }
        }
    }

    fn choose_next_move(&self, global_info: &GlobalInfo) -> CategoricalDistribution<GremlinNobMove> {
        let valid_moves = self.get_valid_moves(global_info);
        let weights = self.get_move_weights(&valid_moves);
        
        let outcomes_and_weights: Vec<(GremlinNobMove, f64)> = valid_moves
            .into_iter()
            .zip(weights)
            .collect();

        CategoricalDistribution::new(outcomes_and_weights)
    }

    /// Get current enrage stacks (for external inspection)
    pub fn get_enrage_stacks(&self) -> u32 {
        self.enrage_stacks
    }

    /// Add enrage stacks (called when player plays a Skill card)
    pub fn add_enrage_stacks(&mut self, amount: u32) -> u32 {
        self.enrage_stacks += amount;
        amount // Return the amount of Strength gained
    }
}

impl EnemyTrait for GremlinNob {
    type MoveType = GremlinNobMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));
        
        GremlinNob::new(hp)
    }

    fn get_name() -> String {
        "Gremlin Nob".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (GremlinNobMove, Vec<Effect>) {
        let move_distribution = self.choose_next_move(global_info);
        let selected_move = move_distribution.sample_owned(rng);
        
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
    use crate::game::global_info::GlobalInfo;

    #[test]
    fn test_gremlin_nob_creation() {
        let gremlin_nob = GremlinNob::new(85);
        assert_eq!(gremlin_nob.hp, 85);
        assert_eq!(gremlin_nob.enrage_stacks, 0);
        assert!(gremlin_nob.last_moves.is_empty());
        assert_eq!(gremlin_nob.turn_count, 0);
        assert!(!gremlin_nob.has_used_first_move);
    }

    #[test]
    fn test_gremlin_nob_ascension_scaling() {
        // HP scaling
        let global_info_low = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_high = GlobalInfo { ascention: 8, current_floor: 1 };
        
        assert_eq!(GremlinNob::calculate_hp_range(&global_info_low), (82, 86));
        assert_eq!(GremlinNob::calculate_hp_range(&global_info_high), (85, 90));

        // Damage scaling
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };
        let global_info_asc3 = GlobalInfo { ascention: 3, current_floor: 1 };
        
        assert_eq!(GremlinNob::calculate_skull_bash_damage(&global_info_asc2), 6);
        assert_eq!(GremlinNob::calculate_skull_bash_damage(&global_info_asc3), 8);
        assert_eq!(GremlinNob::calculate_bull_rush_damage(&global_info_asc2), 14);
        assert_eq!(GremlinNob::calculate_bull_rush_damage(&global_info_asc3), 16);

        // Enrage scaling
        let global_info_asc17 = GlobalInfo { ascention: 17, current_floor: 1 };
        let global_info_asc18 = GlobalInfo { ascention: 18, current_floor: 1 };
        
        assert_eq!(GremlinNob::calculate_enrage_amount(&global_info_asc17), 2);
        assert_eq!(GremlinNob::calculate_enrage_amount(&global_info_asc18), 3);
    }

    #[test]
    fn test_first_move_is_always_bellow() {
        let mut gremlin_nob = GremlinNob::new(85);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();
        
        // First move should always be Bellow
        let (first_move, first_effects) = gremlin_nob.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(first_move, GremlinNobMove::Bellow);
        assert_eq!(first_effects, vec![Effect::ActivateEnrage(2)]);
        assert!(gremlin_nob.has_used_first_move);
    }

    #[test]
    fn test_standard_move_pattern() {
        let mut gremlin_nob = GremlinNob::new(85);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();
        
        // Use first move (Bellow)
        gremlin_nob.choose_move_and_effects(&global_info, &mut rng);
        
        // Test that we get both types of moves in subsequent turns
        let mut moves_seen = std::collections::HashSet::new();
        for _ in 0..50 {
            let (move_type, _effects) = gremlin_nob.choose_move_and_effects(&global_info, &mut rng);
            moves_seen.insert(move_type);
        }
        
        // Should see both SkullBash and BullRush
        assert!(moves_seen.contains(&GremlinNobMove::SkullBash));
        assert!(moves_seen.contains(&GremlinNobMove::BullRush));
    }

    #[test]
    fn test_consecutive_bull_rush_prevention() {
        let mut gremlin_nob = GremlinNob::new(85);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Use first move (Bellow)
        gremlin_nob.record_move(GremlinNobMove::Bellow);
        gremlin_nob.has_used_first_move = true;
        
        // Force two consecutive Bull Rush moves
        gremlin_nob.record_move(GremlinNobMove::BullRush);
        gremlin_nob.record_move(GremlinNobMove::BullRush);
        
        // Third move should be forced to be Skull Bash
        let valid_moves = gremlin_nob.get_valid_moves(&global_info);
        assert_eq!(valid_moves, vec![GremlinNobMove::SkullBash]);
    }

    #[test]
    fn test_ascension_18_pattern() {
        let mut gremlin_nob = GremlinNob::new(85);
        let global_info = GlobalInfo { ascention: 18, current_floor: 1 };
        
        // Simulate the pattern: Bellow -> Skull Bash -> Bull Rush -> Bull Rush -> repeat
        gremlin_nob.record_move(GremlinNobMove::Bellow); // Turn 1
        gremlin_nob.has_used_first_move = true;
        
        // Turn 2: Should be Skull Bash
        let valid_moves_2 = gremlin_nob.get_valid_moves(&global_info);
        assert_eq!(valid_moves_2, vec![GremlinNobMove::SkullBash]);
        
        gremlin_nob.record_move(GremlinNobMove::SkullBash); // Turn 2
        
        // Turn 3: Should be Bull Rush
        let valid_moves_3 = gremlin_nob.get_valid_moves(&global_info);
        assert_eq!(valid_moves_3, vec![GremlinNobMove::BullRush]);
        
        gremlin_nob.record_move(GremlinNobMove::BullRush); // Turn 3
        
        // Turn 4: Should be Bull Rush
        let valid_moves_4 = gremlin_nob.get_valid_moves(&global_info);
        assert_eq!(valid_moves_4, vec![GremlinNobMove::BullRush]);
        
        gremlin_nob.record_move(GremlinNobMove::BullRush); // Turn 4
        
        // Turn 5: Should be Skull Bash again
        let valid_moves_5 = gremlin_nob.get_valid_moves(&global_info);
        assert_eq!(valid_moves_5, vec![GremlinNobMove::SkullBash]);
    }

    #[test]
    fn test_move_effects() {
        let gremlin_nob = GremlinNob::new(85);
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc3 = GlobalInfo { ascention: 3, current_floor: 1 };
        let global_info_asc18 = GlobalInfo { ascention: 18, current_floor: 1 };
        
        // Test Bellow effects
        let bellow_effects_asc0 = gremlin_nob.get_move_effects(GremlinNobMove::Bellow, &global_info_asc0);
        assert_eq!(bellow_effects_asc0, vec![Effect::ActivateEnrage(2)]);
        
        let bellow_effects_asc18 = gremlin_nob.get_move_effects(GremlinNobMove::Bellow, &global_info_asc18);
        assert_eq!(bellow_effects_asc18, vec![Effect::ActivateEnrage(3)]);
        
        // Test Skull Bash effects
        let skull_bash_effects_asc0 = gremlin_nob.get_move_effects(GremlinNobMove::SkullBash, &global_info_asc0);
        assert_eq!(skull_bash_effects_asc0, vec![
            Effect::AttackToTarget { amount: 6, num_attacks: 1 },
            Effect::ApplyVulnerable(2),
        ]);
        
        let skull_bash_effects_asc3 = gremlin_nob.get_move_effects(GremlinNobMove::SkullBash, &global_info_asc3);
        assert_eq!(skull_bash_effects_asc3, vec![
            Effect::AttackToTarget { amount: 8, num_attacks: 1 },
            Effect::ApplyVulnerable(2),
        ]);
        
        // Test Bull Rush effects
        let bull_rush_effects_asc0 = gremlin_nob.get_move_effects(GremlinNobMove::BullRush, &global_info_asc0);
        assert_eq!(bull_rush_effects_asc0, vec![Effect::AttackToTarget { amount: 14, num_attacks: 1 }]);
        
        let bull_rush_effects_asc3 = gremlin_nob.get_move_effects(GremlinNobMove::BullRush, &global_info_asc3);
        assert_eq!(bull_rush_effects_asc3, vec![Effect::AttackToTarget { amount: 16, num_attacks: 1 }]);
    }

    #[test]
    fn test_enrage_mechanism() {
        let mut gremlin_nob = GremlinNob::new(85);
        
        assert_eq!(gremlin_nob.get_enrage_stacks(), 0);
        
        // Add enrage stacks (simulating player playing Skill cards)
        let strength_gained = gremlin_nob.add_enrage_stacks(2);
        assert_eq!(strength_gained, 2);
        assert_eq!(gremlin_nob.get_enrage_stacks(), 2);
        
        // Add more stacks
        gremlin_nob.add_enrage_stacks(3);
        assert_eq!(gremlin_nob.get_enrage_stacks(), 5);
    }

    #[test]
    fn test_instantiate_hp_range() {
        let mut rng = rand::rng();
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc8 = GlobalInfo { ascention: 8, current_floor: 1 };
        
        // Test multiple instantiations to ensure HP is in correct range
        for _ in 0..10 {
            let nob_asc0 = GremlinNob::instantiate(&mut rng, &global_info_asc0);
            assert!(nob_asc0.hp >= 82 && nob_asc0.hp <= 86);
            
            let nob_asc8 = GremlinNob::instantiate(&mut rng, &global_info_asc8);
            assert!(nob_asc8.hp >= 85 && nob_asc8.hp <= 90);
        }
    }

    #[test]
    fn test_gremlin_nob_name() {
        assert_eq!(GremlinNob::get_name(), "Gremlin Nob");
    }

    #[test]
    fn test_gremlin_nob_battle_integration() {
        use crate::battle::{Battle, enemy_in_battle::EnemyInBattle};
        use crate::enemies::EnemyEnum;
        use crate::cards::ironclad::starter_deck::starter_deck;
        use crate::game::enemy::EnemyTrait;
        
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create Gremlin Nob enemy
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];
        let battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Test that the enemy is properly set up
        assert_eq!(battle.get_enemies().len(), 1);
        assert!(battle.get_enemies()[0].battle_info.is_alive());
        let enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert!(enemy_hp >= 82 && enemy_hp <= 86);
        
        // Test move generation using a separate GremlinNob instance
        let mut test_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let (enemy_move, effects) = test_nob.choose_move_and_effects(&global_info, &mut rng);
        
        // First move should always be Bellow
        assert_eq!(enemy_move, GremlinNobMove::Bellow);
        assert_eq!(effects, vec![Effect::ActivateEnrage(2)]);
        
        // Second move should be either SkullBash or BullRush
        let (second_move, second_effects) = test_nob.choose_move_and_effects(&global_info, &mut rng);
        match second_move {
            GremlinNobMove::SkullBash => {
                assert_eq!(second_effects, vec![
                    Effect::AttackToTarget { amount: 6, num_attacks: 1 },
                    Effect::ApplyVulnerable(2),
                ]);
            }
            GremlinNobMove::BullRush => {
                assert_eq!(second_effects, vec![Effect::AttackToTarget { amount: 14, num_attacks: 1 }]);
            }
            GremlinNobMove::Bellow => {
                panic!("Second move should not be Bellow again");
            }
        }
    }

    #[test]
    fn test_activate_enrage_effect() {
        use crate::battle::{Battle, enemy_in_battle::EnemyInBattle};
        use crate::enemies::EnemyEnum;
        use crate::cards::ironclad::starter_deck::starter_deck;
        use crate::game::{enemy::EnemyTrait, effect::BaseEffect};
        use crate::battle::target::Entity;
        
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create Gremlin Nob enemy
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Manually apply ActivateEnrage effect
        let activate_enrage_effect = BaseEffect::ActivateEnrage {
            source: Entity::Enemy(0),
            amount: 2,
        };
        battle.eval_base_effect(&activate_enrage_effect);
        
        // Now test that the EnrageListener works
        use crate::battle::events::BattleEvent;
        let skill_event = BattleEvent::SkillCardPlayed {
            source: Entity::Player,
        };
        
        let initial_strength = battle.get_enemies()[0].battle_info.get_strength();
        battle.emit_event(skill_event);
        let final_strength = battle.get_enemies()[0].battle_info.get_strength();
        
        assert_eq!(final_strength, initial_strength + 2);
    }

    #[test] 
    fn test_gremlin_nob_enrage_mechanic() {
        use crate::battle::{Battle, enemy_in_battle::EnemyInBattle, events::BattleEvent, target::Entity};
        use crate::enemies::EnemyEnum;
        use crate::cards::ironclad::starter_deck::starter_deck;
        use crate::game::{enemy::EnemyTrait, effect::BaseEffect};
        
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Test that a fresh GremlinNob generates Bellow as first move
        let mut gremlin_nob_test = GremlinNob::instantiate(&mut rng, &global_info);
        let (first_move, first_effects) = gremlin_nob_test.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(first_move, GremlinNobMove::Bellow, "First move should be Bellow");
        assert_eq!(first_effects.len(), 1, "Bellow should have exactly one effect");
        
        match &first_effects[0] {
            crate::game::effect::Effect::ActivateEnrage(amount) => {
                assert_eq!(*amount, GremlinNob::calculate_enrage_amount(&global_info), "ActivateEnrage amount should be correct");
            }
            _ => panic!("Bellow should generate ActivateEnrage effect, got {:?}", first_effects[0]),
        }
        
        // Test the enrage mechanic directly
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_strength = battle.get_enemies()[0].battle_info.get_strength();
        
        // Manually apply the ActivateEnrage effect (as would happen after Bellow)
        let enrage_effect = BaseEffect::ActivateEnrage { 
            source: Entity::Enemy(0), 
            amount: GremlinNob::calculate_enrage_amount(&global_info) 
        };
        battle.eval_base_effect(&enrage_effect);
        
        // Emit skill event and verify enrage triggers
        let skill_event = BattleEvent::SkillCardPlayed {
            source: Entity::Player,
        };
        battle.emit_event(skill_event);
        
        let final_strength = battle.get_enemies()[0].battle_info.get_strength();
        let expected_strength_gain = GremlinNob::calculate_enrage_amount(&global_info);
        assert_eq!(final_strength, initial_strength + expected_strength_gain,
                  "Enrage should trigger on skill card. Initial: {}, Final: {}, Expected gain: {}",
                  initial_strength, final_strength, expected_strength_gain);
        
        // Test multiple triggers
        let second_skill_event = BattleEvent::SkillCardPlayed {
            source: Entity::Player,
        };
        battle.emit_event(second_skill_event);
        let final_strength_2 = battle.get_enemies()[0].battle_info.get_strength();
        assert_eq!(final_strength_2, initial_strength + (2 * expected_strength_gain),
                  "Multiple skill cards should stack enrage. Expected: {}, Got: {}", 
                  initial_strength + (2 * expected_strength_gain), final_strength_2);
    }
}