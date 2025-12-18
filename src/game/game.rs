use crate::{events::SLSEvent, game::{action::{GameAction, RestSiteAction}, card_reward::CardRewardPool, deck::Deck, game_event::{GameEvent, GameEventListener}, global_info::GlobalInfo}};
use crate::map::{Map, MapError, NodeType, MapNode};
use crate::battle::{Battle, BattleResult, BattleError, enemy_in_battle::EnemyInBattle};
use crate::events::map_events::{MapEvent, EventChoice, EventOutcome};
use log::{info, debug};

/// The overall state of the game
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    /// Player is currently in a battle
    InBattle,
    /// Player is on the map choosing their next path
    OnMap,
    /// Player is selecting a card reward from 3 options
    CardRewardSelection(Vec<crate::game::card::Card>),
    /// Player is in an SLS Event making choices
    InEvent(MapEvent, Vec<EventChoice>),
    /// Player is at a rest site
    RestSite,
    /// Player is selecting a card from their deck to upgrade
    SelectUpgradeFromDeck,
    /// Player is in a shop
    Shop(crate::game::shop::ShopState),
}

/// Errors that can occur during game actions
#[derive(Debug, Clone, PartialEq)]
pub enum GameError {
    /// Battle-specific error
    Battle(BattleError),
    /// Map-specific error
    Map(MapError),
    /// Action not valid in current game state
    InvalidState,
    /// Invalid card index
    InvalidCardIndex,
    /// Invalid choice index
    InvalidChoice,
    /// Not enough gold to purchase
    NotEnoughGold,
    /// No active battle
    NoBattle,
}

/// Result of a game action
#[derive(Debug, Clone, PartialEq)]
pub enum GameResult {
    /// Action completed, game continues
    Continue,
    /// Run completed successfully
    Victory,
    /// Run ended in defeat
    Defeat,
}

pub struct Game {
    pub global_info: GlobalInfo,
    pub state: GameState,
    pub deck: Deck,
    pub battle: Option<Battle>,
    pub map: Map,
    pub current_node_position: (u32, u32),
    pub player_hp: u32,
    pub player_max_hp: u32,
    pub gold: u32,
    relics: Vec<crate::relics::Relic>,
    game_event_listeners: Vec<Box<dyn GameEventListener>>,
    event_history: Vec<SLSEvent>,
}

impl Game {
    /// Create a new game with starting deck, global info, and map
    pub fn new(starting_deck: Deck, global_info: GlobalInfo, map: Map, start_node_position: (u32, u32), starting_hp: u32, max_hp: u32) -> Self {
        Game {
            global_info,
            state: GameState::OnMap,
            deck: starting_deck,
            battle: None,
            map,
            current_node_position: start_node_position,
            player_hp: starting_hp,
            player_max_hp: max_hp,
            gold: 99, // Starting gold (Ironclad starts with 99 gold)
            relics: Vec::new(),
            game_event_listeners: Vec::new(),
            event_history: Vec::new(),
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
                crate::game::effect::Effect::Heal(amount) => {
                    self.player_hp = (self.player_hp + amount).min(self.player_max_hp);
                }
                // Handle other effects as needed
                _ => {}
            }
        }
    }

    /// Evaluate a game action and update game state accordingly
    pub fn eval_action(&mut self, action: GameAction, rng: &mut impl rand::Rng) -> Result<GameResult, GameError> {
        match action {
            GameAction::Battle(battle_action) => {
                // Delegate to battle if one is active
                if let Some(battle) = &mut self.battle {
                    match battle.eval_action(battle_action, rng) {
                        Ok(BattleResult::Continued) => Ok(GameResult::Continue),
                        Ok(BattleResult::Won) => {
                            // Battle won, sync HP and gold back
                            // Extract values before modifying self
                            let (final_hp, gold_to_lose) = if let Some(battle) = &self.battle {
                                let hp = battle.get_final_player_hp();
                                let enemies_escaped = battle.get_enemies().iter().any(|e| e.battle_info.has_escaped());
                                let gold_lost = if enemies_escaped {
                                    battle.get_gold_stolen()
                                } else {
                                    0 // Gold is returned if all enemies were killed
                                };
                                (hp, gold_lost)
                            } else {
                                (self.player_hp, 0)
                            };

                            self.set_player_hp(final_hp);
                            self.gold = self.gold.saturating_sub(gold_to_lose);
                            self.battle = None;
                            self.global_info.current_floor += 1;

                            // Emit combat victory event for relic effects
                            self.emit_game_event(GameEvent::CombatVictory);

                            // Start card reward selection after victory
                            self.start_card_reward_selection(rng);

                            Ok(GameResult::Continue)
                        },
                        Ok(BattleResult::Lost) => {
                            // Battle lost, sync HP back and game over
                            if let Some(battle) = &self.battle {
                                self.set_player_hp(battle.get_final_player_hp());
                            }
                            self.battle = None;
                            self.state = GameState::OnMap; // For now, just return to map
                            Ok(GameResult::Defeat)
                        },
                        Err(battle_error) => Err(GameError::Battle(battle_error)),
                    }
                } else {
                    Err(GameError::NoBattle)
                }
            },
            
            GameAction::ChoosePath(path_choice) => {
                // Only valid when on map
                if !matches!(self.state, GameState::OnMap) {
                    return Err(GameError::InvalidState);
                }
                
                // Get accessible nodes from current position
                let accessible_nodes = self.map.get_neighbors(self.current_node_position);
                if accessible_nodes.is_empty() {
                    return Err(GameError::InvalidState); // No paths available
                }
                
                // Choose node based on path choice
                let chosen_node_id = self.choose_node_from_path(&accessible_nodes, path_choice)?;
                
                // Move to the chosen node
                self.current_node_position = chosen_node_id;
                self.global_info.current_floor = self.get_current_node()
                    .map(|node| node.floor)
                    .unwrap_or(self.global_info.current_floor);
                
                // Check what type of encounter this is
                if let Some(node) = self.get_current_node() {
                    match node.node_type {
                        NodeType::Combat => {
                            let event = crate::events::encounter_events::sample_encounter_event(&self.global_info, &self.event_history, rng);
                            self.event_history.push(SLSEvent::EncounterEvent(event));

                            let enemy_enums = event.instantiate(rng, &self.global_info);
                            let enemies = enemy_enums.into_iter().map(|enemy| EnemyInBattle::new(enemy)).collect();
                            
                            // Start a battle
                            let battle = Battle::new_with_shuffle(self.deck.clone(), self.global_info, self.player_hp, self.player_max_hp, enemies, rng);
                            self.battle = Some(battle);
                            self.state = GameState::InBattle;
                        },
                        NodeType::Elite => {
                            // Elite encounters - sample from elite pool
                            let event = crate::events::encounter_events::sample_elite_encounter(&self.global_info, rng);
                            self.event_history.push(SLSEvent::EncounterEvent(event));

                            let enemy_enums = event.instantiate(rng, &self.global_info);
                            let enemies = enemy_enums.into_iter().map(|enemy| EnemyInBattle::new(enemy)).collect();

                            // Start a battle
                            let battle = Battle::new_with_shuffle(self.deck.clone(), self.global_info, self.player_hp, self.player_max_hp, enemies, rng);
                            self.battle = Some(battle);
                            self.state = GameState::InBattle;
                        },
                        NodeType::Boss => {
                            // Boss encounters - for now use regular encounters (TODO: implement boss)
                            let event = crate::events::encounter_events::sample_encounter_event(&self.global_info, &self.event_history, rng);
                            self.event_history.push(SLSEvent::EncounterEvent(event));

                            let enemy_enums = event.instantiate(rng, &self.global_info);
                            let enemies = enemy_enums.into_iter().map(|enemy| EnemyInBattle::new(enemy)).collect();
                            
                            // Start a battle
                            let battle = Battle::new_with_shuffle(self.deck.clone(), self.global_info, self.player_hp, self.player_max_hp, enemies, rng);
                            self.battle = Some(battle);
                            self.state = GameState::InBattle;
                        },
                        NodeType::Event => {
                            // SLS Event - sample and start an event
                            let event = crate::events::map_events::sample_sls_event(&self.global_info, rng);
                            self.event_history.push(SLSEvent::MapEvent(event));

                            self.start_event(event);
                        },
                        NodeType::RestSite => {
                            // Rest site - enter rest site state
                            self.state = GameState::RestSite;
                        },
                        NodeType::Shop => {
                            // Shop - enter shop state with 5 cards for sale
                            self.start_shop(rng);
                        },
                        _ => {
                            // Other encounter types - for now just stay on map
                            // Future: implement treasure rooms, etc.
                        }
                    }
                }
  
                Ok(GameResult::Continue)
            },

            GameAction::SelectCardReward(card_index) => {
                // Only valid when in CardRewardSelection state
                let reward_options = match &self.state {
                    GameState::CardRewardSelection(options) => options.clone(),
                    _ => return Err(GameError::InvalidState),
                };

                // Validate card index
                if card_index >= reward_options.len() {
                    return Err(GameError::InvalidCardIndex);
                }

                // Add selected card to deck
                let selected_card = reward_options[card_index].clone();
                self.deck.add_card(selected_card);

                // Return to map
                self.state = GameState::OnMap;

                Ok(GameResult::Continue)
            },

            GameAction::ChooseEvent(choice_index) => {
                // Only valid when in event state
                let (event, mut choices) = match &self.state {
                    GameState::InEvent(event, choices) => (event.clone(), choices.clone()),
                    _ => return Err(GameError::InvalidState),
                };

                // Validate choice index
                if choice_index >= choices.len() {
                    return Err(GameError::InvalidChoice);
                }

                // Process the chosen outcome
                let choice = choices.remove(choice_index);
                match choice.outcome {
                    EventOutcome::Effects(effects) => {
                        // Apply all effects from the event choice
                        for effect in effects {
                            self.apply_event_effect(effect);
                        }

                        // Event is complete, return to map
                        self.state = GameState::OnMap;
                        Ok(GameResult::Continue)
                    },
                    EventOutcome::NextChoices(new_choices) => {
                        // Transition to next set of choices
                        self.state = GameState::InEvent(event, new_choices);
                        Ok(GameResult::Continue)
                    },
                }
            },

            GameAction::SelectCardToUpgrade(card_index) => {
                // Only valid when in SelectUpgradeFromDeck state
                if !matches!(self.state, GameState::SelectUpgradeFromDeck) {
                    return Err(GameError::InvalidState);
                }

                // Validate card index
                if card_index >= self.deck.size() {
                    return Err(GameError::InvalidCardIndex);
                }

                // Get the card to upgrade
                let card_to_upgrade = self.deck.get_card(card_index).cloned();
                if let Some(card) = card_to_upgrade {
                    // Check if card is already upgraded
                    if card.is_upgraded() {
                        info!("Card '{}' is already upgraded", card.get_name());
                        return Err(GameError::InvalidCardIndex); // Or create a new error type
                    }

                    // Get names before upgrade
                    let old_name = card.get_name();

                    // Upgrade the card
                    let upgraded_card = card.upgrade();
                    let new_name = upgraded_card.get_name();

                    // Remove the old card and add the upgraded version at the same position
                    self.deck.remove_card(card_index);
                    self.deck.insert_card(card_index, upgraded_card);

                    info!("Upgraded '{}' to '{}'", old_name, new_name);
                } else {
                    return Err(GameError::InvalidCardIndex);
                }

                // Card upgrade is complete, return to map
                self.state = GameState::OnMap;
                Ok(GameResult::Continue)
            },

            GameAction::RestSiteChoice(rest_site_action) => {
                // Only valid when in RestSite state
                if !matches!(self.state, GameState::RestSite) {
                    return Err(GameError::InvalidState);
                }

                match rest_site_action {
                    RestSiteAction::Rest => {
                        // Heal 30% of max HP (minimum 15)
                        let heal_amount = ((self.player_max_hp as f32 * 0.3) as u32).max(15);
                        self.player_hp = (self.player_hp + heal_amount).min(self.player_max_hp);
                        info!("Player rested and healed {} HP", heal_amount);

                        // Rest site is complete, return to map
                        self.state = GameState::OnMap;
                    },
                    RestSiteAction::ObtainGold => {
                        // Obtain 15 gold
                        self.gold += 15;
                        info!("Player obtained 15 gold");

                        // Rest site is complete, return to map
                        self.state = GameState::OnMap;
                    },
                    RestSiteAction::Remove => {
                        // TODO: Implement card removal UI and logic
                        info!("Card removal option chosen (not implemented)");

                        // Rest site is complete, return to map
                        self.state = GameState::OnMap;
                    },
                    RestSiteAction::Upgrade => {
                        // Start card upgrade selection - don't return to map yet
                        self.state = GameState::SelectUpgradeFromDeck;
                        info!("Card upgrade option chosen - select a card to upgrade");

                        // Don't return to map yet - wait for card selection
                        return Ok(GameResult::Continue);
                    },
                }
                Ok(GameResult::Continue)
            },

            GameAction::BuyCard(card_index) => {
                // Only valid when in Shop state
                let mut shop_state = match &self.state {
                    GameState::Shop(shop_state) => shop_state.clone(),
                    _ => return Err(GameError::InvalidState),
                };

                // Validate card index
                if card_index >= shop_state.card_count() {
                    return Err(GameError::InvalidCardIndex);
                }

                // Get card and price
                let card_price = shop_state.get_card_price(card_index)
                    .ok_or(GameError::InvalidCardIndex)?;

                // Check if player has enough gold
                if self.gold < card_price {
                    return Err(GameError::NotEnoughGold);
                }

                // Purchase the card
                let purchased_card = shop_state.purchase_card(card_index)
                    .ok_or(GameError::InvalidCardIndex)?;

                // Deduct gold and add card to deck
                self.gold -= card_price;
                self.deck.add_card(purchased_card);

                info!("Purchased card for {} gold. Remaining gold: {}", card_price, self.gold);

                // Update shop state
                self.state = GameState::Shop(shop_state);

                Ok(GameResult::Continue)
            },

            GameAction::LeaveShop => {
                // Only valid when in Shop state
                match &self.state {
                    GameState::Shop(_) => {
                        // Leave shop and return to map
                        self.state = GameState::OnMap;
                        info!("Left shop, returning to map");
                        Ok(GameResult::Continue)
                    },
                    _ => return Err(GameError::InvalidState),
                }
            },
        }
    }
    
    /// Get the current game state
    pub fn get_state(&self) -> &GameState {
        &self.state
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
    
    /// Check if player is alive
    pub fn is_player_alive(&self) -> bool {
        self.player_hp > 0
    }

    /// Start card reward selection - generates 3 random card options
    pub fn start_card_reward_selection(&mut self, rng: &mut impl rand::Rng) {
        let card_pool = CardRewardPool::new();
        let reward_options = card_pool.generate_reward_options(rng);
        info!("Generated {} card reward options", reward_options.len());
        for (i, card) in reward_options.iter().enumerate() {
            debug!("  Option {}: {} (Cost: {})", i + 1, card.get_name(), card.get_cost());
        }
        self.state = GameState::CardRewardSelection(reward_options);
    }

    /// Get the current card reward options (only valid in CardRewardSelection state)
    pub fn get_card_reward_options(&self) -> &[crate::game::card::Card] {
        match &self.state {
            GameState::CardRewardSelection(options) => options,
            _ => &[],
        }
    }

    /// Start an SLS Event
    pub fn start_event(&mut self, event: MapEvent) {
        let choices = event.get_choices();
        self.state = GameState::InEvent(event, choices);
        info!("Started event: {}", event.get_description());
    }

    /// Get the current event (only valid in InEvent state)
    pub fn get_current_event(&self) -> Option<&MapEvent> {
        match &self.state {
            GameState::InEvent(event, _) => Some(event),
            _ => None,
        }
    }

    /// Get the current event choices (only valid in InEvent state)
    pub fn get_current_event_choices(&self) -> &[EventChoice] {
        match &self.state {
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
        self.state = GameState::Shop(shop_state);
    }

    /// Get the current shop state (only valid in Shop state)
    pub fn get_shop_state(&self) -> Option<&crate::game::shop::ShopState> {
        match &self.state {
            GameState::Shop(shop_state) => Some(shop_state),
            _ => None,
        }
    }

    /// Apply a single event effect to the player/game
    fn apply_event_effect(&mut self, effect: crate::game::effect::Effect) {
        use crate::game::effect::Effect;

        match effect {
            Effect::Heal(amount) => {
                // Handle special case: amount 0 means heal 1/3 of max HP
                let heal_amount = if amount == 0 {
                    self.player_max_hp / 3
                } else {
                    amount
                };
                self.player_hp = (self.player_hp + heal_amount).min(self.player_max_hp);
                info!("Healed {} HP", heal_amount);
            },
            Effect::HealAndIncreaseMaxHp(amount) => {
                self.player_hp = (self.player_hp + amount).min(self.player_max_hp + amount);
                self.player_max_hp += amount;
                info!("Gained {} Max HP and healed to full", amount);
            },
            Effect::LoseHp(amount) => {
                self.player_hp = self.player_hp.saturating_sub(amount);
                info!("Lost {} HP", amount);
            },
            // TODO: Implement other event effects as needed
            // For now, most effects are logged but not implemented
            _ => {
                info!("Event effect not yet implemented: {:?}", effect);
            }
        }
    }

    /// Choose a node from available options based on path choice (0-based index)
    fn choose_node_from_path(&self, accessible_nodes: &[(u32, u32)], path_choice: usize) -> Result<(u32, u32), GameError> {
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
    use crate::{cards::ironclad::starter_deck::starter_deck, battle::action::Action};
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

        (map, (0, 0)) // Return map and start node position
    }

    #[test]
    fn test_game_creation() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_position) = create_test_map();
        let game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        
        assert_eq!(game.get_state(), &GameState::OnMap);
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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();
        
        // Choose a path to start a battle
        let result = game.eval_action(GameAction::ChoosePath(1), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GameResult::Continue);
        
        // Game should now be in battle (moved to combat node)
        assert!(matches!(game.get_state(), GameState::InBattle));
        assert!(game.get_battle().is_some());
        assert_eq!(game.current_node_position, (1, 0)); // Moved to combat node
    }

    #[test]
    fn test_battle_action_delegation() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();
        
        // Start a battle first
        game.eval_action(GameAction::ChoosePath(1), &mut rng).unwrap();
        
        // Try to end turn (battle action)
        let battle_action = GameAction::Battle(Action::EndTurn);
        let result = game.eval_action(battle_action, &mut rng);
        
        // Should succeed as a valid battle action
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_state_actions() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();
        
        // Try battle action without starting battle
        let battle_action = GameAction::Battle(Action::EndTurn);
        let result = game.eval_action(battle_action, &mut rng);
        
        // Should fail with NoBattle error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::NoBattle);
    }

    #[test]
    fn test_hp_syncing_between_game_and_battle() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 70, 80);
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
        game.eval_action(GameAction::Battle(Action::EndTurn), &mut rng).unwrap();
        
        // Check if HP was affected during battle
        if let Some(battle) = game.get_battle() {
            let battle_hp = battle.get_final_player_hp();
            // Game HP should still be the old value until battle ends
            assert_eq!(game.get_player_hp(), initial_game_hp);
        }
        
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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        
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

        let mut game = Game::new(deck, global_info, map, (0, 0), 80, 80);
        
        // Move to elite node
        let result = game.eval_action(GameAction::ChoosePath(1), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GameResult::Continue);
        
        // Should now be in battle with GremlinNob
        assert_eq!(game.get_state(), &GameState::InBattle);
        
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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();

        // Initially should not be in card reward selection
        assert!(!matches!(game.get_state(), GameState::CardRewardSelection(_)));
        assert!(game.get_card_reward_options().is_empty());

        // Start card reward selection
        game.start_card_reward_selection(&mut rng);

        // Should now be in card reward selection state
        assert!(matches!(game.get_state(), GameState::CardRewardSelection(_)));
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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();

        // Start card reward selection
        game.start_card_reward_selection(&mut rng);
        let initial_deck_size = game.deck.size();
        let reward_options = game.get_card_reward_options().to_vec();

        // Select first card reward
        let result = game.eval_action(GameAction::SelectCardReward(0), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GameResult::Continue);

        // Should return to map state
        assert!(matches!(game.get_state(), GameState::OnMap));

        // Card should be added to deck
        assert_eq!(game.deck.size(), initial_deck_size + 1);

        // Reward options should be cleared
        assert!(game.get_card_reward_options().is_empty());
    }

    #[test]
    fn test_select_card_reward_invalid_state() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();

        // Start card reward selection
        game.start_card_reward_selection(&mut rng);

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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();

        // Generate card rewards multiple times
        game.start_card_reward_selection(&mut rng);
        let first_options = game.get_card_reward_options().to_vec();
        game.state = GameState::OnMap; // Reset state

        game.start_card_reward_selection(&mut rng);
        let second_options = game.get_card_reward_options().to_vec();

        // Should have different options (most likely due to randomness)
        // Note: This test might occasionally fail due to randomness, but it's very unlikely
        assert_ne!(first_options, second_options, "Card rewards should be randomized");
    }

    #[test]
    fn test_card_reward_selection_no_duplicates() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();

        // Start card reward selection
        game.start_card_reward_selection(&mut rng);
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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);

        // Start an event
        game.start_event(MapEvent::BigFish);

        // Should now be in event state
        assert!(matches!(game.get_state(), GameState::InEvent(_, _)));

        // Should have current event set
        assert!(game.get_current_event().is_some());
        assert_eq!(game.get_current_event().unwrap(), &MapEvent::BigFish);

        // Should have choices available
        let choices = game.get_current_event_choices();
        assert_eq!(choices.len(), 3); // Big Fish has 3 choices

        // Check choice texts
        let choice_texts: Vec<String> = choices.iter().map(|c| c.text.clone()).collect();
        assert!(choice_texts.contains(&"Banana".to_string()));
        assert!(choice_texts.contains(&"Donut".to_string()));
        assert!(choice_texts.contains(&"Box".to_string()));
    }

    #[test]
    fn test_choose_event_banana() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();

        // Start an event
        game.start_event(MapEvent::BigFish);

        let initial_hp = game.get_player_hp();
        let initial_max_hp = game.get_player_max_hp();

        // Choose Banana (should be first choice)
        let result = game.eval_action(GameAction::ChooseEvent(0), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GameResult::Continue);

        // Should return to map state
        assert_eq!(game.get_state(), &GameState::OnMap);

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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 60, 90); // Start with low HP
        let mut rng = rand::rng();

        // Start an event
        game.start_event(MapEvent::BigFish);

        let initial_hp = game.get_player_hp();
        let initial_max_hp = game.get_player_max_hp();

        // Choose Donut (should be second choice)
        let result = game.eval_action(GameAction::ChooseEvent(1), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GameResult::Continue);

        // Should return to map state
        assert_eq!(game.get_state(), &GameState::OnMap);

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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
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

        let mut game = Game::new(deck, global_info, map, (0, 0), 80, 80);

        // Move to event node
        let result = game.eval_action(GameAction::ChoosePath(0), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GameResult::Continue);

        // Should now be in event state
        assert!(matches!(game.get_state(), GameState::InEvent(_, _)));

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
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();

        // Move to rest site first
        let mut rest_map = Map::new();
        let start_node = MapNode::new(0, 0, NodeType::Start);
        let rest_node = MapNode::new(1, 0, NodeType::RestSite);
        rest_map.add_node(start_node);
        rest_map.add_node(rest_node);
        rest_map.add_edge((0, 0), (1, 0)).unwrap();

        let deck = starter_deck();
        let mut game = Game::new(deck, global_info, rest_map, (0, 0), 80, 80);
        game.eval_action(GameAction::ChoosePath(0), &mut rng).unwrap();

        // Choose upgrade at rest site
        let result = game.eval_action(GameAction::RestSiteChoice(RestSiteAction::Upgrade), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GameResult::Continue);

        // Should now be in upgrade selection state
        assert_eq!(game.get_state(), &GameState::SelectUpgradeFromDeck);
    }

    #[test]
    fn test_select_card_to_upgrade_valid() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();

        // Set to upgrade selection state
        game.state = GameState::SelectUpgradeFromDeck;

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
        let result = game.eval_action(GameAction::SelectCardToUpgrade(card_index), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GameResult::Continue);

        // Should return to map state
        assert_eq!(game.get_state(), &GameState::OnMap);

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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();

        // Try to upgrade card without being in upgrade selection state
        let result = game.eval_action(GameAction::SelectCardToUpgrade(0), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidState);
    }

    #[test]
    fn test_select_card_to_upgrade_invalid_index() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();

        // Set to upgrade selection state
        game.state = GameState::SelectUpgradeFromDeck;

        // Try to upgrade with invalid index
        let result = game.eval_action(GameAction::SelectCardToUpgrade(999), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidCardIndex);
    }

    #[test]
    fn test_get_upgradeable_cards() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);

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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);

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
        let (map, start_node_position) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_position, 80, 80);
        let mut rng = rand::rng();

        // Add an already upgraded card to the deck
        let upgraded_strike = crate::cards::ironclad::strike::strike_upgraded();
        game.deck.add_card(upgraded_strike);
        let upgraded_card_index = game.deck.size() - 1;

        // Set to upgrade selection state
        game.state = GameState::SelectUpgradeFromDeck;

        // Try to upgrade the already upgraded card
        let result = game.eval_action(GameAction::SelectCardToUpgrade(upgraded_card_index), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidCardIndex);
    }
}