use std::io::{self, Write};
use crate::game::{
    game::{Game, GameState, GameResult, GameError}, 
    action::{GameAction, PathChoice},
    global_info::GlobalInfo,
    map::{Map, MapNode, NodeType},
};
use crate::cards::ironclad::starter_deck::starter_deck;
use crate::battle_cli::BattleCli;

pub struct GameCli {
    game: Game,
    rng: rand::rngs::ThreadRng,
}

impl GameCli {
    /// Create a new game CLI with starter deck and generated map
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 20, current_floor: 1 };
        let deck = starter_deck();
        
        // Create a simple test map for now
        let (map, start_node) = Self::create_simple_map();
        
        let game = Game::new(deck, global_info, map, start_node, 80, 80);
        
        GameCli { game, rng }
    }
    
    /// Create a simple linear map for testing
    fn create_simple_map() -> (Map, u32) {
        let mut map = Map::new();
        
        // Floor 0: Start
        let start_node = MapNode::new(0, 0, 0, NodeType::Start);
        map.add_node(start_node);
        
        // Floor 1: Combat encounters
        let combat1 = MapNode::new(1, 1, 0, NodeType::Combat);
        let combat2 = MapNode::new(2, 1, 1, NodeType::Combat);
        let combat3 = MapNode::new(3, 1, 2, NodeType::Combat);
        map.add_node(combat1);
        map.add_node(combat2);
        map.add_node(combat3);
        
        // Floor 2: Elite or Rest
        let elite = MapNode::new(4, 2, 0, NodeType::Elite);
        let rest = MapNode::new(5, 2, 1, NodeType::RestSite);
        map.add_node(elite);
        map.add_node(rest);
        
        // Floor 3: Boss
        let boss = MapNode::new(6, 3, 0, NodeType::Boss);
        map.add_node(boss);
        
        // Connect nodes
        map.add_edge(0, 1).unwrap(); // Start -> Combat 1
        map.add_edge(0, 2).unwrap(); // Start -> Combat 2
        map.add_edge(0, 3).unwrap(); // Start -> Combat 3
        
        map.add_edge(1, 4).unwrap(); // Combat 1 -> Elite
        map.add_edge(2, 4).unwrap(); // Combat 2 -> Elite
        map.add_edge(2, 5).unwrap(); // Combat 2 -> Rest
        map.add_edge(3, 5).unwrap(); // Combat 3 -> Rest
        
        map.add_edge(4, 6).unwrap(); // Elite -> Boss
        map.add_edge(5, 6).unwrap(); // Rest -> Boss
        
        (map, 0) // Return map and start node ID
    }
    
    /// Start the game loop
    pub fn run(&mut self) {
        println!("\n🎮 Welcome to Slay the Spire!");
        println!("Ascension Level: {}", self.game.global_info.ascention);
        self.display_game_state();
        
        while !self.game.is_game_over() {
            match self.game.get_state() {
                GameState::OnMap => {
                    if let Err(e) = self.handle_map_phase() {
                        println!("Error during map phase: {:?}", e);
                        break;
                    }
                },
                GameState::InBattle => {
                    if let Err(e) = self.handle_battle_phase() {
                        println!("Error during battle phase: {:?}", e);
                        break;
                    }
                },
            }
            
            // Check if game ended
            if !self.game.is_player_alive() {
                println!("\n💀 GAME OVER - You have been defeated!");
                break;
            }
            
            self.display_game_state();
        }
        
        println!("\n🏁 Game ended. Thanks for playing!");
    }
    
    /// Handle map navigation phase
    fn handle_map_phase(&mut self) -> Result<(), GameError> {
        let current_node_info = self.game.get_current_node()
            .map(|node| (self.format_node_type(&node.node_type), node.floor));
        let neighbors = self.game.get_map().get_neighbors(self.game.current_node_id);
        
        if neighbors.is_empty() {
            println!("\n🎉 VICTORY! You've reached the end of your journey!");
            return Ok(());
        }
        
        println!("\n--- Map Navigation ---");
        if let Some((location_name, floor)) = current_node_info {
            println!("Current location: {} (Floor {})", location_name, floor);
        }
        
        println!("\nAvailable paths:");
        let mut paths = Vec::new();
        for &neighbor_id in &neighbors {
            if let Some(neighbor_node) = self.game.get_map().get_node(neighbor_id) {
                paths.push((neighbor_node.position, neighbor_node.floor, self.format_node_type(&neighbor_node.node_type)));
            }
        }
        paths.sort_by_key(|(pos, _, _)| *pos);
        
        for (i, (_, floor, node_type)) in paths.iter().enumerate() {
            let direction = match i {
                0 => "Left",
                1 if paths.len() == 3 => "Middle",
                _ => "Right",
            };
            println!("   {}. {} - {} (Floor {})", i + 1, direction, node_type, floor);
        }
        
        loop {
            print!("Choose your path (1-{}, or 'left'/'middle'/'right'): ", paths.len());
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();
            
            let choice = match input.as_str() {
                "1" | "left" | "l" => PathChoice::Left,
                "2" | "middle" | "m" => PathChoice::Middle,
                "3" | "right" | "r" => PathChoice::Right,
                _ => {
                    println!("Invalid choice. Please try again.");
                    continue;
                }
            };
            
            match self.game.eval_action(GameAction::ChoosePath(choice), &mut self.rng) {
                Ok(GameResult::Continue) => break,
                Ok(GameResult::Victory) => {
                    println!("\n🎉 VICTORY! You've completed the spire!");
                    return Ok(());
                },
                Ok(GameResult::Defeat) => {
                    println!("\n💀 DEFEAT! Your journey ends here.");
                    return Ok(());
                },
                Err(e) => {
                    println!("Invalid path choice: {:?}. Please try again.", e);
                    continue;
                }
            }
        }
        
        Ok(())
    }
    
    /// Handle battle phase using BattleCli
    fn handle_battle_phase(&mut self) -> Result<(), GameError> {
        if let Some(battle) = self.game.battle.take() {
            println!("\n⚔️  ENTERING COMBAT!");
            
            // Create a BattleCli with the current battle
            let mut battle_cli = BattleCli::from_existing_battle(battle);
            battle_cli.run(&mut self.rng);
            
            // Get the battle result and sync back to game
            let final_battle = battle_cli.into_battle();
            let player_hp = final_battle.get_final_player_hp();
            let battle_won = final_battle.get_enemies().iter().all(|e| !e.battle_info.is_alive());
            
            // Update game state
            self.game.set_player_hp(player_hp);
            self.game.state = GameState::OnMap;
            
            if battle_won {
                println!("\n🎊 Battle Won! Moving forward...");
                self.game.global_info.current_floor += 1;
            } else if player_hp == 0 {
                println!("\n💀 Battle Lost! Game Over.");
                return Ok(());
            }
        } else {
            return Err(GameError::NoBattle);
        }
        
        Ok(())
    }
    
    /// Display current game state
    fn display_game_state(&self) {
        println!("\n{}", "=".repeat(60));
        println!("🏥 PLAYER STATUS");
        println!("   HP: {}/{}", self.game.get_player_hp(), self.game.get_player_max_hp());
        println!("   Floor: {}", self.game.global_info.current_floor);
        
        if let Some(node) = self.game.get_current_node() {
            println!("   Location: {}", self.format_node_type(&node.node_type));
        }
        
        // Show deck summary
        println!("   Deck Size: {}", self.game.deck.size());
        println!("{}", "=".repeat(60));
    }
    
    /// Format node type for display
    fn format_node_type(&self, node_type: &NodeType) -> String {
        match node_type {
            NodeType::Start => "🏠 Start".to_string(),
            NodeType::Combat => "⚔️ Combat".to_string(),
            NodeType::Elite => "👹 Elite Combat".to_string(),
            NodeType::Boss => "🐉 Boss".to_string(),
            NodeType::RestSite => "🔥 Rest Site".to_string(),
            NodeType::Shop => "🏪 Shop".to_string(),
            NodeType::Event => "❓ Event".to_string(),
            NodeType::Treasure => "💰 Treasure".to_string(),
        }
    }
}

impl BattleCli {
    /// Create BattleCli from existing battle (for GameCli integration)
    pub fn from_existing_battle(battle: crate::battle::Battle) -> Self {
        BattleCli { battle }
    }
    
    /// Extract the battle from BattleCli (for GameCli integration)
    pub fn into_battle(self) -> crate::battle::Battle {
        self.battle
    }
}