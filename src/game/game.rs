use crate::{events::SLSEvent, game::{card_reward::CardRewardPool, deck::Deck, game_error::GameError, game_event::{GameEvent, GameEventListener}, global_info::GlobalInfo, game_state::GameState, reward_state::RewardState}};
use crate::map::{Map, NodeType, MapNode};
use crate::battle::Battle;
use crate::events::map_events::{MapEvent, EventChoice};
use log::{info, debug};

#[cfg(test)]
use crate::game::{action::{GameAction, RestSiteAction}, game_result::{GameResult, GameOutcome}};

pub struct Game {
    pub global_info: GlobalInfo,
    pub deck: Deck,
    pub battle: Option<Battle>,
    pub map: Map,
    pub current_node_position: (u32, u32),
    pub player_hp: u32,
    pub player_max_hp: u32,
    pub gold: u32,
    pub potions: crate::potion::PotionInventory,
    pub potion_pool: crate::potion::PotionPool,
    card_reward_pool: CardRewardPool,
    pub(crate) relics: Vec<crate::relics::Relic>,
    game_event_listeners: Vec<Box<dyn GameEventListener>>,
    pub(crate) event_history: Vec<SLSEvent>,
    state_stack: Vec<GameState>,
}

impl Game {
    /// Create a new game with starting deck, global info, and map
    /// Uses the map's starting position
    pub fn new(starting_deck: Deck, global_info: GlobalInfo, map: Map, starting_hp: u32, max_hp: u32) -> Self {
        let current_node_position = map.get_starting_position()
            .expect("Map must have a starting position set");

        Game {
            global_info,
            deck: starting_deck,
            battle: None,
            map,
            current_node_position,
            player_hp: starting_hp,
            player_max_hp: max_hp,
            gold: 99, // Starting gold (Ironclad starts with 99 gold)
            potions: crate::potion::PotionInventory::default(),
            potion_pool: crate::potion::PotionPool::default(),
            card_reward_pool: CardRewardPool::new(),
            relics: Vec::new(),
            game_event_listeners: Vec::new(),
            event_history: Vec::new(),
            state_stack: vec![GameState::OnMap],
        }
    }

    
    /// Add a game event listener to the game
    pub fn add_game_event_listener(&mut self, listener: Box<dyn GameEventListener>) {
        self.game_event_listeners.push(listener);
    }

    /// Add a relic to the game and register its event listener if applicable
    pub fn add_relic(&mut self, relic: crate::relics::Relic) {
        self.relics.push(relic.clone());

        // Register game event listeners if the relic supports them
        if let Some(listener) = relic.to_game_event_listener() {
            self.add_game_event_listener(listener);
        }
    }

    /// Get the length of the event history
    pub fn get_event_history_len(&self) -> usize {
        self.event_history.len()
    }

    /// Get a reference to the event history
    pub fn get_event_history(&self) -> &Vec<SLSEvent> {
        &self.event_history
    }

    /// Emit a game event to all active listeners and apply their effects
    pub fn emit_game_event(&mut self, event: GameEvent) {
        let mut new_effects = Vec::new();

        // Process all active listeners
        for listener in &mut self.game_event_listeners {
            if listener.is_active() {
                let effects = listener.on_game_event(&event);
                for effect in effects {
                    new_effects.push(effect);
                }
            }
        }

        // Remove inactive listeners
        self.game_event_listeners.retain(|listener| listener.is_active());

        // Apply healing effects directly to player HP
        for effect in new_effects {
            match effect {
                crate::game::effect::BattleEffect::Heal(amount) => {
                    self.player_hp = (self.player_hp + amount).min(self.player_max_hp);
                }
                // Handle other effects as needed
                _ => {}
            }
        }
    }

    /// Get the current game state (top of the stack)
    pub(crate) fn current_state(&self) -> &GameState {
        self.state_stack.last().unwrap_or(&GameState::OnMap)
    }

    /// Get the current game state (legacy method, use get_game_state instead)
    pub fn get_state(&self) -> &GameState {
        self.current_state()
    }

    /// Get the current game state
    pub fn get_game_state(&self) -> &GameState {
        self.current_state()
    }

    /// Set the game state (replace the top of the stack)
    pub fn set_game_state(&mut self, new_state: GameState) {
        if self.state_stack.is_empty() {
            self.state_stack.push(new_state);
        } else {
            self.state_stack.pop();
            self.state_stack.push(new_state);
        }
    }

    /// Push a new state onto the stack
    fn push_state(&mut self, new_state: GameState) {
        self.state_stack.push(new_state);
    }

    /// Pop the current state from the stack and return it
    pub(crate) fn pop_state(&mut self) -> Option<GameState> {
        if self.state_stack.len() > 1 {
            self.state_stack.pop()
        } else {
            // Never pop the last state, replace with OnMap instead
            self.state_stack.pop();
            self.state_stack.push(GameState::OnMap);
            None
        }
    }

    /// Check if the game is over
    pub fn is_game_over(&self) -> bool {
        // For the simplified version, game is never truly over
        false
    }
    
    /// Get the current battle if one is active
    pub fn get_battle(&self) -> Option<&Battle> {
        self.battle.as_ref()
    }
    
    /// Get the current map node
    pub fn get_current_node(&self) -> Option<&MapNode> {
        self.map.get_node(self.current_node_position)
    }
    
    /// Get the map
    pub fn get_map(&self) -> &Map {
        &self.map
    }
    
    /// Get player's current HP
    pub fn get_player_hp(&self) -> u32 {
        self.player_hp
    }
    
    /// Get player's maximum HP
    pub fn get_player_max_hp(&self) -> u32 {
        self.player_max_hp
    }
    
    /// Heal the player by the specified amount (outside of battle)
    pub fn heal_player(&mut self, amount: u32) {
        self.player_hp = (self.player_hp + amount).min(self.player_max_hp);
    }
    
    /// Set player's current HP (for battle syncing)
    pub fn set_player_hp(&mut self, hp: u32) {
        self.player_hp = hp.min(self.player_max_hp);
    }
    
    /// Increase player's max HP (from events, relics, etc.)
    pub fn increase_max_hp(&mut self, amount: u32) {
        self.player_max_hp += amount;
        // Also heal if at full HP
        if self.player_hp == self.player_max_hp - amount {
            self.player_hp = self.player_max_hp;
        }
    }

    pub fn get_relics(&self) -> &Vec<crate::relics::Relic> {
        &self.relics
    }

    /// Check if player is alive
    pub fn is_player_alive(&self) -> bool {
        self.player_hp > 0
    }

    /// Create a reward state based on the current node type
    pub(crate) fn create_reward_state_for_current_node(&mut self, rng: &mut impl rand::Rng) -> RewardState {
        // Get the current node from the map
        let node = self.map.get_node(self.current_node_position);

        // Roll for potion drop using the potion pool
        let potion_drop = match node.map(|n| &n.node_type) {
            Some(NodeType::Elite) => {
                // Elite combat: 40% base drop chance + increases
                Some(self.potion_pool.roll_potion_drop(rng))
            }
            Some(NodeType::Boss) => {
                // Boss combat: no potion drops
                None
            }
            Some(NodeType::Combat) | _ => {
                // Normal combat: 40% base drop chance + increases
                Some(self.potion_pool.roll_potion_drop(rng))
            }
        };

        // Determine gold reward and card selection based on node type
        let (gold_reward, card_selection_available) = match node.map(|n| &n.node_type) {
            Some(NodeType::Elite) => (rng.random_range(25..=35), true),
            Some(NodeType::Boss) => (rng.random_range(95..=105), true),
            Some(NodeType::Combat) | _ => (rng.random_range(10..=20), true),
        };

        RewardState {
            gold_reward,
            card_selection_available,
            gold_claimed: false,
            potion_reward: potion_drop.flatten(), // Convert Option<Option<Potion>> to Option<Potion>
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        }
    }

    /// Start card reward selection - generates 3 random card options
    /// Uses the persistent card_reward_pool to maintain rare offset across the run
    pub fn start_card_reward_selection(&mut self, rng: &mut impl rand::Rng, reward_state: RewardState) {
        let reward_options = self.card_reward_pool.generate_reward_options(rng);
        info!("Generated {} card reward options", reward_options.len());
        for (i, card) in reward_options.iter().enumerate() {
            debug!("  Option {}: {} (Cost: {})", i + 1, card.get_name(), card.get_cost());
        }
        self.push_state(GameState::CardRewardSelection(reward_options));
    }

    /// Get the current card reward options (only valid in CardRewardSelection state)
    pub fn get_card_reward_options(&self) -> &[crate::game::card::Card] {
        match self.current_state() {
            GameState::CardRewardSelection(options) => options,
            _ => &[],
        }
    }

    /// Get event choices with current game context
    pub fn get_event_choices(&self, event: &MapEvent) -> Vec<EventChoice> {
        use crate::events::map_events::EventContext;

        let ctx = EventContext {
            floor: self.global_info.current_floor,
            player_hp: self.player_hp,
            player_max_hp: self.player_max_hp,
            gold: self.gold,
            ascension: self.global_info.ascention,
        };

        event.get_choices_with_context(&ctx)
    }

    /// Start an SLS Event (using game context for event choices)
    pub fn start_event(&mut self, event: MapEvent) {
        let choices = self.get_event_choices(&event);
        self.set_game_state(GameState::InEvent(event, choices));
        info!("Started event: {}", event.get_description());
    }

    /// Get the current event (only valid in InEvent state)
    pub fn get_current_event(&self) -> Option<&MapEvent> {
        match self.current_state() {
            GameState::InEvent(event, _) => Some(event),
            _ => None,
        }
    }

    /// Get the current event choices (only valid in InEvent state)
    pub fn get_current_event_choices(&self) -> &[EventChoice] {
        match self.current_state() {
            GameState::InEvent(_, choices) => choices,
            _ => &[],
        }
    }

    /// Start shop visit with 5 random cards for sale
    pub fn start_shop(&mut self, rng: &mut impl rand::Rng) {
        let shop_state = crate::game::shop::ShopState::new(5, rng);
        info!("Started shop with {} cards for sale", shop_state.card_count());
        for (i, card) in shop_state.cards_for_sale.iter().enumerate() {
            if let Some(price) = shop_state.get_card_price(i) {
                debug!("  Card {}: {} - Cost: {}, Price: {} gold", i + 1, card.get_name(), card.get_cost(), price);
            }
        }
        self.set_game_state(GameState::Shop(shop_state));
    }

    /// Get the current shop state (only valid in Shop state)
    pub fn get_shop_state(&self) -> Option<&crate::game::shop::ShopState> {
        match self.current_state() {
            GameState::Shop(shop_state) => Some(shop_state),
            _ => None,
        }
    }

    /// Evaluate a single effect and apply it to the player/game
    pub(crate) fn eval_effect(&mut self, effect: crate::game::effect::Effect, rng: &mut impl rand::Rng) {
        use crate::game::effect::{Effect, BattleEffect, GameEffect};

        match effect {
            Effect::Battle(battle_effect) => {
                match battle_effect {
                    BattleEffect::Heal(amount) => {
                        // Handle special case: amount 0 means heal 1/3 of max HP
                        let heal_amount = if amount == 0 {
                            self.player_max_hp / 3
                        } else {
                            amount
                        };
                        self.player_hp = (self.player_hp + heal_amount).min(self.player_max_hp);
                        info!("Healed {} HP", heal_amount);
                    },
                    BattleEffect::HealAndIncreaseMaxHp(amount) => {
                        self.player_hp = (self.player_hp + amount).min(self.player_max_hp + amount);
                        self.player_max_hp += amount;
                        info!("Gained {} Max HP and healed to full", amount);
                    },
                    BattleEffect::LoseHp(amount) => {
                        self.player_hp = self.player_hp.saturating_sub(amount);
                        info!("Lost {} HP", amount);
                    },
                    BattleEffect::AddCardToDrawPile(card_enum) => {
                        // Add card to draw pile (will be added to deck when battle starts)
                        // For now, just add directly to deck
                        use crate::cards::status::slimed::slimed;
                        use crate::cards::ironclad::strike::strike;
                        use crate::cards::ironclad::defend::defend;

                        let card = match card_enum {
                            crate::game::card_enum::CardEnum::Slimed => slimed(),
                            crate::game::card_enum::CardEnum::Strike => strike(),
                            crate::game::card_enum::CardEnum::Defend => defend(),
                            _ => {
                                info!("Cannot add card {:?} to deck (not yet implemented)", card_enum);
                                return;
                            }
                        };
                        self.deck.add_card(card);
                        info!("Added {} to deck", card_enum.name());
                    },
                    // TODO: Implement other battle effects as needed
                    _ => {
                        info!("Battle effect not yet implemented: {:?}", battle_effect);
                    }
                }
            },
            Effect::Game(game_effect) => {
                match game_effect {
                    GameEffect::GainGold { amount } => {
                        self.gold += amount;
                        info!("Gained {} gold", amount);
                    },
                    GameEffect::SpendGold { amount } => {
                        if self.gold < amount {
                            info!("Not enough gold to spend {} (have {})", amount, self.gold);
                            // For now, we'll just spend what we can
                            self.gold = 0;
                        } else {
                            self.gold -= amount;
                            info!("Spent {} gold", amount);
                        }
                    },
                    GameEffect::ObtainRandomRelic => {
                        // TODO: Implement relic system
                        info!("Obtained a random relic (not yet implemented)");
                    },
                    GameEffect::EnterSelectCardsToUpgrade { count } => {
                        // For now, just transition to upgrade state
                        // TODO: Handle multi-card selection
                        self.set_game_state(GameState::SelectingCardFromDeck(crate::game::game_state::CardFromDeckTo::Upgrade));
                        info!("Enter card upgrade selection (count: {})", count);
                    },
                    GameEffect::EnterSelectCardsToRemove { count } => {
                        // For now, just transition to remove state
                        // TODO: Handle multi-card selection and shop context
                        self.set_game_state(GameState::SelectingCardFromDeck(crate::game::game_state::CardFromDeckTo::Remove));
                        info!("Enter card removal selection (count: {})", count);
                    },
                    GameEffect::EnterSelectCardsToTransform { count } => {
                        // TODO: Implement card transformation
                        info!("Enter card transform selection (count: {}) - not yet implemented", count);
                    },
                    GameEffect::UpgradeRandomCards { count } => {
                        let mut upgradeable_indices: Vec<usize> = self.deck.get_cards()
                            .iter()
                            .enumerate()
                            .filter(|(_, card)| !card.is_upgraded())
                            .map(|(i, _)| i)
                            .collect();

                        if upgradeable_indices.is_empty() {
                            info!("No cards to upgrade");
                            return;
                        }

                        // Sample random cards to upgrade (without replacement)
                        let num_to_upgrade = count.min(upgradeable_indices.len() as u32) as usize;
                        let mut indices_to_upgrade = Vec::new();

                        for _ in 0..num_to_upgrade {
                            let random_idx = rng.random_range(0..upgradeable_indices.len());
                            indices_to_upgrade.push(upgradeable_indices[random_idx]);
                            upgradeable_indices.remove(random_idx);
                        }

                        // Upgrade the selected cards
                        for &idx in &indices_to_upgrade {
                            if let Some(card) = self.deck.get_card(idx) {
                                let old_name = card.get_name();
                                let upgraded_card = card.clone().upgrade();
                                let new_name = upgraded_card.get_name();
                                self.deck.remove_card(idx);
                                self.deck.insert_card(idx, upgraded_card);
                                info!("Upgraded '{}' to '{}'", old_name, new_name);
                            }
                        }
                    },
                    GameEffect::TriggerCombatEvent => {
                        // TODO: Implement combat event triggering
                        info!("Trigger combat event - not yet implemented");
                    },
                }
            }
        }
    }

    /// Choose a node from available options based on path choice (0-based index)
    pub(crate) fn choose_node_from_path(&self, accessible_nodes: &[(u32, u32)], path_choice: usize) -> Result<(u32, u32), GameError> {
        if accessible_nodes.is_empty() {
            return Err(GameError::InvalidState);
        }

        // Get nodes and sort by position for consistent left/middle/right mapping
        let mut nodes_with_positions: Vec<((u32, u32), u32)> = accessible_nodes.iter()
            .filter_map(|&node_id| {
                self.map.get_node(node_id).map(|node| (node_id, node.position))
            })
            .collect();

        nodes_with_positions.sort_by_key(|&(_, position)| position);

        // Convert path choice to index with bounds checking
        let chosen_index = path_choice.min(nodes_with_positions.len() - 1);

        Ok(nodes_with_positions[chosen_index].0)
    }

    /// Get a list of upgradeable cards from the deck with their indices
    /// Returns a vector of (deck_index, card) tuples
    pub fn get_upgradeable_cards(&self) -> Vec<(usize, crate::game::card::Card)> {
        let mut upgradeable = Vec::new();

        for (index, card) in self.deck.get_cards().iter().enumerate() {
            // Only include cards that are not already upgraded
            if !card.is_upgraded() {
                upgradeable.push((index, card.clone()));
            }
        }

        upgradeable
    }

    /// Check if the deck has any upgradeable cards
    pub fn has_upgradeable_cards(&self) -> bool {
        self.deck.get_cards().iter().any(|card| !card.is_upgraded())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cards::ironclad::starter_deck::starter_deck, battle::battle_action::BattleAction};
    use crate::map::{Map, MapNode, NodeType};
    use crate::events::map_events::MapEvent;

    /// Create a simple test map: Start -> Combat -> Boss
    fn create_test_map() -> (Map, (u32, u32)) {
        let mut map = Map::new();

        // Create nodes
        let start_node = MapNode::new(0, 0, NodeType::Start);
        let combat_node = MapNode::new(1, 0, NodeType::Combat);
        let boss_node = MapNode::new(2, 0, NodeType::Boss);

        map.add_node(start_node);
        map.add_node(combat_node);
        map.add_node(boss_node);

        // Create edges
        map.add_edge((0, 0), (1, 0)).unwrap();
        map.add_edge((1, 0), (2, 0)).unwrap();

        // Set starting position
        map.set_starting_position((0, 0)).unwrap();

        (map, (0, 0)) // Return map and start node position
    }

    #[test]
    fn test_game_creation() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let game = Game::new(deck, global_info, map, 80, 80);
        
        assert_eq!(game.get_game_state(), &GameState::OnMap);
        assert!(!game.is_game_over());
        assert!(game.get_battle().is_none());
        assert_eq!(game.current_node_position, (0, 0));
        assert_eq!(game.get_player_hp(), 80);
        assert_eq!(game.get_player_max_hp(), 80);
    }

    #[test]
    fn test_choose_path_action() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();
        
        // Choose a path to start a battle
        let result = game.eval_action(GameAction::ChoosePath(1), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().outcome, GameOutcome::Continue);
        
        // Game should now be in battle (moved to combat node)
        assert!(matches!(game.get_game_state(), GameState::InBattle));
        assert!(game.get_battle().is_some());
        assert_eq!(game.current_node_position, (1, 0)); // Moved to combat node
    }

    #[test]
    fn test_battle_action_delegation() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, __node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();
        
        // Start a battle first
        game.eval_action(GameAction::ChoosePath(1), &mut rng).unwrap();
        
        // Try to end turn (battle action)
        let battle_action = GameAction::Battle(BattleAction::EndTurn);
        let result = game.eval_action(battle_action, &mut rng);
        
        // Should succeed as a valid battle action
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_state_actions() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();
        
        // Try battle action without starting battle
        let battle_action = GameAction::Battle(BattleAction::EndTurn);
        let result = game.eval_action(battle_action, &mut rng);
        
        // Should fail with NoBattle error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::NoBattle);
    }

    #[test]
    fn test_hp_syncing_between_game_and_battle() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 70, 80);
        let mut rng = rand::rng();
        
        // Verify initial state
        assert_eq!(game.get_player_hp(), 70);
        assert_eq!(game.get_player_max_hp(), 80);
        
        // Start a battle
        game.eval_action(GameAction::ChoosePath(1), &mut rng).unwrap();
        assert!(game.get_battle().is_some());
        
        // Verify battle player has correct HP
        if let Some(battle) = game.get_battle() {
            assert_eq!(battle.get_player().battle_info.get_hp(), 70);
            assert_eq!(battle.get_player().battle_info.get_max_hp(), 80);
        }
        
        // Simulate taking damage in battle by ending turn (enemy will attack)
        let initial_game_hp = game.get_player_hp();
        game.eval_action(GameAction::Battle(BattleAction::EndTurn), &mut rng).unwrap();
        
        // Test healing outside of battle
        game.heal_player(5);
        let healed_hp = game.get_player_hp();
        assert!(healed_hp >= initial_game_hp); // Should be healed or at max
        assert!(healed_hp <= game.get_player_max_hp()); // Should not exceed max
    }

    #[test]
    fn test_max_hp_management() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        
        // Test max HP increase
        game.increase_max_hp(10);
        assert_eq!(game.get_player_max_hp(), 90);
        assert_eq!(game.get_player_hp(), 90); // Should heal to full when at full HP
        
        // Test max HP increase when not at full HP
        game.set_player_hp(70);
        game.increase_max_hp(5);
        assert_eq!(game.get_player_max_hp(), 95);
        assert_eq!(game.get_player_hp(), 70); // Should not auto-heal when not at full
        
        // Test healing
        game.heal_player(100); // Try to overheal
        assert_eq!(game.get_player_hp(), 95); // Should cap at max
    }

    #[test]
    fn test_elite_encounter_spawns_gremlin_nob() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();
        
        // Create a simple map with an elite encounter
        let mut map = Map::new();
        let start_node = MapNode::new(0, 0, NodeType::Start);
        let elite_node = MapNode::new(1, 0, NodeType::Elite);
        map.add_node(start_node);
        map.add_node(elite_node);
        map.add_edge((0, 0), (1, 0)).unwrap();
        map.set_starting_position((0, 0)).unwrap();

        let mut game = Game::new(deck, global_info, map, 80, 80);

        // Move to elite node (path index 0 since there's only one available path)
        let result = game.eval_action(GameAction::ChoosePath(0), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().outcome, GameOutcome::Continue);
        
        // Should now be in battle with GremlinNob
        assert_eq!(game.get_game_state(), &GameState::InBattle);
        
        if let Some(battle) = game.get_battle() {
            let enemies = battle.get_enemies();
            assert_eq!(enemies.len(), 1);
            
            // Check that we have a GremlinNob
            match &enemies[0].enemy {
                crate::enemies::enemy_enum::EnemyEnum::GremlinNob(_) => {
                    // Success - we got a GremlinNob
                }
                _ => panic!("Expected GremlinNob enemy, got {:?}", enemies[0].enemy),
            }
        } else {
            panic!("Expected battle to be active");
        }
    }

    #[test]
    fn test_card_reward_selection_state() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Initially should not be in card reward selection
        assert!(!matches!(game.get_game_state(), GameState::CardRewardSelection(..)));
        assert!(game.get_card_reward_options().is_empty());

        // Start card reward selection with dummy reward state
        let test_reward_state = RewardState {
            gold_reward: 0,
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: None,
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        };
        game.start_card_reward_selection(&mut rng, test_reward_state);

        // Should now be in card reward selection state
        assert!(matches!(game.get_game_state(), GameState::CardRewardSelection(_)));
        assert_eq!(game.get_card_reward_options().len(), 3);

        // Verify all reward options are valid cards
        for card in game.get_card_reward_options() {
            assert!(card.get_cost() <= 3); // Reasonable cost check
            assert!(!card.get_name().is_empty()); // Should have a name
        }
    }

    #[test]
    fn test_select_card_reward_valid_action() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Start card reward selection with dummy reward state
        let test_reward_state = RewardState {
            gold_reward: 0,
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: None,
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        };
        game.start_card_reward_selection(&mut rng, test_reward_state);
        let initial_deck_size = game.deck.size();

        // Select first card reward
        let result = game.eval_action(GameAction::SelectCardReward(0), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().outcome, GameOutcome::Continue);

        // Should return to map state
        assert!(matches!(game.get_game_state(), GameState::OnMap));

        // Card should be added to deck
        assert_eq!(game.deck.size(), initial_deck_size + 1);

        // Reward options should be cleared
        assert!(game.get_card_reward_options().is_empty());
    }

    #[test]
    fn test_select_card_reward_invalid_state() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Try to select card reward without being in CardRewardSelection state
        let result = game.eval_action(GameAction::SelectCardReward(0), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidState);
    }

    #[test]
    fn test_select_card_reward_invalid_index() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Start card reward selection with dummy reward state
        let test_reward_state = RewardState {
            gold_reward: 0,
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: None,
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        };
        game.start_card_reward_selection(&mut rng, test_reward_state);

        // Try to select card with invalid index
        let result = game.eval_action(GameAction::SelectCardReward(5), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidCardIndex);

        // Try to select card with index equal to length
        let result = game.eval_action(GameAction::SelectCardReward(3), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidCardIndex);
    }

    #[test]
    fn test_card_reward_selection_different_options() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Generate card rewards multiple times
        let test_reward_state = RewardState {
            gold_reward: 0,
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: None,
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        };
        game.start_card_reward_selection(&mut rng, test_reward_state.clone());
        let first_options = game.get_card_reward_options().to_vec();
        game.set_game_state(GameState::OnMap); // Reset state

        game.start_card_reward_selection(&mut rng, test_reward_state);
        let second_options = game.get_card_reward_options().to_vec();

        // Should have different options (most likely due to randomness)
        // Note: This test might occasionally fail due to randomness, but it's very unlikely
        assert_ne!(first_options, second_options, "Card rewards should be randomized");
    }

    #[test]
    fn test_card_reward_selection_no_duplicates() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Start card reward selection with dummy reward state
        let test_reward_state = RewardState {
            gold_reward: 0,
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: None,
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        };
        game.start_card_reward_selection(&mut rng, test_reward_state);
        let reward_options = game.get_card_reward_options();

        // Check for duplicates in a single reward set
        let mut card_names = Vec::new();
        for card in reward_options {
            let name = card.get_name();
            assert!(!card_names.contains(&name), "Found duplicate card: {}", name);
            card_names.push(name);
        }
    }

    #[test]
    fn test_start_event() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);

        // Start an event
        game.start_event(MapEvent::BigFish);

        // Should now be in event state
        assert!(matches!(game.get_game_state(), GameState::InEvent(_, _)));

        // Should have current event set
        assert!(game.get_current_event().is_some());
        assert_eq!(game.get_current_event().unwrap(), &MapEvent::BigFish);

        // Should have choices available
        let choices = game.get_current_event_choices();
        assert_eq!(choices.len(), 3); // Big Fish has 3 choices

        // Check choice texts contain the expected keywords
        let choice_texts: Vec<String> = choices.iter().map(|c| c.text.clone()).collect();
        assert!(choice_texts.iter().any(|t| t.contains("Banana")));
        assert!(choice_texts.iter().any(|t| t.contains("Donut")));
        assert!(choice_texts.iter().any(|t| t.contains("Box")));
    }

    #[test]
    fn test_choose_event_banana() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Start an event
        game.start_event(MapEvent::BigFish);

        let initial_max_hp = game.get_player_max_hp();

        // Choose Banana (should be first choice)
        let result = game.eval_action(GameAction::ChooseEvent(0), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().outcome, GameOutcome::Continue);

        // Should return to map state
        assert_eq!(game.get_game_state(), &GameState::OnMap);

        // Should have gained 5 Max HP and healed to full
        assert_eq!(game.get_player_max_hp(), initial_max_hp + 5);
        assert_eq!(game.get_player_hp(), initial_max_hp + 5);

        // Event should be cleared
        assert!(game.get_current_event().is_none());
        assert!(game.get_current_event_choices().is_empty());
    }

    #[test]
    fn test_choose_event_donut() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 60, 90); // Start with low HP
        let mut rng = rand::rng();

        // Start an event
        game.start_event(MapEvent::BigFish);

        let initial_hp = game.get_player_hp();
        let initial_max_hp = game.get_player_max_hp();

        // Choose Donut (should be second choice)
        let result = game.eval_action(GameAction::ChooseEvent(1), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().outcome, GameOutcome::Continue);

        // Should return to map state
        assert_eq!(game.get_game_state(), &GameState::OnMap);

        // Should have healed 1/3 of Max HP (90 / 3 = 30)
        assert_eq!(game.get_player_max_hp(), initial_max_hp); // Max HP unchanged
        assert_eq!(game.get_player_hp(), initial_hp + 30);

        // Event should be cleared
        assert!(game.get_current_event().is_none());
        assert!(game.get_current_event_choices().is_empty());
    }

    #[test]
    fn test_choose_event_invalid_state() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Try to choose event without being in event state
        let result = game.eval_action(GameAction::ChooseEvent(0), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidState);
    }

    #[test]
    fn test_choose_event_invalid_index() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Start an event
        game.start_event(MapEvent::BigFish);

        // Try to choose invalid index
        let result = game.eval_action(GameAction::ChooseEvent(5), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidChoice);

        // Try to choose index equal to length
        let result = game.eval_action(GameAction::ChooseEvent(3), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidChoice);
    }

    #[test]
    fn test_event_node_triggers_event() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();

        // Create a map with an event node
        let mut map = Map::new();
        let start_node = MapNode::new(0, 0, NodeType::Start);
        let event_node = MapNode::new(1, 0, NodeType::Event);
        map.add_node(start_node);
        map.add_node(event_node);
        map.add_edge((0, 0), (1, 0)).unwrap();
        map.set_starting_position((0, 0)).unwrap();

        let mut game = Game::new(deck, global_info, map, 80, 80);

        // Move to event node
        let result = game.eval_action(GameAction::ChoosePath(0), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().outcome, GameOutcome::Continue);

        // Should now be in event state
        assert!(matches!(game.get_game_state(), GameState::InEvent(_, _)));

        // Should have BigFish event started
        assert!(game.get_current_event().is_some());
        assert_eq!(game.get_current_event().unwrap(), &MapEvent::BigFish);

        // Should have choices available
        let choices = game.get_current_event_choices();
        assert_eq!(choices.len(), 3);

        // Should be at the event node position
        assert_eq!(game.current_node_position, (1, 0));
    }

    #[test]
    fn test_rest_site_upgrade_starts_selection_state() {
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();

        // Move to rest site first
        let mut rest_map = Map::new();
        let start_node = MapNode::new(0, 0, NodeType::Start);
        let rest_node = MapNode::new(1, 0, NodeType::RestSite);
        rest_map.add_node(start_node);
        rest_map.add_node(rest_node);
        rest_map.add_edge((0, 0), (1, 0)).unwrap();
        rest_map.set_starting_position((0, 0)).unwrap();

        let deck = starter_deck();
        let mut game = Game::new(deck, global_info, rest_map, 80, 80);
        game.eval_action(GameAction::ChoosePath(0), &mut rng).unwrap();

        // Choose upgrade at rest site
        let result = game.eval_action(GameAction::RestSiteChoice(RestSiteAction::Upgrade), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().outcome, GameOutcome::Continue);

        // Should now be in upgrade selection state
        assert_eq!(game.get_game_state(), &GameState::SelectingCardFromDeck(crate::game::game_state::CardFromDeckTo::Upgrade));
    }

    #[test]
    fn test_select_card_to_upgrade_valid() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Set to upgrade selection state
        game.set_game_state(GameState::SelectingCardFromDeck(crate::game::game_state::CardFromDeckTo::Upgrade));

        // Get initial deck size and cards
        let initial_deck_size = game.deck.size();
        let upgradeable_cards = game.get_upgradeable_cards();
        assert!(!upgradeable_cards.is_empty(), "Should have upgradeable cards");

        // Find a Strike card to upgrade (they definitely change name when upgraded)
        let (card_index, original_card) = upgradeable_cards.iter()
            .find(|(_, card)| card.get_name() == "Strike")
            .expect("Should find a Strike card to upgrade")
            .clone();
        let original_name = original_card.get_name();
        assert_eq!(original_name, "Strike");

        // Upgrade the card
        let result = game.eval_action(GameAction::SelectCardFromDeck(card_index), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().outcome, GameOutcome::Continue);

        // Should return to map state
        assert_eq!(game.get_game_state(), &GameState::OnMap);

        // Deck size should remain the same
        assert_eq!(game.deck.size(), initial_deck_size);

        // Card should now be upgraded
        let upgraded_card = game.deck.get_card(card_index).unwrap();
        assert_ne!(upgraded_card.get_name(), original_name);
        assert!(upgraded_card.is_upgraded());
        assert_eq!(upgraded_card.get_name(), "Strike+");
    }

    #[test]
    fn test_select_card_to_upgrade_invalid_state() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Try to upgrade card without being in upgrade selection state
        let result = game.eval_action(GameAction::SelectCardFromDeck(0), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidState);
    }

    #[test]
    fn test_select_card_to_upgrade_invalid_index() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Set to upgrade selection state
        game.set_game_state(GameState::SelectingCardFromDeck(crate::game::game_state::CardFromDeckTo::Upgrade));

        // Try to upgrade with invalid index
        let result = game.eval_action(GameAction::SelectCardFromDeck(999), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidCardIndex);
    }

    #[test]
    fn test_get_upgradeable_cards() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let game = Game::new(deck, global_info, map, 80, 80);

        // Get upgradeable cards
        let upgradeable = game.get_upgradeable_cards();

        // Should have some upgradeable cards
        assert!(!upgradeable.is_empty());

        // All returned cards should not be upgraded
        for (index, card) in &upgradeable {
            assert!(!card.is_upgraded());
            // Check that the index is valid
            assert!(*index < game.deck.size());
        }

        // Check that deck card at returned index matches the card
        for (deck_index, card) in &upgradeable {
            let deck_card = game.deck.get_card(*deck_index).unwrap();
            assert_eq!(deck_card.get_card_enum(), card.get_card_enum());
            assert_eq!(deck_card.get_name(), card.get_name());
        }
    }

    #[test]
    fn test_has_upgradeable_cards() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let game = Game::new(deck, global_info, map, 80, 80);

        // Starter deck should have upgradeable cards
        assert!(game.has_upgradeable_cards());

        // Get upgradeable cards to verify
        let upgradeable = game.get_upgradeable_cards();
        assert_eq!(game.has_upgradeable_cards(), !upgradeable.is_empty());
    }

    #[test]
    fn test_already_upgraded_card_cannot_be_upgraded() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Add an already upgraded card to the deck
        let upgraded_strike = crate::cards::ironclad::strike::strike_upgraded();
        game.deck.add_card(upgraded_strike);
        let upgraded_card_index = game.deck.size() - 1;

        // Set to upgrade selection state
        game.set_game_state(GameState::SelectingCardFromDeck(crate::game::game_state::CardFromDeckTo::Upgrade));

        // Try to upgrade the already upgraded card
        let result = game.eval_action(GameAction::SelectCardFromDeck(upgraded_card_index), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidCardIndex);
    }

    #[test]
    fn test_potion_pool_initialization() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let game = Game::new(deck, global_info, map, 80, 80);

        // Check that potion pool is initialized correctly
        assert_eq!(game.potion_pool.get_combats_since_drop(), 0);
        assert_eq!(game.potion_pool.get_current_drop_chance(), 0.4);
    }

    #[test]
    fn test_reward_state_creation_with_potion_pool() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Create reward state for current node
        let reward_state = game.create_reward_state_for_current_node(&mut rng);

        // Check basic structure
        assert!(!reward_state.gold_claimed);
        assert!(!reward_state.potion_claimed);
        assert!(reward_state.card_selection_available);

        // Gold should be in expected range for normal combat
        assert!(reward_state.gold_reward >= 10 && reward_state.gold_reward <= 20);

        // Potion might be None or Some depending on RNG, but it should be valid
        if let Some(potion) = reward_state.potion_reward {
            // Verify the potion has a valid name (not empty)
            assert!(!potion.name().is_empty(), "Potion should have a valid name");
        }
    }

    #[test]
    fn test_potion_pool_progression() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);

        // Manually set counter to test progression
        game.potion_pool.reset_drop_counter();
        assert_eq!(game.potion_pool.get_current_drop_chance(), 0.4);

        // Simulate no drops for several combats
        game.potion_pool.set_combats_since_drop(1);
        assert!((game.potion_pool.get_current_drop_chance() - 0.5).abs() < f64::EPSILON);

        game.potion_pool.set_combats_since_drop(2);
        assert!((game.potion_pool.get_current_drop_chance() - 0.6).abs() < f64::EPSILON);

        // Should cap at 100%
        game.potion_pool.set_combats_since_drop(10);
        assert_eq!(game.potion_pool.get_current_drop_chance(), 1.0);
    }

    #[test]
    fn test_claim_potion_action() {
        use crate::potion::Potion;

        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Set up a reward state with an unclaimed potion
        let reward_state = RewardState {
            gold_reward: 15,
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: Some(Potion::StrengthPotion),
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        };
        game.set_game_state(GameState::Reward(reward_state));

        // Initially no potions in inventory
        assert_eq!(game.potions.potion_count(), 0);

        // Claim the potion
        let result = game.eval_action(GameAction::ClaimPotion, &mut rng);
        assert!(result.is_ok());

        // Should now have one potion
        assert_eq!(game.potions.potion_count(), 1);
        assert_eq!(game.potions.get_potion(0), Some(Potion::StrengthPotion));

        // Potion should be marked as claimed
        if let GameState::Reward(reward) = game.current_state() {
            assert!(reward.potion_claimed);
        }

        // Trying to claim again should fail
        let result = game.eval_action(GameAction::ClaimPotion, &mut rng);
        assert!(result.is_err());
    }

    #[test]
    fn test_claim_potion_full_inventory() {
        use crate::potion::Potion;

        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Fill up the potion inventory
        for _ in 0..3 {
            game.potions.add_potion(Potion::StrengthPotion);
        }
        assert!(game.potions.is_full());

        // Set up a reward state with an unclaimed potion
        let reward_state = RewardState {
            gold_reward: 15,
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: Some(Potion::StrengthPotion),
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        };
        game.set_game_state(GameState::Reward(reward_state));

        // Trying to claim potion should fail when inventory is full
        let result = game.eval_action(GameAction::ClaimPotion, &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidState);
    }

    #[test]
    fn test_cleric_event_uses_game_context() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 10 };
        let (map, _) = create_test_map();

        // Create game with damaged player (50/80 HP)
        let game = Game::new(deck, global_info, map, 50, 80);

        // Get choices for Cleric event
        let choices = game.get_event_choices(&MapEvent::TheCleric);

        // Should have 2 choices
        assert_eq!(choices.len(), 2);

        // First choice should show correct heal amount (30 HP) and gold cost
        // Gold cost at floor 10 = 35 + (10 * 2 / 5) = 35 + 4 = 39 gold
        let heal_choice = &choices[0];
        assert!(heal_choice.text.contains("30 HP"));
        assert!(heal_choice.text.contains("39 gold"));

        // Second choice should be "Leave"
        assert_eq!(choices[1].text, "Leave");
    }

    #[test]
    fn test_cleric_event_gold_cost_scales_with_floor() {
        let deck = starter_deck();
        let (map, _) = create_test_map();

        // Test at floor 0
        let global_info = GlobalInfo { ascention: 0, current_floor: 0 };
        let game = Game::new(deck.clone(), global_info, map.clone(), 50, 80);
        let choices = game.get_event_choices(&MapEvent::TheCleric);
        // Floor 0: 35 + (0 * 2 / 5) = 35
        assert!(choices[0].text.contains("35 gold"));

        // Test at floor 5
        let global_info = GlobalInfo { ascention: 0, current_floor: 5 };
        let game = Game::new(deck.clone(), global_info, map.clone(), 50, 80);
        let choices = game.get_event_choices(&MapEvent::TheCleric);
        // Floor 5: 35 + (5 * 2 / 5) = 35 + 2 = 37
        assert!(choices[0].text.contains("37 gold"));

        // Test at floor 15
        let global_info = GlobalInfo { ascention: 0, current_floor: 15 };
        let game = Game::new(deck.clone(), global_info, map.clone(), 50, 80);
        let choices = game.get_event_choices(&MapEvent::TheCleric);
        // Floor 15: 35 + (15 * 2 / 5) = 35 + 6 = 41
        assert!(choices[0].text.contains("41 gold"));
    }

    #[test]
    fn test_treasure_chest_rewards() {
        use crate::game::reward_state::{ChestType, RelicRarity};
        use crate::map::node::NodeType;
        use crate::map::Map;

        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 5 };
        let mut map = Map::new();
        let start_node = MapNode::new(0, 0, NodeType::Start);
        let treasure_node = MapNode::new(1, 0, NodeType::Treasure);
        map.add_node(start_node);
        map.add_node(treasure_node);
        map.add_edge((0, 0), (1, 0)).unwrap();
        map.set_starting_position((0, 0)).unwrap();

        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Move to treasure node
        let result = game.eval_action(GameAction::ChoosePath(0), &mut rng);
        assert!(result.is_ok());

        // Should now be in Reward state
        assert!(matches!(game.get_game_state(), GameState::Reward(_)));

        // Reward should have a relic but no card selection
        if let GameState::Reward(reward) = game.current_state() {
            assert!(reward.relic_reward.is_some());
            assert!(!reward.card_selection_available);
            assert!(!reward.relic_claimed);
        }
    }

    #[test]
    fn test_claim_relic_from_chest() {
        use crate::game::reward_state::{ChestType, RelicRarity};

        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Create a chest reward with a common relic
        let chest_reward = RewardState {
            gold_reward: 25,
            card_selection_available: false,
            gold_claimed: false,
            potion_reward: None,
            potion_claimed: false,
            relic_reward: Some(RelicRarity::Common),
            relic_claimed: false,
        };
        game.set_game_state(GameState::Reward(chest_reward));

        // Claim the relic
        let result = game.eval_action(GameAction::ClaimRelic, &mut rng);
        assert!(result.is_ok());

        // Relic should be marked as claimed
        if let GameState::Reward(reward) = game.current_state() {
            assert!(reward.relic_claimed);
            assert!(reward.relic_reward.is_none());
        }

        // Trying to claim again should fail
        let result = game.eval_action(GameAction::ClaimRelic, &mut rng);
        assert!(result.is_err());
    }

    #[test]
    fn test_chest_type_sampling_distribution() {
        use crate::game::reward_state::ChestType;

        let mut rng = rand::rng();
        let mut small_count = 0;
        let mut medium_count = 0;
        let mut large_count = 0;

        // Sample 1000 chests
        for _ in 0..1000 {
            match ChestType::sample(&mut rng) {
                ChestType::Small => small_count += 1,
                ChestType::Medium => medium_count += 1,
                ChestType::Large => large_count += 1,
            }
        }

        // Check distributions are roughly correct (50%, 33%, 17%)
        // Allow 10% margin of error
        assert!((small_count as f64 / 1000.0 - 0.50).abs() < 0.10, "Small: {}", small_count);
        assert!((medium_count as f64 / 1000.0 - 0.33).abs() < 0.10, "Medium: {}", medium_count);
        assert!((large_count as f64 / 1000.0 - 0.17).abs() < 0.10, "Large: {}", large_count);
    }

    #[test]
    fn test_chest_gold_rewards() {
        use crate::game::reward_state::ChestType;

        let mut rng = rand::rng();

        // Test small chest gold range
        for _ in 0..100 {
            let reward = ChestType::Small.create_reward_state(&mut rng);
            if reward.gold_reward > 0 {
                assert!(reward.gold_reward >= 23 && reward.gold_reward <= 27);
            }
        }

        // Test medium chest gold range
        for _ in 0..100 {
            let reward = ChestType::Medium.create_reward_state(&mut rng);
            if reward.gold_reward > 0 {
                assert!(reward.gold_reward >= 45 && reward.gold_reward <= 55);
            }
        }

        // Test large chest gold range
        for _ in 0..100 {
            let reward = ChestType::Large.create_reward_state(&mut rng);
            if reward.gold_reward > 0 {
                assert!(reward.gold_reward >= 68 && reward.gold_reward <= 82);
            }
        }
    }

    #[test]
    fn test_chest_relic_rarity_distribution() {
        use crate::game::reward_state::{ChestType, RelicRarity};

        let mut rng = rand::rng();

        // Test small chest relic distribution (75% Common, 25% Uncommon)
        let mut common = 0;
        let mut uncommon = 0;
        for _ in 0..1000 {
            let reward = ChestType::Small.create_reward_state(&mut rng);
            match reward.relic_reward.unwrap() {
                RelicRarity::Common => common += 1,
                RelicRarity::Uncommon => uncommon += 1,
                RelicRarity::Rare => panic!("Small chest should not give rare relics"),
            }
        }
        assert!((common as f64 / 1000.0 - 0.75).abs() < 0.10);
        assert!((uncommon as f64 / 1000.0 - 0.25).abs() < 0.10);

        // Test large chest relic distribution (75% Uncommon, 25% Rare)
        let mut uncommon = 0;
        let mut rare = 0;
        for _ in 0..1000 {
            let reward = ChestType::Large.create_reward_state(&mut rng);
            match reward.relic_reward.unwrap() {
                RelicRarity::Common => panic!("Large chest should not give common relics"),
                RelicRarity::Uncommon => uncommon += 1,
                RelicRarity::Rare => rare += 1,
            }
        }
        assert!((uncommon as f64 / 1000.0 - 0.75).abs() < 0.10);
        assert!((rare as f64 / 1000.0 - 0.25).abs() < 0.10);
    }

    #[test]
    fn test_skip_reward_from_beginning() {
        use crate::potion::Potion;

        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Set up a reward state with unclaimed rewards
        let reward_state = RewardState {
            gold_reward: 25,
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: Some(Potion::StrengthPotion),
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        };
        game.set_game_state(GameState::Reward(reward_state));

        // Player should be able to skip immediately without claiming anything
        let result = game.eval_action(GameAction::Skip, &mut rng);
        assert!(result.is_ok());

        // Should now be back on the map
        assert!(matches!(game.get_game_state(), GameState::OnMap));

        // Gold should not have been added (still at starting 99)
        assert_eq!(game.gold, 99);

        // Potion inventory should still be empty
        assert_eq!(game.potions.potion_count(), 0);
    }

    #[test]
    fn test_skip_treasure_chest_without_claiming() {
        use crate::game::reward_state::RelicRarity;

        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Set up a treasure chest reward with unclaimed relic and gold
        let chest_reward = RewardState {
            gold_reward: 50,
            card_selection_available: false,
            gold_claimed: false,
            potion_reward: None,
            potion_claimed: false,
            relic_reward: Some(RelicRarity::Uncommon),
            relic_claimed: false,
        };
        game.set_game_state(GameState::Reward(chest_reward));

        // Skip without claiming the relic or gold
        let result = game.eval_action(GameAction::Skip, &mut rng);
        assert!(result.is_ok());

        // Should be back on map
        assert!(matches!(game.get_game_state(), GameState::OnMap));

        // Gold should not have been claimed
        assert_eq!(game.gold, 99);
    }

    #[test]
    fn test_continue_to_map_after_partial_claims() {
        use crate::potion::Potion;

        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, _) = create_test_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);
        let mut rng = rand::rng();

        // Set up a reward state with multiple rewards
        let reward_state = RewardState {
            gold_reward: 30,
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: Some(Potion::StrengthPotion),
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        };
        game.set_game_state(GameState::Reward(reward_state));

        // Claim only the gold
        let result = game.eval_action(GameAction::ClaimGold, &mut rng);
        assert!(result.is_ok());
        assert_eq!(game.gold, 129); // 99 + 30

        // Now skip the rest (potion and card selection)
        let result = game.eval_action(GameAction::Skip, &mut rng);
        assert!(result.is_ok());

        // Should be back on map
        assert!(matches!(game.get_game_state(), GameState::OnMap));

        // Potion should not have been claimed
        assert_eq!(game.potions.potion_count(), 0);
    }
}