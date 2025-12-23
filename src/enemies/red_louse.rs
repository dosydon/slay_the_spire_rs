use crate::{game::{effect::BattleEffect, enemy::EnemyTrait, global_info::GlobalInfo}, utils::CategoricalDistribution};
use crate::battle::{battle_events::{BattleEvent, EventListener}, target::Entity};

#[derive(Clone, Debug)]
pub struct RedLouse {
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

    pub fn record_move(&mut self, move_type: RedLouseMove) {
        self.last_moves.push(move_type);
        // Keep only the last 3 moves to prevent unbounded growth
        if self.last_moves.len() > 3 {
            self.last_moves.remove(0);
        }
    }

    pub fn get_move_effects(&self, move_type: RedLouseMove) -> Vec<BattleEffect> {
        match move_type {
            RedLouseMove::Attack => {
                vec![BattleEffect::AttackToTarget {
                    amount: self.base_damage,
                    num_attacks: 1,
                    strength_multiplier: 1
                }]
            }
            RedLouseMove::Grow => {
                vec![BattleEffect::GainStrength { amount: 3 }]
            }
        }
    }


    fn choose_next_move(&self, _global_info: &GlobalInfo) -> CategoricalDistribution<RedLouseMove> {
        let possible_moves = self.get_valid_moves();
        let weights = self.get_move_weights(&possible_moves);
        
        let outcomes_and_weights: Vec<(RedLouseMove, f64)> = possible_moves
            .into_iter()
            .zip(weights.into_iter())
            .map(|(move_type, weight)| (move_type, weight as f64))
            .collect();
        
        CategoricalDistribution::new(outcomes_and_weights)
    }

}

impl EnemyTrait for RedLouse {
    type MoveType = RedLouseMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        // Calculate base damage using ascension scaling
        let base_damage = Self::calculate_base_damage(global_info, rng);
        // Create the enemy instance
        let hp = 10 + rng.random_range(0..=5); // 10-15 HP range
        let red_louse = RedLouse::new(base_damage, hp);

        red_louse
    }

    fn get_name() -> String {
        "Louse".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (RedLouseMove, Vec<BattleEffect>) {
        let move_distribution = self.choose_next_move(global_info);
        let selected_move = move_distribution.sample_owned(rng);
        
        // Record the move for consecutive move tracking
        self.record_move(selected_move);
        
        // Generate the effects for this move
        let effects = self.get_move_effects(selected_move);
        
        (selected_move, effects)
    }
}

// CurlUpListener implementation for Louse enemies
#[derive(Debug)]
pub struct CurlUpListener {
    used: bool,
    block_amount: u32,
    owner: Entity,
}

impl CurlUpListener {
    pub(crate) fn new(owner: Entity, ascension_level: u32, rng: &mut impl rand::Rng) -> Self {
        let block_amount = match ascension_level {
            a if a >= 17 => rng.random_range(9..=12), // A17+ gives 9-12 block
            a if a >= 7 => rng.random_range(4..=8),   // A7-16 gives 4-8 block
            _ => rng.random_range(3..=7),             // A0-6 gives 3-7 block
        };

        CurlUpListener {
            used: false,
            block_amount,
            owner,
        }
    }
}

impl EventListener for CurlUpListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::DamageTaken { target, amount, .. }
                if *target == self.owner && *amount > 0 && !self.used => {
                self.used = true;
                vec![BattleEffect::GainDefense { amount: self.block_amount }]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        !self.used
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::game::PlayerRunState;

    use super::*;

    #[test]
    fn test_red_louse_creation() {
        let louse = RedLouse::new(6, 12);
        assert!(louse.last_moves.is_empty());
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
        assert!(enemy.hp >= 10);
        assert!(enemy.hp <= 15);
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
            assert!(hp >= 10);
            assert!(hp <= 15);
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
        assert_eq!(effects[0], BattleEffect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }); 
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

    #[test]
    fn test_choose_move_and_effects_records_moves() {
        let mut louse = RedLouse::new(6, 12);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        assert!(louse.last_moves.is_empty());
        
        // Choose move and effects should record the selected move
        let (_move, effects) = louse.choose_move_and_effects(&global_info, &mut rng);
        
        // Should have recorded one move
        assert_eq!(louse.last_moves.len(), 1);
        
        // Effects should match the recorded move
        let recorded_move = louse.last_moves[0];
        let expected_effects = louse.get_move_effects(recorded_move);
        assert_eq!(effects, expected_effects);
    }

    #[test]
    fn test_choose_move_and_effects_respects_consecutive_rule() {
        let mut louse = RedLouse::new(6, 12);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Force two consecutive attacks by manipulating the last_moves
        louse.last_moves = vec![RedLouseMove::Attack, RedLouseMove::Attack];
        
        // Choose effects - should not get three attacks in a row
        let (_move, effects) = louse.choose_move_and_effects(&global_info, &mut rng);
        
        // Should have chosen Grow (since Attack would violate consecutive rule)
        assert_eq!(effects, vec![BattleEffect::GainStrength { amount: 3 }]);
        assert_eq!(louse.last_moves.last().unwrap(), &RedLouseMove::Grow);
    }

    #[test]
    fn test_curl_up_listener_creation() {
        let mut rng = rand::rng();
        let listener = CurlUpListener::new(Entity::Enemy(0), 0, &mut rng);
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Enemy(0));
        // Block amount should be in range 3-7 for ascension 0
        assert!(listener.block_amount >= 3 && listener.block_amount <= 7);
    }

    #[test]
    fn test_curl_up_listener_ascension() {
        let mut rng = rand::rng();
        let normal_listener = CurlUpListener::new(Entity::Enemy(0), 0, &mut rng);
        let mid_ascension_listener = CurlUpListener::new(Entity::Enemy(0), 10, &mut rng);
        let high_ascension_listener = CurlUpListener::new(Entity::Enemy(0), 17, &mut rng);

        // Test ascension scaling ranges
        assert!(normal_listener.block_amount >= 3 && normal_listener.block_amount <= 7);
        assert!(mid_ascension_listener.block_amount >= 4 && mid_ascension_listener.block_amount <= 8);
        assert!(high_ascension_listener.block_amount >= 9 && high_ascension_listener.block_amount <= 12);
    }

    #[test]
    fn test_curl_up_triggers_on_damage() {
        let mut rng = rand::rng();
        let mut listener = CurlUpListener::new(Entity::Enemy(0), 0, &mut rng);
        let expected_block = listener.block_amount; // Store the randomly generated amount

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 6,
            source: Entity::Player,
        };

        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], BattleEffect::GainDefense { amount: expected_block });
        assert!(!listener.is_active()); // Used up
    }

    #[test]
    fn test_curl_up_only_triggers_once() {
        let mut rng = rand::rng();
        let mut listener = CurlUpListener::new(Entity::Enemy(0), 0, &mut rng);

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 6,
            source: Entity::Player,
        };

        // First damage triggers curl up
        listener.on_event(&damage_event);

        // Second damage should not trigger
        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_curl_up_wrong_target() {
        let mut rng = rand::rng();
        let mut listener = CurlUpListener::new(Entity::Enemy(0), 0, &mut rng);

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(1), // Different target
            amount: 6,
            source: Entity::Player,
        };

        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active()); // Still active
    }

    #[test]
    fn test_curl_up_zero_damage() {
        let mut rng = rand::rng();
        let mut listener = CurlUpListener::new(Entity::Enemy(0), 0, &mut rng);

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 0, // No damage
            source: Entity::Player,
        };

        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active()); // Still active
    }

    #[test]
    fn test_choose_move_and_effects_attack() {
        let mut louse = RedLouse::new(8, 12);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Force the louse to choose attack by making grow invalid
        louse.last_moves = vec![RedLouseMove::Grow, RedLouseMove::Grow];
        
        let (_move, effects) = louse.choose_move_and_effects(&global_info, &mut rng);
        
        // Should have chosen Attack
        assert_eq!(effects, vec![BattleEffect::AttackToTarget { amount: 8, num_attacks: 1, strength_multiplier: 1 }]);
        assert_eq!(louse.last_moves.last().unwrap(), &RedLouseMove::Attack);
    }

    #[test] 
    fn test_choose_move_and_effects_grow() {
        let mut louse = RedLouse::new(6, 12);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Force the louse to choose grow by making attack invalid
        louse.last_moves = vec![RedLouseMove::Attack, RedLouseMove::Attack];
        
        let (_move, effects) = louse.choose_move_and_effects(&global_info, &mut rng);
        
        // Should have chosen Grow
        assert_eq!(effects, vec![BattleEffect::GainStrength { amount: 3 }]);
        assert_eq!(louse.last_moves.last().unwrap(), &RedLouseMove::Grow);
    }

    #[test]
    fn test_red_louse_two_turn_battle_fixed_hand() {
        use crate::{battle::Battle, battle::battle_action::BattleAction, battle::target::Entity, game::deck::Deck, battle::BattleResult};
        use crate::cards::ironclad::{strike::strike, defend::defend, bash::bash};
        
        // Create a deck with specific card order for initial hand
        // Since draw_card() takes from index 0, we put desired hand cards at the beginning
        let deck_cards = vec![
            // These will be the initial hand (first 5 cards, drawn in order)
            strike(), strike(), strike(), defend(), defend(),
            // Remaining cards in deck
            strike(), strike(), bash(), defend(), defend(),
        ];
        
        let deck = Deck::new(deck_cards);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![crate::battle::enemy_in_battle::EnemyInBattle::new(crate::enemies::enemy_enum::EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), enemies, &mut rng);

        println!("=== FIXED HAND BATTLE TEST ===");
        println!("Initial hand:");
        for (i, card) in battle.get_hand().iter().enumerate() {
            println!("  {}: {} (cost: {})", i, card.get_name(), card.get_cost());
        }

        // Verify we have the expected hand (first 5 cards from deck: Strike, Strike, Strike, Strike, Defend)
        let hand = battle.get_hand();
        assert!(hand.len() >= 3);
        assert_eq!(hand[0].get_name(), "Strike");
        assert_eq!(hand[1].get_name(), "Strike"); 
        assert_eq!(hand[2].get_name(), "Strike");
        
        let initial_player_hp = battle.get_player().battle_info.get_hp();
        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        
        println!("Initial state - Player HP: {}, Enemy HP: {}, Player Energy: {}", 
            initial_player_hp, initial_enemy_hp, battle.get_player().get_energy());
        
        // === TURN 1: PLAYER ===
        println!("\n--- Turn 1: Player ---");
        
        // Play first Strike (card index 0, targeting enemy)
        println!("Playing first Strike targeting enemy");
        let action = BattleAction::PlayCard(0, Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng).expect("Action should succeed");
        assert!(matches!(result, BattleResult::Continued(_)));
        println!("After first Strike - Enemy HP: {}, Player Energy: {}", 
            battle.get_enemies()[0].battle_info.get_hp(), battle.get_player().get_energy());
        
        let enemy_block_after_first_strike = battle.get_enemies()[0].battle_info.get_block();
        println!("Enemy block after first Strike: {} (Curl Up should have activated)", enemy_block_after_first_strike);
        
        // Play second Strike (now at index 0 since first was removed, targeting enemy) 
        println!("Playing second Strike targeting enemy");
        let action = BattleAction::PlayCard(0, Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng).expect("Action should succeed");
        assert!(matches!(result, BattleResult::Continued(_)));
        println!("After second Strike - Enemy HP: {}, Player Energy: {}", 
            battle.get_enemies()[0].battle_info.get_hp(), battle.get_player().get_energy());
        
        // Play one Defend to use remaining energy (now at index 1, targeting self)
        println!("Playing one Defend");
        let action = BattleAction::PlayCard(1, Entity::Player);
        let result = battle.eval_action(action, &mut rng).expect("Action should succeed");
        assert!(matches!(result, BattleResult::Continued(_)));
        println!("After Defend - Player Block: {}, Player Energy: {}", 
            battle.get_player().get_block(), battle.get_player().get_energy());
        
        let enemy_hp_after_turn1 = battle.get_enemies()[0].battle_info.get_hp();
        let player_damage_dealt = initial_enemy_hp - enemy_hp_after_turn1;
        let curl_up_block = battle.get_enemies()[0].battle_info.get_block();
        
        println!("Turn 1 complete - Total damage dealt to enemy: {}, Player block: {}, Enemy block (Curl Up): {}", 
            player_damage_dealt, battle.get_player().get_block(), curl_up_block);
        
        // Verify expected outcomes for turn 1
        // First strike: 6 damage, triggers Curl Up for 3-7 block
        // Second strike: 6 damage attempted, but some blocked by Curl Up
        // Note: curl_up_block is 0 here because block gets reset at end of turn, 
        // but we can calculate from the damage dealt
        let expected_damage_first_strike = 6;
        let actual_curl_up_block = enemy_block_after_first_strike; // We captured this earlier
        let expected_damage_second_strike = if actual_curl_up_block >= 6 { 0 } else { 6 - actual_curl_up_block };
        let expected_total_damage = expected_damage_first_strike + expected_damage_second_strike;
        
        assert_eq!(player_damage_dealt, expected_total_damage, "Two Strikes with Curl Up blocking");
        assert_eq!(battle.get_player().get_block(), 5, "One Defend should give 5 block");
        assert_eq!(battle.get_player().get_energy(), 0, "Should have spent all 3 energy");
        assert!(actual_curl_up_block >= 3 && actual_curl_up_block <= 7, "Curl Up should give 3-7 block for ascension 0");
        
        // === TURN 1: ENEMY ===
        println!("\n--- Turn 1: Enemy ---");
        
        battle.sample_enemy_actions(&mut rng);
        battle.process_enemy_effects(&mut rng, &global_info);
        battle.at_end_of_enemy_turn();
        
        // End of turn 1 - start new player turn
        battle.at_start_of_player_turn(&mut rng);
        
        println!("End of Turn 1 - Player HP: {}, Enemy HP: {}, Enemy Strength: {}", 
            battle.get_player().battle_info.get_hp(), 
            battle.get_enemies()[0].battle_info.get_hp(),
            battle.get_enemies()[0].battle_info.get_strength());
        
        // === TURN 2: PLAYER ===
        println!("\n--- Turn 2: Player ---");
        
        // Play one Strike (at index 0)
        let cards_in_hand = battle.get_hand().len();
        println!("Cards in hand: {}", cards_in_hand);
        
        println!("Playing Strike (cost: 1)");
        let action = BattleAction::PlayCard(0, Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng).expect("Action should succeed");
        // Enemy might die from this strike, so it could be Won or Continued
        assert!(matches!(result, BattleResult::Won(_) | BattleResult::Continued(_)));
        println!("Turn 2 Player: 1 Strike played, result: {:?}", result);
        
        // === TURN 2: ENEMY ===
        battle.sample_enemy_actions(&mut rng);
        battle.process_enemy_effects(&mut rng, &global_info);
        battle.at_end_of_enemy_turn();
        
        // Verify battle mechanics worked as expected
        assert!(battle.get_player().is_alive(), "Player should still be alive after 2 turns");
        
        // Verify damage was dealt during the battle
        let total_damage = (initial_player_hp - battle.get_player().battle_info.get_hp()) +
                          (initial_enemy_hp - battle.get_enemies()[0].battle_info.get_hp());
        assert!(total_damage > 0, "Some damage should have been dealt during the battle");
        
        println!("Fixed hand battle test completed successfully!");
    }
}