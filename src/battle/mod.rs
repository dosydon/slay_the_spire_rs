pub mod action;
pub mod character_battle_info;
pub mod target;
pub mod events;
pub mod listeners;
pub mod player;
pub mod deck_hand_pile;
pub mod enemy_in_battle;

use crate::{enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum}, game::{card::Card, deck::Deck, effect::BaseEffect, enemy::EnemyTrait, global_info::GlobalInfo}};
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
}

impl Battle {
    pub fn new(deck: Deck, global_info: GlobalInfo, initial_hp: u32, max_hp: u32, enemies: Vec<EnemyInBattle>, rng: &mut impl rand::Rng) -> Self {
        let cards = DeckHandPile::new(deck);
        let mut battle = Battle {
            player: Player::new(initial_hp, max_hp, 3),
            enemies,
            cards,
            event_listeners: Vec::new(),
            global_info,
        };
        
        // Initialize event listeners for enemies
        battle.initialize_enemy_listeners(&global_info, rng);
        battle
    }
    
    /// Get the final HP after battle for syncing back to Game
    pub fn get_final_player_hp(&self) -> u32 {
        self.player.battle_info.get_hp()
    }
    
    /// Initialize event listeners for enemies based on their type
    fn initialize_enemy_listeners(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) {
        use crate::battle::listeners::CurlUpListener;
        
        for (i, enemy) in self.enemies.iter().enumerate() {
            match &enemy.enemy {
                EnemyEnum::RedLouse(_) => {
                    // Red Louse gets a curl up listener with randomly generated block amount
                    let curl_up = CurlUpListener::new(Entity::Enemy(i), global_info.ascention, rng);
                    self.event_listeners.push(Box::new(curl_up));
                }
                EnemyEnum::GreenLouse(_) => {
                    // Green Louse also gets a curl up listener with randomly generated block amount
                    let curl_up = CurlUpListener::new(Entity::Enemy(i), global_info.ascention, rng);
                    self.event_listeners.push(Box::new(curl_up));
                }
                EnemyEnum::JawWorm(_) => {
                    // Jaw Worm has no special listeners
                }
            }
        }
    }
    
    /// Emit a battle event to all listeners
    pub fn emit_event(&mut self, event: BattleEvent) {
        let mut effects_to_apply = Vec::new();
        
        for listener in &mut self.event_listeners {
            if listener.is_active() {
                let triggered_effects = listener.on_event(&event);
                for effect in triggered_effects {
                    let base_effect = BaseEffect::from_effect(effect, listener.get_owner(), listener.get_owner());
                    effects_to_apply.push(base_effect);
                }
            }
        }
        
        // Apply all triggered effects
        for effect in effects_to_apply {
            self.eval_effect_with_target(&effect);
        }
    }
    
    pub fn get_player(&self) -> &Player {
        &self.player
    }
    
    pub fn start_player_turn(&mut self) {
        self.player.start_turn();
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
    
    pub fn is_valid_target(&self, target: &Entity) -> bool {
        match target {
            Entity::Enemy(idx) => *idx < self.enemies.len(),
            Entity::Player => true,  // Player is always a valid target
            Entity::None => false,   // None is not a valid target
        }
    }
    
    pub fn eval_action(&mut self, action: Action, rng: &mut impl rand::Rng) -> Result<BattleResult, BattleError> {
        if self.is_battle_over() {
            return Err(BattleError::GameAlreadyOver);
        }

        match action {
            Action::PlayCard(idx, target) => {
                if idx >= self.cards.hand_size() {
                    return Err(BattleError::CardNotInHand);
                }
                
                if !self.is_valid_target(&target) {
                    return Err(BattleError::InvalidTarget);
                }
                
                let hand = self.cards.get_hand();
                let card = &hand[idx];
                if !self.player.spend_energy(card.get_cost()) {
                    return Err(BattleError::NotEnoughEnergy);
                }
                
                // Restore energy since we're checking but not actually spending yet
                self.player.battle_info.gain_energy(card.get_cost());
                
                self.play_card(idx, target);
            }
            Action::EndTurn => {
                let global_info = self.global_info;
                self.end_turn(rng, &global_info);
            }
        }
        
        // Check if battle is over after the action
        if !self.player.is_alive() {
            Ok(BattleResult::Lost)
        } else if self.enemies.iter().all(|e| !e.battle_info.is_alive()) {
            Ok(BattleResult::Won)
        } else {
            Ok(BattleResult::Continued)
        }
    }

    pub fn play_card(&mut self, idx: usize, target: Entity) {
        if idx >= self.cards.hand_size() { return; }
        
        let hand = self.cards.get_hand();
        let card = &hand[idx];
        if !self.player.spend_energy(card.get_cost()) { return; }
        
        let card_effects = card.get_effects().clone();
        if let Some(played_card) = self.cards.play_card_from_hand(idx) {
            for effect in card_effects {
                self.eval_effect_with_target(&BaseEffect::from_effect(effect, Entity::Player, target));
            }
        }
    }
    
    pub fn eval_effect_with_target(&mut self, effect: &BaseEffect) {
        match effect {
            BaseEffect::AttackToTarget { source, target, amount, num_attacks } => {
                for _ in 0..*num_attacks {
                    let damage = match source {
                        Entity::Player => self.player.battle_info.calculate_damage(*amount),
                        Entity::Enemy(idx) => {
                            if *idx < self.enemies.len() {
                                self.enemies[*idx].battle_info.calculate_damage(*amount)
                            } else {
                                *amount // Fallback to base damage if enemy not found
                            }
                        },
                        Entity::None => *amount, // Use base damage
                    };
                    self.apply_damage(*target, damage);
                }
            },
            BaseEffect::GainDefense { source, amount } => {
                // Defense effects apply to the source entity
                self.apply_block(*source, *amount);
            },
            BaseEffect::ApplyVulnerable { target, duration } => {
                match target {
                    Entity::Player => self.player.battle_info.apply_vulnerable(*duration),
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.apply_vulnerable(*duration);
                        }
                    },
                    Entity::None => {} // No target
                }
            },
            BaseEffect::ApplyWeak { target, duration } => {
                match target {
                    Entity::Player => self.player.battle_info.apply_weak(*duration),
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.apply_weak(*duration);
                        }
                    },
                    Entity::None => {} // No target
                }
            },
            BaseEffect::GainStrength { source, amount } => {
                match source {
                    Entity::Player => self.player.battle_info.gain_strength(*amount),
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.gain_strength(*amount);
                        }
                    },
                    Entity::None => {} // No source
                }
            },
        }
    }

    /// Apply damage to an entity (player or enemy)
    pub fn apply_damage(&mut self, target: Entity, damage: u32) -> u32 {
        let actual_damage = match target {
            Entity::Player => self.player.battle_info.take_damage(damage),
            Entity::Enemy(idx) => {
                if idx < self.enemies.len() {
                    self.enemies[idx].battle_info.take_damage(damage)
                } else {
                    0 // Invalid enemy index, no damage dealt
                }
            }
            Entity::None => 0, // No target, no damage dealt
        };
        
        // Emit damage taken event if actual damage was dealt
        if actual_damage > 0 {
            let damage_event = BattleEvent::DamageTaken {
                target,
                amount: actual_damage,
                source: Entity::None, // TODO: Track damage source
            };
            self.emit_event(damage_event);
        }
        
        actual_damage
    }

    /// Apply block to an entity (player or enemy) 
    pub fn apply_block(&mut self, target: Entity, amount: u32) {
        match target {
            Entity::Player => self.player.battle_info.gain_block(amount),
            Entity::Enemy(idx) => {
                if idx < self.enemies.len() {
                    self.enemies[idx].battle_info.gain_block(amount);
                }
            }
            Entity::None => {} // No target, no block gained
        }
    }

    /// Refresh both player and all enemies (reset blocks, decrement status effects)
    pub fn refresh_all(&mut self) {
        self.player.battle_info.refresh();
        for enemy in &mut self.enemies {
            enemy.battle_info.refresh();
        }
    }
    
    /// Ends the player turn and starts a new turn sequence
    pub fn end_turn(&mut self, rng: &mut impl rand::Rng, global_info: &GlobalInfo) {
        // 1. Discard all remaining cards in hand
        self.cards.discard_entire_hand();
        
        // 2. Execute enemy turn
        self.enemy_turn(rng, global_info);
        
        // 3. Refresh all characters after enemy turn (reset block, decrement status effects)
        self.refresh_all();
        
        // 4. Start new player turn
        self.start_player_turn();
        
        // 5. Draw new hand (typically 5 cards)
        for _ in 0..5 {
            self.cards.draw_card();
        }
    }

    pub fn enemy_turn(&mut self, rng: &mut impl rand::Rng, _global_info: &GlobalInfo) {
        let mut all_effects = Vec::new();
        
        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            let source = Entity::Enemy(i);
            
            // Use the new choose_effects method that handles everything in one step
            let effects = enemy.enemy.choose_effects(_global_info, rng);
            
            for effect in effects {
                let base_effect = BaseEffect::from_effect(effect, source, Entity::Player);
                all_effects.push(base_effect);
            }
        }
        
        // Apply all collected effects
        for effect in all_effects {
            self.eval_effect_with_target(&effect);
        }
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
let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse), hp)];
let battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        assert_eq!(battle.player.battle_info.get_hp(), 80);
        assert_eq!(battle.player.get_block(), 0);
        assert_eq!(battle.player.get_energy(), 3);
        assert!(!battle.enemies.is_empty());
        
        println!("{:?}", battle.cards.get_deck());
        println!("{:?}", battle.cards.get_hand());
    }

    #[test]
    fn test_eval_effect_with_target() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse), hp)];
let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 10,
            num_attacks: 1,
        };
        
        battle.eval_effect_with_target(&damage_effect);
        
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_enemy_hp - 10);
    }

    #[test]
    fn test_play_card_with_target() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse), hp)];
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
let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse), hp)];
let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Apply vulnerable to enemy
        let vulnerable_effect = BaseEffect::ApplyVulnerable { target: Entity::Enemy(0), duration: 2 };
        battle.eval_effect_with_target(&vulnerable_effect);
        
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
        battle.eval_effect_with_target(&damage_effect);
        
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
let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse), hp)];
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
        battle.eval_effect_with_target(&damage_effect);
        
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
let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse), hp)];
let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_hp = battle.player.battle_info.get_hp();
        
        // Create an effect that damages the player
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Enemy(0),
            target: Entity::Player,
            amount: 10,
            num_attacks: 1,
        };
        battle.eval_effect_with_target(&damage_effect);
        
        // Player should take 10 damage
        assert_eq!(battle.player.battle_info.get_hp(), initial_hp - 10);
    }

    #[test]
    fn test_attack_with_strength() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse), hp)];
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
        battle.eval_effect_with_target(&attack_effect);
        
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
let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse), hp)];
let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Record initial state
        let initial_player_hp = battle.player.battle_info.get_hp();
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        let initial_energy = battle.player.get_energy();
        
        // === PLAYER TURN ===
        
        // Player starts turn (should reset block and refresh energy)
        battle.player.start_turn();
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
        
        battle.enemy_turn(&mut rng, &global_info);
        
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
let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse), hp)];
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
        let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse), hp)];
        let mut normal_battle = Battle::new(deck.clone(), normal_global_info, 80, 80, enemies, &mut rng);
        normal_battle.apply_damage(Entity::Enemy(0), 6);
        let normal_block = normal_battle.enemies[0].battle_info.get_block();
        assert!(normal_block >= 3 && normal_block <= 7);
        
        // Test mid ascension (7-16): should give 4-8 block
        let mid_global_info = GlobalInfo { ascention: 10, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &mid_global_info);
        let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse), hp)];
        let mut mid_battle = Battle::new(deck.clone(), mid_global_info, 80, 80, enemies, &mut rng);
        mid_battle.apply_damage(Entity::Enemy(0), 6);
        let mid_block = mid_battle.enemies[0].battle_info.get_block();
        assert!(mid_block >= 4 && mid_block <= 8);
        
        // Test high ascension (17+): should give 9-12 block
        let high_global_info = GlobalInfo { ascention: 17, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &high_global_info);
        let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse), hp)];
        let mut high_battle = Battle::new(deck, high_global_info, 80, 80, enemies, &mut rng);
        high_battle.apply_damage(Entity::Enemy(0), 6);
        let high_block = high_battle.enemies[0].battle_info.get_block();
        assert!(high_block >= 9 && high_block <= 12);
    }

}