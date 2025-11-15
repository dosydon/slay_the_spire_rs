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
mod listener_manager;

use crate::{enemies::{red_louse::{RedLouse, RedLouseMove}, green_louse::GreenLouseMove, jaw_worm::JawWormMove, enemy_enum::{EnemyEnum, EnemyMove}}, game::{card::Card, deck::Deck, effect::{BaseEffect, Effect}, enemy::EnemyTrait, global_info::GlobalInfo}, relics::Relic};
use self::{target::Entity, events::{EventListener, BattleEvent}, player::Player, deck_hand_pile::DeckHandPile, enemy_in_battle::EnemyInBattle};

#[derive(Debug, Clone, PartialEq)]
pub enum BattleError {
    InvalidAction,
    NotEnoughEnergy,
    CardNotInHand,
    InvalidTarget,
    GameAlreadyOver,
    CardNotPlayable,
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
    pub(crate) cards: DeckHandPile,
    event_listeners: Vec<Box<dyn EventListener>>,
    global_info: GlobalInfo,
    /// Stores the next move and effects for each enemy (index corresponds to enemies Vec)
    enemy_actions: Vec<Option<(EnemyMove, Vec<Effect>)>>,
    relics: Vec<Relic>,
    /// Stores all powers played during the current battle
    powers: Vec<crate::game::card::Card>,
}

impl Battle {
    pub fn new(deck: Deck, global_info: GlobalInfo, initial_hp: u32, max_hp: u32, enemies: Vec<EnemyInBattle>, rng: &mut impl rand::Rng) -> Self {
        Self::new_with_relics(deck, global_info, initial_hp, max_hp, enemies, Vec::new(), rng)
    }

    /// Create a new battle with optional relics
    pub fn new_with_relics(deck: Deck, global_info: GlobalInfo, initial_hp: u32, max_hp: u32, enemies: Vec<EnemyInBattle>, relics: Vec<crate::relics::Relic>, rng: &mut impl rand::Rng) -> Self {
        let cards = DeckHandPile::new(deck);
        let enemy_count = enemies.len();

        // Convert relics to event listeners
        let event_listeners: Vec<_> = relics.into_iter()
            .filter_map(|relic| relic.to_battle_event_listener())
            .collect();

        let mut battle = Battle {
            player: Player::new(initial_hp, max_hp, 3),
            enemies,
            cards,
            event_listeners,
            global_info,
            enemy_actions: vec![None; enemy_count],
            relics: Vec::new(),
            powers: Vec::new(),
        };

        // Initialize event listeners for enemies
        battle.initialize_enemy_listeners(&global_info, rng);

        // Emit combat start event for relics
        battle.emit_event(BattleEvent::CombatStart { player: Entity::Player });

        // Initialize the first player turn (draw cards, sample enemy actions, but don't reset block)
        battle.initialize_first_turn(rng);

        battle
    }

    /// Create a new battle with deck shuffling
    pub fn new_with_shuffle(deck: Deck, global_info: GlobalInfo, initial_hp: u32, max_hp: u32, enemies: Vec<EnemyInBattle>, rng: &mut impl rand::Rng) -> Self {
        Self::new_with_shuffle_and_relics(deck, global_info, initial_hp, max_hp, enemies, Vec::new(), rng)
    }

    /// Create a new battle with deck shuffling and relics
    pub fn new_with_shuffle_and_relics(mut deck: Deck, global_info: GlobalInfo, initial_hp: u32, max_hp: u32, enemies: Vec<EnemyInBattle>, relics: Vec<crate::relics::Relic>, rng: &mut impl rand::Rng) -> Self {
        // Shuffle the deck first
        deck.shuffle(rng);

        // Then call the constructor with relics
        Self::new_with_relics(deck, global_info, initial_hp, max_hp, enemies, relics, rng)
    }

    pub fn set_relics(self, relics: Vec<Relic>) -> Self {
        Battle {
            relics,
            ..self
        }
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

    /// Get all powers played during this battle
    pub fn get_powers(&self) -> &Vec<Card> {
        &self.powers
    }
    
    pub fn is_battle_over(&self) -> bool {
        !self.player.is_alive() || self.enemies.iter().all(|e| !e.battle_info.is_alive())
    }
    
    /// Calculate incoming damage with all modifiers (strength, weak, vulnerable)
    pub fn calculate_incoming_damage(&self, attacker: Entity, target: Entity, base_damage: u32) -> u32 {
        self.calculate_incoming_damage_with_multiplier(attacker, target, base_damage, 1)
    }

    /// Calculate incoming damage with all modifiers and custom strength multiplier
    pub fn calculate_incoming_damage_with_multiplier(&self, attacker: Entity, target: Entity, base_damage: u32, strength_multiplier: u32) -> u32 {
        // Step 1: Calculate damage with attacker's modifiers (strength, weak)
        let modified_damage = match attacker {
            Entity::Player => self.player.battle_info.calculate_damage_with_multiplier(base_damage, strength_multiplier),
            Entity::Enemy(idx) => {
                if idx < self.enemies.len() {
                    self.enemies[idx].battle_info.calculate_damage_with_multiplier(base_damage, strength_multiplier)
                } else {
                    base_damage
                }
            }
            Entity::None => base_damage,
        };
        
        // Step 2: Apply target's vulnerable multiplier
        match target {
            Entity::Player => self.player.battle_info.calculate_incoming_damage(modified_damage),
            Entity::Enemy(idx) => {
                if idx < self.enemies.len() {
                    self.enemies[idx].battle_info.calculate_incoming_damage(modified_damage)
                } else {
                    modified_damage
                }
            }
            Entity::None => modified_damage,
        }
    }
    
    

    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::ironclad::starter_deck::starter_deck;
    use crate::game::enemy::EnemyTrait;

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
}