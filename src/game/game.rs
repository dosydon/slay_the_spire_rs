use crate::game::{global_info::GlobalInfo, action::{GameAction, PathChoice}, deck::Deck, map::{Map, MapError}};
use crate::battle::{Battle, BattleResult, BattleError};

/// The overall state of the game
#[derive(Debug, PartialEq)]
pub enum GameState {
    /// Player is currently in a battle
    InBattle,
    /// Player is on the map choosing their next path
    OnMap,
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
    pub current_node_id: u32,
}

impl Game {
    /// Create a new game with starting deck, global info, and map
    pub fn new(starting_deck: Deck, global_info: GlobalInfo, map: Map, start_node_id: u32) -> Self {
        Game {
            global_info,
            state: GameState::OnMap,
            deck: starting_deck,
            battle: None,
            map,
            current_node_id: start_node_id,
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
                            // Battle won, clean up and transition state
                            self.battle = None;
                            self.state = GameState::OnMap;
                            self.global_info.current_floor += 1;
                            Ok(GameResult::Continue)
                        },
                        Ok(BattleResult::Lost) => {
                            // Battle lost, game over
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
                let accessible_nodes = self.map.get_neighbors(self.current_node_id);
                if accessible_nodes.is_empty() {
                    return Err(GameError::InvalidState); // No paths available
                }
                
                // Choose node based on path choice
                let chosen_node_id = self.choose_node_from_path(&accessible_nodes, path_choice)?;
                
                // Move to the chosen node
                self.current_node_id = chosen_node_id;
                self.global_info.current_floor = self.get_current_node()
                    .map(|node| node.floor)
                    .unwrap_or(self.global_info.current_floor);
                
                // Check what type of encounter this is
                if let Some(node) = self.get_current_node() {
                    match node.node_type {
                        crate::game::map::NodeType::Combat | 
                        crate::game::map::NodeType::Elite |
                        crate::game::map::NodeType::Boss => {
                            // Start a battle
                            let battle = Battle::new(self.deck.clone(), self.global_info, rng);
                            self.battle = Some(battle);
                            self.state = GameState::InBattle;
                        },
                        _ => {
                            // Other encounter types - for now just stay on map
                            // Future: implement events, shops, rest sites, etc.
                        }
                    }
                }
                
                Ok(GameResult::Continue)
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
    pub fn get_current_node(&self) -> Option<&crate::game::map::MapNode> {
        self.map.get_node(self.current_node_id)
    }
    
    /// Get the map
    pub fn get_map(&self) -> &Map {
        &self.map
    }
    
    /// Choose a node from available options based on path choice
    fn choose_node_from_path(&self, accessible_nodes: &[u32], path_choice: PathChoice) -> Result<u32, GameError> {
        if accessible_nodes.is_empty() {
            return Err(GameError::InvalidState);
        }
        
        // Get nodes and sort by position for consistent left/middle/right mapping
        let mut nodes_with_positions: Vec<(u32, u32)> = accessible_nodes.iter()
            .filter_map(|&node_id| {
                self.map.get_node(node_id).map(|node| (node_id, node.position))
            })
            .collect();
        
        nodes_with_positions.sort_by_key(|&(_, position)| position);
        
        let chosen_index = match path_choice {
            PathChoice::Left => 0,
            PathChoice::Middle => {
                if nodes_with_positions.len() >= 2 {
                    nodes_with_positions.len() / 2
                } else {
                    0
                }
            },
            PathChoice::Right => nodes_with_positions.len() - 1,
        };
        
        Ok(nodes_with_positions[chosen_index].0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cards::ironclad::starter_deck::starter_deck, battle::action::Action};
    use crate::game::map::{Map, MapNode, NodeType};

    /// Create a simple test map: Start -> Combat -> Boss
    fn create_test_map() -> (Map, u32) {
        let mut map = Map::new();
        
        // Create nodes
        let start_node = MapNode::new(0, 0, 0, NodeType::Start);
        let combat_node = MapNode::new(1, 1, 0, NodeType::Combat);
        let boss_node = MapNode::new(2, 2, 0, NodeType::Boss);
        
        map.add_node(start_node);
        map.add_node(combat_node);
        map.add_node(boss_node);
        
        // Create edges
        map.add_edge(0, 1).unwrap();
        map.add_edge(1, 2).unwrap();
        
        (map, 0) // Return map and start node id
    }

    #[test]
    fn test_game_creation() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_id) = create_test_map();
        let game = Game::new(deck, global_info, map, start_node_id);
        
        assert_eq!(game.get_state(), &GameState::OnMap);
        assert!(!game.is_game_over());
        assert!(game.get_battle().is_none());
        assert_eq!(game.current_node_id, 0);
    }

    #[test]
    fn test_choose_path_action() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_id) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_id);
        let mut rng = rand::rng();
        
        // Choose a path to start a battle
        let result = game.eval_action(GameAction::ChoosePath(PathChoice::Middle), &mut rng);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GameResult::Continue);
        
        // Game should now be in battle (moved to combat node)
        assert!(matches!(game.get_state(), GameState::InBattle));
        assert!(game.get_battle().is_some());
        assert_eq!(game.current_node_id, 1); // Moved to combat node
    }

    #[test]
    fn test_battle_action_delegation() {
        let deck = starter_deck();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let (map, start_node_id) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_id);
        let mut rng = rand::rng();
        
        // Start a battle first
        game.eval_action(GameAction::ChoosePath(PathChoice::Middle), &mut rng).unwrap();
        
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
        let (map, start_node_id) = create_test_map();
        let mut game = Game::new(deck, global_info, map, start_node_id);
        let mut rng = rand::rng();
        
        // Try battle action without starting battle
        let battle_action = GameAction::Battle(Action::EndTurn);
        let result = game.eval_action(battle_action, &mut rng);
        
        // Should fail with NoBattle error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::NoBattle);
    }
}