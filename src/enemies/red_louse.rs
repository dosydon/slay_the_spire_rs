use crate::{enemies::{enemy_enum::EnemyEnum, red_louse}, game::{effect::Effect, enemy::EnemyTrait, global_info::GlobalInfo}, utils::CategoricalDistribution};

pub struct RedLouse {
    curl_up_used: bool,
    last_moves: Vec<RedLouseMove>,
    base_damage: u32,
    hp: u32,
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum RedLouseMove {
    Attack,
    Grow,
}

impl RedLouse {
    pub fn new(base_damage: u32, hp: u32) -> Self {
        RedLouse { 
            curl_up_used: false,
            last_moves: Vec::new(),
            base_damage,
            hp,
        }
    }

    pub fn calculate_base_damage(global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> u32 {
        // Base damage is 5-7, +1 on Ascension 2+
        let base_damage_roll = 5 + rng.random_range(0..=2);
        let ascension_bonus = if global_info.ascention >= 2 { 1 } else { 0 };
        base_damage_roll + ascension_bonus
    }

    pub fn use_curl_up(&mut self) {
        self.curl_up_used = true;
    }

    fn get_valid_moves(&self) -> Vec<RedLouseMove> {
        let mut valid_moves = Vec::new();
        
        if !self.would_violate_consecutive_rule(RedLouseMove::Attack) {
            valid_moves.push(RedLouseMove::Attack);
        }
        
        if !self.would_violate_consecutive_rule(RedLouseMove::Grow) {
            valid_moves.push(RedLouseMove::Grow);
        }
        
        if valid_moves.is_empty() {
            vec![RedLouseMove::Attack]
        } else {
            valid_moves
        }
    }

    fn get_move_weights(&self, moves: &[RedLouseMove]) -> Vec<u32> {
        moves.iter().map(|&move_type| {
            match move_type {
                RedLouseMove::Attack => 75,
                RedLouseMove::Grow => 25,
            }
        }).collect()
    }

    fn would_violate_consecutive_rule(&self, move_type: RedLouseMove) -> bool {
        if self.last_moves.len() < 2 {
            return false;
        }
        
        let last_two: Vec<RedLouseMove> = self.last_moves.iter().rev().take(2).cloned().collect();
        last_two.iter().all(|&m| std::mem::discriminant(&m) == std::mem::discriminant(&move_type))
    }

    pub fn get_move_effects(&self, move_type: RedLouseMove) -> Vec<Effect> {
        match move_type {
            RedLouseMove::Attack => {
                vec![Effect::AttackToTarget { 
                    amount: self.base_damage, 
                    num_attacks: 1 
                }]
            }
            RedLouseMove::Grow => {
                vec![Effect::GainStrength(3)]
            }
        }
    }
}

impl EnemyTrait for RedLouse {
    type MoveType = RedLouseMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        // Calculate base damage using ascension scaling
        let base_damage = Self::calculate_base_damage(global_info, rng);
        // Create the enemy instance
        let hp = Self::hp_lb() + rng.random_range(0..=Self::hp_ub() - Self::hp_lb());
        let red_louse = RedLouse::new(base_damage, hp);

        red_louse
    }

    fn hp_lb() -> u32 {
        10
    }
    fn hp_ub() -> u32 {
        15
    }
    fn choose_next_move(&self, global_info: &GlobalInfo) -> CategoricalDistribution<Self::MoveType> {
        let possible_moves = self.get_valid_moves();
        let weights = self.get_move_weights(&possible_moves);
        
        let outcomes_and_weights: Vec<(RedLouseMove, f64)> = possible_moves
            .into_iter()
            .zip(weights.into_iter())
            .map(|(move_type, weight)| (move_type, weight as f64))
            .collect();
        
        CategoricalDistribution::new(outcomes_and_weights)
    }

    fn get_name() -> String {
        "Louse".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_louse_creation() {
        let louse = RedLouse::new(6, 12);
        assert!(!louse.curl_up_used);
        assert!(louse.last_moves.is_empty());
    }

    #[test]
    fn test_curl_up_usage() {
        let mut louse = RedLouse::new(6, 12);
        assert!(!louse.curl_up_used);
        
        louse.use_curl_up();
        assert!(louse.curl_up_used);
    }

    #[test]
    fn test_hp_bounds() {
        assert_eq!(RedLouse::hp_lb(), 10);
        assert_eq!(RedLouse::hp_ub(), 15);
        assert!(RedLouse::hp_lb() <= RedLouse::hp_ub());
    }

    #[test]
    fn test_name() {
        assert_eq!(RedLouse::get_name(), "Louse");
    }

    #[test]
    fn test_choose_next_move() {
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        
        let mut rng = StdRng::seed_from_u64(42);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let louse = RedLouse::new(6, 12);
        let move_dist = louse.choose_next_move(&global_info);
        let m = move_dist.sample_owned(&mut rng);
        
        assert!(matches!(m, RedLouseMove::Attack | RedLouseMove::Grow));
    }

    #[test]
    fn test_instantiate() {
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        
        let mut rng = StdRng::seed_from_u64(42);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let enemy = RedLouse::instantiate(&mut rng, &global_info);
        
        assert_eq!(RedLouse::get_name(), "Louse");
        assert!(enemy.hp >= RedLouse::hp_lb());
        assert!(enemy.hp <= RedLouse::hp_ub());
    }

    #[test]
    fn test_instantiate_hp_range() {
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        
        let mut hp_values = std::collections::HashSet::new();
        
        for seed in 0..100 {
            let mut rng = StdRng::seed_from_u64(seed);
            let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
            let enemy = RedLouse::instantiate(&mut rng, &global_info);
            hp_values.insert(enemy.hp);
        }
        
        for hp in hp_values {
            assert!(hp >= RedLouse::hp_lb());
            assert!(hp <= RedLouse::hp_ub());
        }
    }

    #[test]
    fn test_move_selection() {
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        
        let louse = RedLouse::new(6, 12);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = StdRng::seed_from_u64(42);
        
        // Use the trait method, not the internal implementation
        use crate::game::enemy::EnemyTrait;
        let move_dist = louse.choose_next_move(&global_info);
        let move1 = move_dist.sample_owned(&mut rng);
        assert!(matches!(move1, RedLouseMove::Attack | RedLouseMove::Grow));
        // Can't check last_moves.len() since trait method doesn't mutate state
    }

    #[test]
    fn test_consecutive_move_prevention() {
        let mut louse = RedLouse::new(6, 12);
        louse.last_moves = vec![RedLouseMove::Attack, RedLouseMove::Attack];
        
        let valid_moves = louse.get_valid_moves();
        assert!(!valid_moves.contains(&RedLouseMove::Attack));
        assert!(valid_moves.contains(&RedLouseMove::Grow));
    }

    #[test]
    fn test_move_effects_attack() {
        let louse = RedLouse::new(6, 12);
        let effects = louse.get_move_effects(RedLouseMove::Attack);
        
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::AttackToTarget { amount: 6, num_attacks: 1 }); 
    }

    #[test]
    fn test_ascension_damage_scaling() {
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        
        // Test base ascension (0) - should be 5-7 damage
        let mut rng = StdRng::seed_from_u64(42);
        let base_global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let base_damage = RedLouse::calculate_base_damage(&base_global_info, &mut rng);
        assert!(base_damage >= 5 && base_damage <= 7);
        
        // Test ascension 2+ - should be 6-8 damage (+1 bonus)
        let mut rng2 = StdRng::seed_from_u64(42); // Same seed for comparison
        let asc2_global_info = GlobalInfo { ascention: 2, current_floor: 1 };
        let asc2_damage = RedLouse::calculate_base_damage(&asc2_global_info, &mut rng2);
        assert!(asc2_damage >= 6 && asc2_damage <= 8);
        
        // With same seed, ascension 2+ should be exactly 1 more than base
        let mut rng3 = StdRng::seed_from_u64(123);
        let mut rng4 = StdRng::seed_from_u64(123);
        let base_dmg = RedLouse::calculate_base_damage(&GlobalInfo { ascention: 0, current_floor: 1 }, &mut rng3);
        let asc_dmg = RedLouse::calculate_base_damage(&GlobalInfo { ascention: 2, current_floor: 1 }, &mut rng4);
        assert_eq!(asc_dmg, base_dmg + 1);
    }

    #[test]
    fn test_categorical_distribution_move_selection() {
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        
        let louse = RedLouse::new(6, 12);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Get the categorical distribution
        use crate::game::enemy::EnemyTrait;
        let move_dist = louse.choose_next_move(&global_info);
        
        // Verify the distribution has the expected moves
        let outcomes = move_dist.outcomes();
        assert!(outcomes.contains(&RedLouseMove::Attack));
        assert!(outcomes.contains(&RedLouseMove::Grow));
        assert_eq!(outcomes.len(), 2);
        
        // Test that probabilities follow expected weights (Attack: 75, Grow: 25)
        let probabilities = move_dist.probabilities();
        assert!((probabilities[outcomes.iter().position(|&m| m == RedLouseMove::Attack).unwrap()] - 0.75).abs() < 1e-10);
        assert!((probabilities[outcomes.iter().position(|&m| m == RedLouseMove::Grow).unwrap()] - 0.25).abs() < 1e-10);
        
        // Sample many times to verify the distribution roughly matches expectations
        let mut rng = StdRng::seed_from_u64(42);
        let mut attack_count = 0;
        let mut grow_count = 0;
        let samples = 1000;
        
        for _ in 0..samples {
            match move_dist.sample_owned(&mut rng) {
                RedLouseMove::Attack => attack_count += 1,
                RedLouseMove::Grow => grow_count += 1,
            }
        }
        
        // With 75:25 ratio, we expect roughly 75% attack, 25% grow
        let attack_ratio = attack_count as f64 / samples as f64;
        let grow_ratio = grow_count as f64 / samples as f64;
        
        assert!(attack_ratio > 0.65 && attack_ratio < 0.85, "Attack ratio {:.2} should be around 0.75", attack_ratio);
        assert!(grow_ratio > 0.15 && grow_ratio < 0.35, "Grow ratio {:.2} should be around 0.25", grow_ratio);
        
        println!("Attack: {:.1}%, Grow: {:.1}%", attack_ratio * 100.0, grow_ratio * 100.0);
    }
}