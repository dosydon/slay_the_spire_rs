use crate::{game::{effect::BattleEffect, enemy::EnemyTrait, global_info::GlobalInfo}, utils::CategoricalDistribution};

#[derive(Clone, Debug)]
pub struct GreenLouse {
    last_moves: Vec<GreenLouseMove>,
    base_damage: u32,
    hp: u32,
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum GreenLouseMove {
    Attack,
    Weaken,
}

impl GreenLouse {
    pub fn new(base_damage: u32, hp: u32) -> Self {
        GreenLouse { 
            last_moves: Vec::new(),
            base_damage,
            hp,
        }
    }

    pub fn calculate_base_damage(global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> u32 {
        // Base damage is 3-4, +1 on Ascension 2+
        let base_damage_roll = 3 + rng.random_range(0..=1);
        let ascension_bonus = if global_info.ascention >= 2 { 1 } else { 0 };
        base_damage_roll + ascension_bonus
    }

    fn get_valid_moves(&self) -> Vec<GreenLouseMove> {
        let mut valid_moves = Vec::new();
        
        if !self.would_violate_consecutive_rule(GreenLouseMove::Attack) {
            valid_moves.push(GreenLouseMove::Attack);
        }
        
        if !self.would_violate_consecutive_rule(GreenLouseMove::Weaken) {
            valid_moves.push(GreenLouseMove::Weaken);
        }
        
        if valid_moves.is_empty() {
            vec![GreenLouseMove::Attack]
        } else {
            valid_moves
        }
    }

    fn would_violate_consecutive_rule(&self, move_to_check: GreenLouseMove) -> bool {
        if self.last_moves.len() < 3 {
            return false;
        }

        let last_three = &self.last_moves[self.last_moves.len() - 3..];
        last_three.iter().all(|&m| m == move_to_check)
    }

    fn get_move_weights(&self, moves: &[GreenLouseMove]) -> Vec<f64> {
        moves.iter().map(|&move_type| match move_type {
            GreenLouseMove::Attack => 0.75, // Slightly prefer attacking
            GreenLouseMove::Weaken => 0.25,
        }).collect()
    }

    fn record_move(&mut self, selected_move: GreenLouseMove) {
        self.last_moves.push(selected_move);
    }

    fn get_move_effects(&self, move_type: GreenLouseMove) -> Vec<BattleEffect> {
        match move_type {
            GreenLouseMove::Attack => {
                vec![BattleEffect::AttackToTarget {
                    amount: self.base_damage,
                    num_attacks: 1,
                    strength_multiplier: 1
                }]
            }
            GreenLouseMove::Weaken => {
                vec![BattleEffect::ApplyWeak { duration: 2 }] // Apply 2 turns of weak
            }
        }
    }


    fn choose_next_move(&self, _global_info: &GlobalInfo) -> CategoricalDistribution<GreenLouseMove> {
        let possible_moves = self.get_valid_moves();
        let weights = self.get_move_weights(&possible_moves);
        
        let outcomes_and_weights: Vec<(GreenLouseMove, f64)> = possible_moves
            .into_iter()
            .zip(weights)
            .collect();

        CategoricalDistribution::new(outcomes_and_weights)
    }

}

impl EnemyTrait for GreenLouse {
    type MoveType = GreenLouseMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        // Calculate base damage using ascension scaling
        let base_damage = Self::calculate_base_damage(global_info, rng);
        // Create the enemy instance
        let hp = 11 + rng.random_range(0..=6); // 11-17 HP range
        let green_louse = GreenLouse::new(base_damage, hp);

        green_louse
    }

    fn get_name() -> String {
        "Green Louse".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (GreenLouseMove, Vec<BattleEffect>) {
        let move_distribution = self.choose_next_move(global_info);
        let selected_move = move_distribution.sample_owned(rng);
        
        // Record the move for consecutive move tracking
        self.record_move(selected_move);
        
        // Generate the effects for this move
        let effects = self.get_move_effects(selected_move);
        
        (selected_move, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    

    #[test]
    fn test_green_louse_creation() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let green_louse = GreenLouse::instantiate(&mut rng, &global_info);
        assert!(green_louse.hp >= 11 && green_louse.hp <= 17);
        assert!(green_louse.base_damage >= 3 && green_louse.base_damage <= 4);
    }


    #[test]
    fn test_name() {
        assert_eq!(GreenLouse::get_name(), "Green Louse");
    }

    #[test]
    fn test_ascension_damage_scaling() {
        let mut rng = rand::rng();
        
        // Test normal ascension damage (3-4)
        let normal_global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let normal_damage = GreenLouse::calculate_base_damage(&normal_global_info, &mut rng);
        assert!(normal_damage >= 3 && normal_damage <= 4);
        
        // Test ascension 2+ damage (4-5)
        let high_global_info = GlobalInfo { ascention: 2, current_floor: 1 };
        let high_damage = GreenLouse::calculate_base_damage(&high_global_info, &mut rng);
        assert!(high_damage >= 4 && high_damage <= 5);
    }

    #[test]
    fn test_choose_next_move() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let green_louse = GreenLouse::instantiate(&mut rng, &global_info);
        let move_distribution = green_louse.choose_next_move(&global_info);
        
        // Test that we can sample moves
        let _move1 = move_distribution.sample(&mut rng);
        let _move2 = move_distribution.sample(&mut rng);
    }

    #[test]
    fn test_move_effects_attack() {
        let green_louse = GreenLouse::new(4, 15);
        let effects = green_louse.get_move_effects(GreenLouseMove::Attack);
        
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            BattleEffect::AttackToTarget { amount, num_attacks, strength_multiplier: 1 } => {
                assert_eq!(*amount, 4);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_move_effects_weaken() {
        let green_louse = GreenLouse::new(4, 15);
        let effects = green_louse.get_move_effects(GreenLouseMove::Weaken);
        
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            BattleEffect::ApplyWeak { duration: turns } => {
                assert_eq!(*turns, 2);
            }
            _ => panic!("Expected ApplyWeak effect"),
        }
    }

    #[test]
    fn test_consecutive_move_prevention() {
        let mut green_louse = GreenLouse::new(4, 15);
        
        // Fill up with 3 consecutive attacks
        green_louse.record_move(GreenLouseMove::Attack);
        green_louse.record_move(GreenLouseMove::Attack);
        green_louse.record_move(GreenLouseMove::Attack);
        
        // Fourth attack should be prevented
        assert!(green_louse.would_violate_consecutive_rule(GreenLouseMove::Attack));
        assert!(!green_louse.would_violate_consecutive_rule(GreenLouseMove::Weaken));
        
        let valid_moves = green_louse.get_valid_moves();
        assert!(!valid_moves.contains(&GreenLouseMove::Attack));
        assert!(valid_moves.contains(&GreenLouseMove::Weaken));
    }

    #[test]
    fn test_choose_move_and_effects_attack() {
        let mut green_louse = GreenLouse::new(4, 15);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Force an attack by filling up with weaken moves
        green_louse.record_move(GreenLouseMove::Weaken);
        green_louse.record_move(GreenLouseMove::Weaken);
        green_louse.record_move(GreenLouseMove::Weaken);
        
        let (_move, effects) = green_louse.choose_move_and_effects(&global_info, &mut rng);
        
        // Should be forced to attack
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            BattleEffect::AttackToTarget { amount, num_attacks, strength_multiplier: 1 } => {
                assert_eq!(*amount, 4);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_choose_move_and_effects_records_moves() {
        let mut green_louse = GreenLouse::new(4, 15);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        let initial_move_count = green_louse.last_moves.len();
        let (_move, _effects) = green_louse.choose_move_and_effects(&global_info, &mut rng);
        
        // Should have recorded one more move
        assert_eq!(green_louse.last_moves.len(), initial_move_count + 1);
    }

    #[test]
    fn test_instantiate_hp_range() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Test multiple instantiations to ensure HP is in valid range
        for _ in 0..10 {
            let green_louse = GreenLouse::instantiate(&mut rng, &global_info);
            assert!(green_louse.hp >= 11);
            assert!(green_louse.hp <= 17);
        }
    }

    #[test]
    fn test_weak_effect_integration() {
        use crate::battle::character_battle_info::CharacterBattleInfo;
        
        // Test that Green Louse's weaken move applies weak status correctly
        let green_louse = GreenLouse::new(4, 15);
        let effects = green_louse.get_move_effects(GreenLouseMove::Weaken);
        
        // Create a mock character to apply the effect to
        let mut character = CharacterBattleInfo::new(50, 50, 3);
        assert!(!character.is_weak());
        
        // Apply the weak effect
        if let BattleEffect::ApplyWeak { duration: turns } = effects[0] {
            character.apply_weak(turns);
            assert!(character.is_weak());
            assert_eq!(character.get_weak_turns(), 2);
            
            // Test damage calculation with weak (25% reduction)
            assert_eq!(character.calculate_damage(8), 6); // 8 * 0.75 = 6
        }
    }
}