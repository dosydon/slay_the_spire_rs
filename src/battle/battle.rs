use crate::{enemies::enemy_enum::EnemyMove, game::{card::Card, deck::Deck, effect::{BaseEffect, BattleEffect}, global_info::GlobalInfo, player_run_state::PlayerRunState}, relics::Relic};
use super::{battle_events::{EventListener, BattleEvent}, player::Player, deck_hand_pile::DeckHandPile, enemy_in_battle::EnemyInBattle, battle_error::BattleError, target::Entity, battle_state::BattleState, event_listener_enum::EventListenerEnum};
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Battle {
    pub(super) player: Player,
    pub(super) enemies: Vec<EnemyInBattle>,
    pub(crate) cards: DeckHandPile,
    pub(super) event_listeners: Vec<EventListenerEnum>,
    pub(super) global_info: GlobalInfo,
    /// Stores the next move and effects for each enemy (index corresponds to enemies Vec)
    pub(super) enemy_actions: Vec<Option<(EnemyMove, Vec<BattleEffect>)>>,
    pub(super) relics: Vec<Relic>,
    /// Stores all powers played during the current battle
    pub(super) powers: Vec<crate::game::card::Card>,
    /// Queue of effects to be processed
    pub(super) effect_queue: Vec<BaseEffect>,
    pub(crate) battle_state: BattleState,
    /// Gold stolen during battle (e.g., by Looter)
    pub(super) gold_stolen: u32,
    /// Events that occurred during the last action (for GUI to read)
    pub battle_events: Vec<BattleEvent>,
    /// Potion inventory for the player
    pub(super) potions: crate::potion::PotionInventory,
    /// Cards that need to be discarded after effects are processed
    pub(super) to_be_discarded: Vec<Card>,
}

impl Battle {
    /// Create a new battle with PlayerRunState
    pub fn new(deck: Deck, global_info: GlobalInfo, player_state: PlayerRunState, enemies: Vec<EnemyInBattle>, rng: &mut impl rand::Rng) -> Self {
        let cards = DeckHandPile::new(deck);
        let enemy_count = enemies.len();

        // Convert relics to event listeners
        let event_listeners: Vec<_> = player_state.relics.iter()
            .filter_map(|relic| relic.clone().to_battle_event_listener())
            .collect();

        let mut battle = Battle {
            player: Player::new(player_state.current_hp, player_state.max_hp, 3),
            enemies,
            cards,
            event_listeners,
            global_info,
            enemy_actions: vec![None; enemy_count],
            relics: player_state.relics,
            powers: Vec::new(),
            effect_queue: Vec::new(),
            battle_state: BattleState::PlayerTurn,
            gold_stolen: 0,
            battle_events: Vec::new(),
            potions: player_state.potions,
            to_be_discarded: Vec::new(),
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
    pub fn new_with_shuffle(mut deck: Deck, global_info: GlobalInfo, player_state: PlayerRunState, enemies: Vec<EnemyInBattle>, rng: &mut impl rand::Rng) -> Self {
        // Shuffle the deck first
        deck.shuffle(rng);

        // Then call the main constructor
        Self::new(deck, global_info, player_state, enemies, rng)
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

    /// Extract the final player run state after battle
    /// This includes updated HP, gold (after stolen gold), and potions
    /// Relics are NOT updated as they remain static during battle
    pub fn get_final_player_run_state(&self, original_gold: u32, original_relics: Vec<Relic>) -> PlayerRunState {
        let final_hp = self.player.battle_info.get_hp();
        let final_gold = original_gold.saturating_sub(self.gold_stolen);

        PlayerRunState::new_with_relics_and_potions(
            final_hp,
            self.player.battle_info.get_max_hp(),
            final_gold,
            original_relics,
            self.potions.clone(),
        )
    }

    /// Get the current battle state
    pub fn get_battle_state(&self) -> BattleState {
        self.battle_state.clone()
    }

    /// Set the battle state
    pub fn set_battle_state(&mut self, state: BattleState) {
        self.battle_state = state;
    }




    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub(crate) fn get_player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    /// Get current HP
    pub fn get_current_hp(&self) -> u32 {
        self.player.get_current_hp()
    }

    /// Get max HP
    pub fn get_max_hp(&self) -> u32 {
        self.player.get_max_hp()
    }

    pub fn get_enemies(&self) -> &Vec<EnemyInBattle> {
        &self.enemies
    }

    /// Compute a hash of the battle state for transposition table
    /// Hashes all relevant state for MCTS duplicate detection
    pub fn hash_state(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();

        // Hash player state
        self.player.get_current_hp().hash(&mut hasher);
        self.player.get_max_hp().hash(&mut hasher);
        self.player.get_block().hash(&mut hasher);
        self.player.get_energy().hash(&mut hasher);

        // Hash enemy state
        for enemy in &self.enemies {
            enemy.get_name().hash(&mut hasher);
            enemy.get_current_hp().hash(&mut hasher);
            enemy.battle_info.get_block().hash(&mut hasher);
        }

        // Hash card state
        self.cards.get_hand().len().hash(&mut hasher);
        self.cards.deck_size().hash(&mut hasher);
        self.cards.discard_pile_size().hash(&mut hasher);

        // Hash event listeners (relics) using the object-safe hash_to method
        for listener in &self.event_listeners {
            listener.hash_to(&mut hasher);
        }

        hasher.finish()
    }

    /// Get mutable enemies (for testing purposes)
    pub fn get_enemies_mut(&mut self) -> &mut Vec<EnemyInBattle> {
        &mut self.enemies
    }

    /// Get total gold stolen during battle
    pub fn get_gold_stolen(&self) -> u32 {
        self.gold_stolen
    }

    /// Get global info
    pub fn get_global_info(&self) -> &GlobalInfo {
        &self.global_info
    }

    pub fn get_hand(&self) -> &Vec<Card> {
        self.cards.get_hand()
    }

    /// Count the number of Strike cards in the entire deck (draw pile + hand + discard pile)
    /// Strike cards include: Strike, PerfectedStrike, PommelStrike, TwinStrike, WildStrike, SwiftStrike
    pub fn count_strike_cards_in_deck(&self) -> u32 {
        use crate::game::card_enum::CardEnum;

        let strike_cards = [
            CardEnum::Strike,
            CardEnum::PerfectedStrike,
            CardEnum::PommelStrike,
            CardEnum::TwinStrike,
            CardEnum::WildStrike,
            CardEnum::SwiftStrike,
        ];

        let mut count = 0;

        // Count in hand
        for card in self.cards.get_hand() {
            if strike_cards.contains(&card.get_card_enum()) {
                count += 1;
            }
        }

        // Count in draw pile (deck)
        for card in self.cards.get_deck().get_cards() {
            if strike_cards.contains(&card.get_card_enum()) {
                count += 1;
            }
        }

        // Count in discard pile
        for card in self.cards.get_discard_pile() {
            if strike_cards.contains(&card.get_card_enum()) {
                count += 1;
            }
        }

        count
    }

    /// Add a card to hand (for testing purposes)
    pub fn add_card_to_hand_for_testing(&mut self, card: Card) {
        self.cards.add_card_to_hand(card);
    }

    /// Get the discard pile (for testing purposes)
    pub fn get_discard_pile(&self) -> &Vec<Card> {
        self.cards.get_discard_pile()
    }

    /// Get the exhaust pile (for testing purposes)
    pub fn get_exhaust_pile(&self) -> &Vec<Card> {
        self.cards.get_exhausted()
    }

    /// Get the deck (for testing purposes)
    pub fn get_deck(&self) -> &crate::game::deck::Deck {
        self.cards.get_deck()
    }

    /// Get the number of cards in the draw pile
    pub fn get_draw_pile_count(&self) -> usize {
        self.cards.deck_size()
    }

    /// Get the number of cards in the discard pile
    pub fn get_discard_pile_count(&self) -> usize {
        self.cards.discard_pile_size()
    }

    /// Get the number of cards in the exhaust pile
    pub fn get_exhaust_pile_count(&self) -> usize {
        self.cards.exhausted_size()
    }

    /// Get all powers played during this battle
    pub fn get_powers(&self) -> &Vec<Card> {
        &self.powers
    }

    /// Check if Corruption power is active (makes skills cost 0)
    pub fn has_corruption_active(&self) -> bool {
        self.powers.iter().any(|card| card.get_name() == "Corruption" || card.get_name() == "Corruption+")
    }

    /// Get the modified cost of a card considering active powers like Corruption
    pub fn get_modified_cost(&self, card: &crate::game::card::Card) -> u32 {
        if card.get_card_type() == crate::game::card_type::CardType::Skill && self.has_corruption_active() {
            0  // Skills cost 0 with Corruption
        } else {
            card.get_cost()
        }
    }

    /// Get reference to the potion inventory
    pub fn get_potions(&self) -> &crate::potion::PotionInventory {
        &self.potions
    }

    /// Get mutable reference to the potion inventory
    pub fn get_potions_mut(&mut self) -> &mut crate::potion::PotionInventory {
        &mut self.potions
    }

    /// Get the player's relics
    pub fn get_relics(&self) -> &Vec<Relic> {
        &self.relics
    }

    /// Flush all cards in to_be_discarded to the discard pile
    pub(crate) fn flush_to_be_discarded(&mut self) {
        for card in self.to_be_discarded.drain(..) {
            self.cards.add_card_to_discard(card);
        }
    }

    /// Use a potion at the specified slot index
    /// Returns an error if the slot is empty or the potion cannot be used
    pub fn use_potion(&mut self, slot_index: usize, target: Option<Entity>) -> Result<(), BattleError> {
        // Get the potion from inventory
        let potion = self.potions.use_potion(slot_index)
            .ok_or(BattleError::PotionNotInInventory(slot_index))?;

        // Get the effects
        let (default_target, effects) = potion.get_effects();

        // Determine the actual target
        let actual_target = target.or(default_target)
            .ok_or(BattleError::InvalidAction)?;

        // Apply all effects (potions are used by the player)
        for effect in effects {
            self.queue_effect(BaseEffect::from_effect(effect, Entity::Player, actual_target));
        }

        self.process_effect_queue();

        Ok(())
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

    /// Evaluate if a condition is met in the current battle context
    pub fn eval_condition(&self, condition: crate::game::effect::Condition) -> bool {
        use crate::game::effect::Condition;
        match condition {
            Condition::True => true,
            Condition::False => false,
            Condition::TargetIsVulnerable => {
                // This would need target context, for now return false
                false
            }
            Condition::HandAllAttacks => {
                let hand = self.cards.get_hand();
                hand.iter().all(|c| c.get_card_type() == crate::game::card_type::CardType::Attack)
            }
            Condition::HandNoAttacks => {
                let hand = self.cards.get_hand();
                hand.iter().all(|c| c.get_card_type() != crate::game::card_type::CardType::Attack)
            }
            Condition::EnemyIsAttacking => {
                // Check if any enemy is currently attacking
                // For now, assume all moves are attacking (simplified)
                // In a full implementation, we'd check specific move types
                !self.enemies.is_empty()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::battle_action::BattleAction;
    use crate::battle::battle_state::BattleState;
    use crate::cards::ironclad::starter_deck::starter_deck;
    use crate::game::enemy::EnemyTrait;
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};

    #[test]
    fn test_battle_initialization() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let player_state = PlayerRunState::new(80, 80, 0);
        let battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);
        assert_eq!(battle.player.battle_info.get_hp(), 80);
        assert_eq!(battle.player.get_block(), 0);
        assert_eq!(battle.player.get_energy(), 3);
        assert!(!battle.enemies.is_empty());

        println!("{:?}", battle.cards.get_deck());
        println!("{:?}", battle.cards.get_hand());
    }

    #[test]
    fn test_potion_usage() {
        use crate::potion::Potion;

        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let player_state = PlayerRunState::new(80, 80, 0);
        let mut battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Initially no potions
        assert_eq!(battle.get_potions().potion_count(), 0);

        // Add a strength potion
        assert!(battle.get_potions_mut().add_potion(Potion::StrengthPotion));
        assert_eq!(battle.get_potions().potion_count(), 1);

        // Player should have 0 strength initially
        assert_eq!(battle.player.battle_info.get_strength(), 0);

        // Use the potion (StrengthPotion targets player automatically)
        let result = battle.use_potion(0, None);
        assert!(result.is_ok());

        // Player should now have 2 strength
        assert_eq!(battle.player.battle_info.get_strength(), 2);

        // Potion should be consumed
        assert_eq!(battle.get_potions().potion_count(), 0);
    }

    #[test]
    fn test_potion_usage_invalid_slot() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let player_state = PlayerRunState::new(80, 80, 0);
        let mut battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Try to use potion from empty slot
        let result = battle.use_potion(0, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), BattleError::PotionNotInInventory(0));

        // Try to use potion from invalid slot index
        let result = battle.use_potion(10, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), BattleError::PotionNotInInventory(10));
    }

    #[test]
    fn test_use_potion_action() {
        use crate::potion::Potion;
        use crate::battle::battle_action::BattleAction;

        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let player_state = PlayerRunState::new(80, 80, 0);
        let mut battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Add a strength potion
        assert!(battle.get_potions_mut().add_potion(Potion::StrengthPotion));

        // Verify UsePotion action is in available actions
        let available = battle.list_available_actions();
        assert!(available.iter().any(|a| matches!(a, BattleAction::UsePotion(0, None))));

        // Player should have 0 strength initially
        assert_eq!(battle.player.battle_info.get_strength(), 0);

        // Execute the UsePotion action
        let result = battle.eval_action(BattleAction::UsePotion(0, None), &mut rng);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::battle::BattleResult::Continued(_)));

        // Player should now have 2 strength
        assert_eq!(battle.player.battle_info.get_strength(), 2);

        // Potion should be consumed
        assert_eq!(battle.get_potions().potion_count(), 0);

        // UsePotion action should no longer be in available actions
        let available = battle.list_available_actions();
        assert!(!available.iter().any(|a| matches!(a, BattleAction::UsePotion(_, _))));
    }
}

impl crate::agents::ForwardSimulation for Battle {
    type Action = super::battle_action::BattleAction;

    fn list_available_actions(&self) -> Vec<Self::Action> {
        self.list_available_actions()
    }

    fn eval_action(&mut self, action: Self::Action, rng: &mut impl rand::Rng) -> Result<(), crate::game::game_error::GameError> {
        self.eval_action(action, rng)
            .map(|_| ())
            .map_err(crate::game::game_error::GameError::Battle)
    }

    fn is_terminal(&self) -> bool {
        self.is_battle_over()
    }

    fn evaluate(&self) -> f32 {
        // Terminal: player dead
        if !self.player.battle_info.is_alive() {
            return 0.0;
        }

        // Terminal: victory (all enemies dead)
        if self.enemies.iter().all(|e| !e.battle_info.is_alive()) {
            let player_hp = self.player.battle_info.get_hp() as f32;
            let player_max_hp = self.player.battle_info.get_max_hp() as f32;
            let hp_ratio = if player_max_hp > 0.0 {
                player_hp / player_max_hp
            } else {
                0.0
            };
            // Victory reward: 1.2 + (hp_ratio * 0.5) → range [1.2, 1.7]
            return 1.2 + (hp_ratio * 0.5);
        }

        // Non-terminal: heuristic evaluation
        // Player HP ratio
        let player_hp = self.player.battle_info.get_hp() as f32;
        let player_max_hp = self.player.battle_info.get_max_hp() as f32;
        let player_ratio = if player_max_hp > 0.0 {
            player_hp / player_max_hp
        } else {
            0.0
        };

        // Enemy HP ratios
        let total_enemy_hp: f32 = self.enemies.iter()
            .map(|e| e.battle_info.get_hp() as f32)
            .sum();
        let total_enemy_max_hp: f32 = self.enemies.iter()
            .map(|e| e.battle_info.get_max_hp() as f32)
            .sum();

        let enemy_ratio = if total_enemy_max_hp > 0.0 {
            total_enemy_hp / total_enemy_max_hp
        } else {
            0.0
        };

        // Combined score: player_ratio - enemy_ratio → range [-1, 1]
        player_ratio - enemy_ratio
    }
}
