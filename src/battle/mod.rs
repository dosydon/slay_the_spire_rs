pub mod action;
pub mod character_battle_info;
pub mod target;

use crate::{enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum}, game::{card::Card, deck::Deck, effect::BaseEffect, enemy::{EnemyInGame, EnemyTrait}, global_info::GlobalInfo}};
use self::{action::Action, character_battle_info::CharacterBattleInfo, target::Entity};

#[derive(Debug)]
pub struct Player {
    pub battle_info: CharacterBattleInfo,
}

impl Player {
    pub fn new(hp: u32, energy: u32) -> Self {
        Player {
            battle_info: CharacterBattleInfo::new(hp, energy),
        }
    }

    pub fn spend_energy(&mut self, amount: u32) -> bool {
        self.battle_info.spend_energy(amount)
    }

    pub fn get_energy(&self) -> u32 {
        self.battle_info.get_energy()
    }

    pub fn get_block(&self) -> u32 {
        self.battle_info.get_block()
    }

    pub fn apply_vulnerable(&mut self, turns: u32) {
        self.battle_info.apply_vulnerable(turns);
    }

    pub fn is_vulnerable(&self) -> bool {
        self.battle_info.is_vulnerable()
    }
    pub fn start_turn(&mut self) {
        self.battle_info.refresh();
        // Player gets 3 energy at start of turn
        self.battle_info.energy = 3;
    }

    pub fn is_alive(&self) -> bool {
        self.battle_info.is_alive()
    }
}

pub enum Phase {
    MainPhase,
    SelectEnemyPhase,
}

pub enum GameError {
    InvalidAction,
    NotEnoughEnergy,
    CardNotInHand,
}

pub struct Battle {
    player: Player,
    enemies: Vec<EnemyInGame>,
    hand: Vec<Card>,
    deck: Deck,
    phase: Phase,
}

impl Battle {
    pub fn new(deck: Deck, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> Self {
        let (deck, hand) = deck.initialize_game(rng);
        Battle {
            player: Player::new(80, 3),
            enemies: vec![{
                let red_louse = RedLouse::instantiate(rng, global_info);
                let hp = rng.random_range(RedLouse::hp_lb()..=RedLouse::hp_ub());
                EnemyInGame::new(EnemyEnum::RedLouse(red_louse), hp)
            }],
            hand: hand,
            deck,
            phase: Phase::MainPhase,
        }
    }
    
    pub fn get_player(&self) -> &Player {
        &self.player
    }
    
    pub fn get_enemies(&self) -> &Vec<EnemyInGame> {
        &self.enemies
    }
    
    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }
    
    pub fn get_phase(&self) -> &Phase {
        &self.phase
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
    
    pub fn eval_action(&mut self, action: Action) {
        match action {
            Action::PlayCard(idx, target) => {
                if idx < self.hand.len() && self.is_valid_target(&target) {
                    self.play_card(idx, target);
                }
            }
            Action::EndTurn => {
                // End turn functionality will be handled externally 
                // or through a separate game loop manager
            }
//            Action::SelectEnemy(idx) => {
//                if idx < self.enemies.len() {
//                    self.select_enemy(idx);
//                }
//            }
        }
    }

    pub fn play_card(&mut self, idx: usize, target: Entity) {
        if idx >= self.hand.len() { return; }
        
        let card = &self.hand[idx];
        if !self.player.spend_energy(card.get_cost()) { return; }
        
        let card_effects = card.get_effects().clone();
        self.hand.remove(idx);
        
        for effect in card_effects {
            self.eval_effect_with_target(&BaseEffect::from_effect(effect, Entity::Player, target));
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
        match target {
            Entity::Player => self.player.battle_info.take_damage(damage),
            Entity::Enemy(idx) => {
                if idx < self.enemies.len() {
                    self.enemies[idx].battle_info.take_damage(damage)
                } else {
                    0 // Invalid enemy index, no damage dealt
                }
            }
            Entity::None => 0, // No target, no damage dealt
        }
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

    pub fn enemy_turn(&mut self, rng: &mut impl rand::Rng, _global_info: &GlobalInfo) {
        let mut all_effects = Vec::new();
        
        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            let source = Entity::Enemy(i);
            match &enemy.enemy {
                EnemyEnum::RedLouse(red_louse) => {
                    use crate::game::enemy::EnemyTrait;
                    let move_distribution = red_louse.choose_next_move(_global_info);
                    let mv = move_distribution.sample_owned(rng);
                    let effects = red_louse.get_move_effects(mv);
                    for effect in effects {
                        let base_effect = BaseEffect::from_effect(effect, source, Entity::Player);
                        all_effects.push(base_effect);
                    }
                }
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
        let battle = Battle::new(deck, &global_info, &mut rng);
        assert_eq!(battle.player.battle_info.get_hp(), 80);
        assert_eq!(battle.player.get_block(), 0);
        assert_eq!(battle.player.get_energy(), 3);
        assert!(!battle.enemies.is_empty());
        
        println!("{:?}", battle.deck);
        println!("{:?}", battle.hand);
    }

    #[test]
    fn test_eval_effect_with_target() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
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
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
        let initial_energy = battle.player.get_energy();
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        
        // Find a Strike card in the hand
        let strike_idx = battle.hand.iter().position(|card| card.get_name() == "Strike");
        
        if let Some(idx) = strike_idx {
            // Play the Strike card targeting enemy 0
            let action = Action::PlayCard(idx, Entity::Enemy(0));
            battle.eval_action(action);
            
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
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
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
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
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
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_hp - 3);
        assert_eq!(battle.enemies[0].battle_info.get_block(), 0);
    }

    #[test]
    fn test_damage_to_player() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
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
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
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
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
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
        let defend_idx = battle.hand.iter().position(|card| card.get_name() == "Defend");
        if let Some(idx) = defend_idx {
            let hand_size_before = battle.hand.len();
            battle.play_card(idx, Entity::Player);
            
            // Check card was played
            assert_eq!(battle.hand.len(), hand_size_before - 1);
            // Check energy was spent (Defend costs 1)
            assert_eq!(battle.player.get_energy(), 2);
            // Check block was gained (Defend gives 5 block)
            assert_eq!(battle.player.get_block(), 5);
        }
        
        // Player plays a Strike card targeting enemy
        let strike_idx = battle.hand.iter().position(|card| card.get_name() == "Strike");
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

}