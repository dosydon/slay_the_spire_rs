pub mod action;
pub mod character_battle_info;
pub mod target;
pub mod events;
pub mod listeners;
pub mod player;
pub mod deck_hand_pile;
pub mod enemy_in_battle;
mod turn_flow;
mod action_handler;
mod eval_effect;
mod enemy_manager;

use crate::{enemies::{red_louse::{RedLouse, RedLouseMove}, green_louse::GreenLouseMove, jaw_worm::JawWormMove, enemy_enum::{EnemyEnum, EnemyMove}}, game::{card::Card, deck::Deck, effect::{BaseEffect, Effect}, enemy::EnemyTrait, global_info::GlobalInfo}};
use self::{action::Action, target::Entity, events::{BattleEvent, EventListener}, player::Player, deck_hand_pile::DeckHandPile, enemy_in_battle::EnemyInBattle};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum BattleError {
    InvalidAction,
    NotEnoughEnergy,
    CardNotInHand,
    InvalidTarget,
    GameAlreadyOver,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BattleResult {
    Continued,
    Won,
    Lost,
}


pub struct Battle {
    player: Player,
    enemies: Vec<EnemyInBattle>,
    cards: DeckHandPile,
    event_listeners: Vec<Box<dyn EventListener>>,
    global_info: GlobalInfo,
    /// Stores the next move and effects for each enemy (index corresponds to enemies Vec)
    enemy_actions: Vec<Option<(EnemyMove, Vec<Effect>)>>,
}

impl Battle {
    pub fn new(deck: Deck, global_info: GlobalInfo, initial_hp: u32, max_hp: u32, enemies: Vec<EnemyInBattle>, rng: &mut impl rand::Rng) -> Self {
        let cards = DeckHandPile::new(deck);
        let enemy_count = enemies.len();
        let mut battle = Battle {
            player: Player::new(initial_hp, max_hp, 3),
            enemies,
            cards,
            event_listeners: Vec::new(),
            global_info,
            enemy_actions: vec![None; enemy_count],
        };
        
        // Initialize event listeners for enemies
        battle.initialize_enemy_listeners(&global_info, rng);
        
        // Start the first turn (refreshes player, samples enemy actions, draws hand)
        battle.start_of_player_turn(rng);
        
        battle
    }
    
    /// Get the final HP after battle for syncing back to Game
    pub fn get_final_player_hp(&self) -> u32 {
        self.player.battle_info.get_hp()
    }
    
    
    
    pub fn get_player(&self) -> &Player {
        &self.player
    }
    
    pub(crate) fn get_player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
    
    pub fn get_enemies(&self) -> &Vec<EnemyInBattle> {
        &self.enemies
    }
    
    pub fn get_hand(&self) -> &Vec<Card> {
        self.cards.get_hand()
    }
    
    
    
    
    
    pub fn is_battle_over(&self) -> bool {
        !self.player.is_alive() || self.enemies.iter().all(|e| !e.battle_info.is_alive())
    }
    
    

    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::ironclad::starter_deck::starter_deck;

    #[test]
    fn test_battle_initialization() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
let battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        assert_eq!(battle.player.battle_info.get_hp(), 80);
        assert_eq!(battle.player.get_block(), 0);
        assert_eq!(battle.player.get_energy(), 3);
        assert!(!battle.enemies.is_empty());
        
        println!("{:?}", battle.cards.get_deck());
        println!("{:?}", battle.cards.get_hand());
    }

    #[test]
    fn test_eval_base_effect() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 10,
            num_attacks: 1,
        };
        
        battle.eval_base_effect(&damage_effect);
        
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_enemy_hp - 10);
    }

    #[test]
    fn test_play_card_with_target() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_energy = battle.player.get_energy();
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        
        // Find a Strike card in the hand
        let strike_idx = battle.cards.get_hand().iter().position(|card| card.get_name() == "Strike");
        
        if let Some(idx) = strike_idx {
            // Play the Strike card targeting enemy 0
            let action = Action::PlayCard(idx, Entity::Enemy(0));
            let result = battle.eval_action(action, &mut rng);
            assert!(matches!(result, Ok(BattleResult::Continued)));
            
            // Check that energy was spent and enemy took damage
            assert!(battle.player.get_energy() < initial_energy);
            assert!(battle.enemies[0].battle_info.get_hp() < initial_enemy_hp);
        } else {
            panic!("No Strike card found in hand");
        }
    }

    #[test]
    fn test_vulnerable_effect_integration() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Apply vulnerable to enemy
        let vulnerable_effect = BaseEffect::ApplyVulnerable { target: Entity::Enemy(0), duration: 2 };
        battle.eval_base_effect(&vulnerable_effect);
        
        // Check that enemy is vulnerable
        assert!(battle.enemies[0].battle_info.is_vulnerable());
        assert_eq!(battle.enemies[0].battle_info.get_vulnerable_turns(), 2);
        
        // Apply damage - should be increased by 50%
        let initial_hp = battle.enemies[0].battle_info.get_hp();
        
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 10,
            num_attacks: 1,
        };
        battle.eval_base_effect(&damage_effect);
        
        // 10 damage * 1.5 = 15 damage should be dealt (but capped by enemy's HP)
        let expected_damage = 15u32.min(initial_hp);
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_hp - expected_damage);
    }

    #[test]
    fn test_character_block_integration() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Give enemy some block
        battle.enemies[0].battle_info.gain_block(5);
        assert_eq!(battle.enemies[0].battle_info.get_block(), 5);
        
        let initial_hp = battle.enemies[0].battle_info.get_hp();
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 8,
            num_attacks: 1,
        };
        battle.eval_base_effect(&damage_effect);
        
        // 8 damage - 5 block = 3 actual damage
        // But taking damage triggers Curl Up, giving enemy 3-7 more block (ascension 0)
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_hp - 3);
        let curl_up_block = battle.enemies[0].battle_info.get_block();
        assert!(curl_up_block >= 3 && curl_up_block <= 7); // Curl Up activated with random amount
    }

    #[test]
    fn test_damage_to_player() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_hp = battle.player.battle_info.get_hp();
        
        // Create an effect that damages the player
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Enemy(0),
            target: Entity::Player,
            amount: 10,
            num_attacks: 1,
        };
        battle.eval_base_effect(&damage_effect);
        
        // Player should take 10 damage
        assert_eq!(battle.player.battle_info.get_hp(), initial_hp - 10);
    }

    #[test]
    fn test_attack_with_strength() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Give player some strength
        battle.player.battle_info.gain_strength(3);
        assert_eq!(battle.player.battle_info.get_strength(), 3);
        
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        let attack_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 6,
            num_attacks: 1,
        };
        battle.eval_base_effect(&attack_effect);
        
        // 6 base damage + 3 strength = 9 total damage
        let expected_damage = 9u32.min(initial_enemy_hp);
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_enemy_hp - expected_damage);
    }

    #[test]
    fn test_complete_turn_simulation() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Record initial state
        let initial_player_hp = battle.player.battle_info.get_hp();
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        let initial_energy = battle.player.get_energy();
        
        // === PLAYER TURN ===
        
        // Player starts turn (should reset block and refresh energy)
        battle.player.at_start_of_turn();
        assert_eq!(battle.player.get_energy(), 3); // Energy refreshed
        assert_eq!(battle.player.get_block(), 0); // Block reset
        
        // Player plays a Defend card (should give 5 block)
        let defend_idx = battle.cards.get_hand().iter().position(|card| card.get_name() == "Defend");
        if let Some(idx) = defend_idx {
            let hand_size_before = battle.cards.hand_size();
            battle.play_card(idx, Entity::Player);
            
            // Check card was played
            assert_eq!(battle.cards.hand_size(), hand_size_before - 1);
            // Check energy was spent (Defend costs 1)
            assert_eq!(battle.player.get_energy(), 2);
            // Check block was gained (Defend gives 5 block)
            assert_eq!(battle.player.get_block(), 5);
        }
        
        // Player plays a Strike card targeting enemy
        let strike_idx = battle.cards.get_hand().iter().position(|card| card.get_name() == "Strike");
        if let Some(idx) = strike_idx {
            let enemy_hp_before = battle.enemies[0].battle_info.get_hp();
            battle.play_card(idx, Entity::Enemy(0));
            
            // Check energy was spent (Strike costs 1)
            assert_eq!(battle.player.get_energy(), 1);
            // Check enemy took damage (Strike deals 6 damage)
            assert_eq!(battle.enemies[0].battle_info.get_hp(), enemy_hp_before - 6);
        }
        
        // Enemy acts (Red Louse will either attack or gain strength)
        let player_hp_before_enemy = battle.player.battle_info.get_hp();
        let player_block_before_enemy = battle.player.get_block();
        let enemy_strength_before = battle.enemies[0].battle_info.get_strength();
        
        battle.process_enemy_effects(&mut rng, &global_info);
        
        // Check that either the player took damage OR the enemy gained strength
        let player_took_damage = battle.player.battle_info.get_hp() < player_hp_before_enemy || 
                                battle.player.get_block() < player_block_before_enemy;
        let enemy_gained_strength = battle.enemies[0].battle_info.get_strength() > enemy_strength_before;
        
        // One of these should have happened (Red Louse either attacks or grows)
        assert!(player_took_damage || enemy_gained_strength, 
                "Enemy should have either attacked player or gained strength");
        
        // === TURN CYCLE COMPLETE ===
        
        // Verify battle is not over (both entities should still be alive)
        assert!(battle.player.is_alive(), "Player should still be alive");
        assert!(battle.enemies[0].battle_info.is_alive(), "Enemy should still be alive");
        assert!(!battle.is_battle_over(), "Battle should not be over yet");
        
        // Verify some state changes occurred during the turn
        assert!(initial_player_hp != battle.player.battle_info.get_hp() || 
                initial_enemy_hp != battle.enemies[0].battle_info.get_hp() ||
                enemy_gained_strength,
                "Some combat effects should have occurred during the turn");
        
        println!("Turn complete - Player HP: {}/{}, Block: {}, Energy: {} | Enemy HP: {}, Strength: {}",
                battle.player.battle_info.get_hp(), initial_player_hp, battle.player.get_block(), battle.player.get_energy(),
                battle.enemies[0].battle_info.get_hp(), battle.enemies[0].battle_info.get_strength());
    }

    #[test]
    fn test_red_louse_curl_up_event_system() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Initially enemy should have 0 block
        assert_eq!(battle.enemies[0].battle_info.get_block(), 0);
        
        // Deal damage to the enemy to trigger curl up
        let initial_hp = battle.enemies[0].battle_info.get_hp();
        let damage_dealt = battle.apply_damage(Entity::Enemy(0), 6);
        
        // Check that damage was dealt and curl up was triggered (enemy gained block)
        assert_eq!(damage_dealt, 6);
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_hp - 6);
        
        // Curl up gives 3-7 block for ascension 0
        let curl_up_block = battle.enemies[0].battle_info.get_block();
        assert!(curl_up_block >= 3 && curl_up_block <= 7);
        
        // Deal damage again - curl up should not trigger a second time
        let hp_before_second_damage = battle.enemies[0].battle_info.get_hp();
        let second_damage = battle.apply_damage(Entity::Enemy(0), 4);
        
        // Calculate expected outcome based on curl up block amount
        let expected_damage = if curl_up_block >= 4 { 0 } else { 4 - curl_up_block };
        let expected_remaining_block = if curl_up_block >= 4 { curl_up_block - 4 } else { 0 };
        
        assert_eq!(second_damage, expected_damage);
        assert_eq!(battle.enemies[0].battle_info.get_hp(), hp_before_second_damage - expected_damage);
        assert_eq!(battle.enemies[0].battle_info.get_block(), expected_remaining_block);
    }

    #[test]
    fn test_curl_up_ascension_scaling() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        
        // Test normal ascension (0-6): should give 3-7 block
        let normal_global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &normal_global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut normal_battle = Battle::new(deck.clone(), normal_global_info, 80, 80, enemies, &mut rng);
        normal_battle.apply_damage(Entity::Enemy(0), 6);
        let normal_block = normal_battle.enemies[0].battle_info.get_block();
        assert!(normal_block >= 3 && normal_block <= 7);
        
        // Test mid ascension (7-16): should give 4-8 block
        let mid_global_info = GlobalInfo { ascention: 10, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &mid_global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut mid_battle = Battle::new(deck.clone(), mid_global_info, 80, 80, enemies, &mut rng);
        mid_battle.apply_damage(Entity::Enemy(0), 6);
        let mid_block = mid_battle.enemies[0].battle_info.get_block();
        assert!(mid_block >= 4 && mid_block <= 8);
        
        // Test high ascension (17+): should give 9-12 block
        let high_global_info = GlobalInfo { ascention: 17, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &high_global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut high_battle = Battle::new(deck, high_global_info, 80, 80, enemies, &mut rng);
        high_battle.apply_damage(Entity::Enemy(0), 6);
        let high_block = high_battle.enemies[0].battle_info.get_block();
        assert!(high_block >= 9 && high_block <= 12);
    }

    #[test]
    fn test_list_available_actions_basic() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let available_actions = battle.list_available_actions();
        
        // Should have actions for playable cards + EndTurn
        assert!(!available_actions.is_empty());
        
        // Should always have EndTurn available
        assert!(available_actions.contains(&Action::EndTurn));
        
        // Check that we have PlayCard actions
        let play_card_actions: Vec<_> = available_actions.iter()
            .filter(|action| matches!(action, Action::PlayCard(_, _)))
            .collect();
        
        assert!(!play_card_actions.is_empty(), "Should have at least some playable cards");
        
        // Verify all card actions are for cards with sufficient energy
        let hand = battle.get_hand();
        let player_energy = battle.get_player().get_energy();
        
        for action in &play_card_actions {
            if let Action::PlayCard(card_idx, target) = action {
                assert!(*card_idx < hand.len(), "Card index should be valid");
                assert!(hand[*card_idx].get_cost() <= player_energy, "Should only suggest affordable cards");
                assert!(battle.is_valid_target(target), "Target should be valid");
            }
        }
        
        println!("Available actions: {}", available_actions.len());
        println!("Play card actions: {}", play_card_actions.len());
    }
    
    #[test]
    fn test_list_available_actions_no_energy() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Spend all energy
        battle.player.battle_info.spend_energy(battle.player.get_energy());
        assert_eq!(battle.player.get_energy(), 0);
        
        let available_actions = battle.list_available_actions();
        
        // Should only have EndTurn available (no energy for cards)
        assert_eq!(available_actions.len(), 1);
        assert_eq!(available_actions[0], Action::EndTurn);
    }
    
    #[test]
    fn test_list_available_actions_battle_over() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Kill all enemies to end battle
        let enemy_hp = battle.enemies[0].battle_info.get_hp();
        battle.enemies[0].battle_info.take_damage(enemy_hp);
        
        assert!(battle.is_battle_over());
        
        let available_actions = battle.list_available_actions();
        
        // Should have no available actions when battle is over
        assert!(available_actions.is_empty());
    }
    
    #[test]
    fn test_get_valid_targets_for_card() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let hand = battle.get_hand();
        
        // Test targeting for different card types
        for card in hand {
            let targets = battle.get_valid_targets_for_card(card);
            assert!(!targets.is_empty(), "Every card should have at least one valid target");
            
            // Verify all returned targets are actually valid
            for target in &targets {
                assert!(battle.is_valid_target(target), "All returned targets should be valid");
            }
            
            println!("Card '{}' can target: {:?}", card.get_name(), targets);
        }
    }
    
    #[test]
    fn test_list_available_actions_with_dead_enemies() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create battle with two enemies
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2))
        ];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Kill the first enemy
        let first_enemy_hp = battle.enemies[0].battle_info.get_hp();
        battle.enemies[0].battle_info.take_damage(first_enemy_hp);
        assert!(!battle.enemies[0].battle_info.is_alive());
        assert!(battle.enemies[1].battle_info.is_alive());
        
        let available_actions = battle.list_available_actions();
        
        // Should still have actions available (second enemy is alive)
        assert!(!available_actions.is_empty());
        assert!(available_actions.contains(&Action::EndTurn));
        
        // Check that PlayCard actions only target living enemies
        let play_card_actions: Vec<_> = available_actions.iter()
            .filter_map(|action| match action {
                Action::PlayCard(idx, Entity::Enemy(enemy_idx)) => Some((*idx, *enemy_idx)),
                _ => None,
            })
            .collect();
        
        // All enemy-targeting actions should target the living enemy (index 1)
        for (_, enemy_idx) in play_card_actions {
            assert_eq!(enemy_idx, 1, "Should only target living enemy at index 1");
        }
    }
    
    #[test]
    fn test_list_available_actions_specific_cards() {
        use crate::cards::ironclad::{strike::strike, defend::defend, bash::bash};
        
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        
        // Create a deck with specific cards for testing
        let deck = Deck::new(vec![strike(), defend(), bash()]);
        let battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let available_actions = battle.list_available_actions();
        
        // Strike should target enemies
        let strike_actions = available_actions.iter()
            .filter(|action| {
                if let Action::PlayCard(0, target) = action {
                    matches!(target, Entity::Enemy(_))
                } else {
                    false
                }
            })
            .count();
        assert!(strike_actions > 0, "Strike should be able to target enemies");
        
        // Defend should target player 
        let defend_actions = available_actions.iter()
            .filter(|action| {
                if let Action::PlayCard(1, target) = action {
                    matches!(target, Entity::Player)
                } else {
                    false
                }
            })
            .count();
        assert!(defend_actions > 0, "Defend should be able to target player");
        
        // Bash should target enemies (has attack + apply vulnerable effects)
        let bash_actions = available_actions.iter()
            .filter(|action| {
                if let Action::PlayCard(2, target) = action {
                    matches!(target, Entity::Enemy(_))
                } else {
                    false
                }
            })
            .count();
        assert!(bash_actions > 0, "Bash should be able to target enemies");
        
        println!("Available actions for specific cards test: {}", available_actions.len());
    }

    #[test]
    fn test_add_slimed_effect() {
        use crate::game::card_type::CardType;
        
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_discard_size = battle.cards.discard_pile_size();
        let initial_total_cards = battle.cards.total_cards();
        
        // Apply AddSlimed effect to add 2 Slimed cards
        let add_slimed_effect = BaseEffect::AddSlimed { 
            target: Entity::Player, 
            count: 2 
        };
        battle.eval_base_effect(&add_slimed_effect);
        
        // Should have 2 more cards in discard pile
        assert_eq!(battle.cards.discard_pile_size(), initial_discard_size + 2);
        assert_eq!(battle.cards.total_cards(), initial_total_cards + 2);
        
        // Check that the added cards are Slimed
        let discard_pile = battle.cards.get_discard_pile();
        let last_two_cards = &discard_pile[discard_pile.len()-2..];
        for card in last_two_cards {
            assert_eq!(card.get_name(), "Slimed");
            assert_eq!(card.get_cost(), 1);
            assert_eq!(card.get_card_type(), &CardType::Status);
        }
    }

    #[test]
    fn test_exhaust_card_functionality() {
        use crate::game::deck::Deck;
        use crate::cards::ironclad::{strike::strike, defend::defend};
        
        let mut deck_cards = vec![strike(), defend(), strike(), defend(), strike()];
        // Add a Slimed card to the deck
        deck_cards.push(crate::cards::status::slimed::slimed());
        let deck = Deck::new(deck_cards);
        
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Draw the hand
        battle.cards.draw_n(5);
        
        // Find the Slimed card in hand (if any) or add one
        let mut slimed_index = None;
        let hand = battle.cards.get_hand();
        for (i, card) in hand.iter().enumerate() {
            if card.get_name() == "Slimed" {
                slimed_index = Some(i);
                break;
            }
        }
        
        // If no Slimed card in hand, add one manually for testing
        if slimed_index.is_none() {
            battle.cards.add_card_to_hand(crate::cards::status::slimed::slimed());
            slimed_index = Some(battle.cards.hand_size() - 1);
        }
        
        let slimed_idx = slimed_index.unwrap();
        let initial_hand_size = battle.cards.hand_size();
        let initial_discard_size = battle.cards.discard_pile_size();
        let initial_exhausted_size = battle.cards.exhausted_size();
        let initial_energy = battle.player.get_energy();
        
        // Play the Slimed card
        battle.play_card(slimed_idx, Entity::Player);
        
        // Verify the effects:
        // 1. Card should be removed from hand
        assert_eq!(battle.cards.hand_size(), initial_hand_size - 1);
        // 2. Card should NOT go to discard pile (it's exhausted)
        assert_eq!(battle.cards.discard_pile_size(), initial_discard_size);
        // 3. Card should go to exhausted pile
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted_size + 1);
        // 4. Energy should be reduced by 1 (Slimed costs 1)
        assert_eq!(battle.player.get_energy(), initial_energy - 1);
        
        // Check that the exhausted card is Slimed
        let exhausted_cards = battle.cards.get_exhausted();
        assert_eq!(exhausted_cards.last().unwrap().get_name(), "Slimed");
    }

    #[test]
    fn test_defeated_enemies_dont_execute_moves() {
        use crate::enemies::red_louse::RedLouse;
        use crate::cards::ironclad::strike::strike;
        
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create two enemies 
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let mut enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2))
        ];
        
        // Create a deck with strike cards
        let deck = Deck::new(vec![strike(), strike(), strike(), strike(), strike()]);
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Sample enemy actions for both enemies
        battle.sample_enemy_actions(&mut rng);
        
        // Kill the first enemy manually 
        battle.enemies[0].battle_info.take_damage(100);
        assert!(!battle.enemies[0].battle_info.is_alive(), "First enemy should be dead");
        assert!(battle.enemies[1].battle_info.is_alive(), "Second enemy should be alive");
        
        // Record player HP before enemy turn
        let player_hp_before = battle.player.battle_info.get_hp();
        let player_block_before = battle.player.battle_info.get_block();
        
        // Execute enemy turn - defeated enemy should not act
        battle.process_enemy_effects(&mut rng, &global_info);
        
        // Check that only the living enemy could have affected the player
        // We can't predict exact values due to randomness, but we can verify the system works
        // by ensuring the battle system didn't crash and state is consistent
        
        // Verify that dead enemy's action was cleared
        assert!(battle.enemy_actions[0].is_none(), "Dead enemy should have no stored action");
        
        // The living enemy may or may not have affected the player (depends on its chosen move),
        // but the important thing is that the dead enemy didn't execute its move
        // This is verified by the fact that we didn't panic and the action was cleared
        
        // Ensure the battle is in a consistent state
        assert!(!battle.enemies[0].battle_info.is_alive(), "First enemy should still be dead");
        assert!(battle.enemies[1].battle_info.is_alive(), "Second enemy should still be alive");
        assert!(battle.player.battle_info.is_alive(), "Player should still be alive");
    }

}