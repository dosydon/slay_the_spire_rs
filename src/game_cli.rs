use std::io::Write;
use crate::game::{
    game::{Game, GameState, GameResult, GameError},
    action::GameAction,
    global_info::GlobalInfo,
    deck::Deck,
    map::{NodeType, test_map_large},
};
use crate::cards::ironclad::starter_deck::starter_deck;
use crate::cards::implemented_cards_deck::*;
use crate::battle_cli::BattleCli;

pub struct GameCli {
    game: Game,
    rng: rand::rngs::ThreadRng,
}

impl GameCli {
    /// Create a new game CLI with starter deck and generated map
    pub fn new() -> Self {
        Self::new_with_deck_choice(false)
    }

    /// Create a new game CLI with optional deck choice prompt
    pub fn new_with_deck_choice(prompt_for_deck: bool) -> Self {
        let rng = rand::rng();
        let global_info = GlobalInfo { ascention: 20, current_floor: 1 };

        let deck = if prompt_for_deck {
            Self::prompt_deck_choice()
        } else {
            starter_deck()
        };

        // Use the large test map
        let map = test_map_large();
        let start_node = (0, 0); // Start position based on test_map_large 0-indexing

        let mut game = Game::new(deck, global_info, map, start_node, 80, 80);

        // Add Burning Blood relic to the game
        game.add_relic(crate::relics::Relic::BurningBlood);

        GameCli { game, rng }
    }

    /// Prompt user to choose a deck
    fn prompt_deck_choice() -> Deck {
        println!("\n🎴 Choose your starting deck:");
        println!("1. Starter Deck (Standard Ironclad starter)");
        println!("2. Implemented Cards Deck (One of each implemented card)");
        println!("3. New Cards Deck (Only newly implemented cards)");
        println!("4. Power Cards Deck (Focus on power cards)");
        println!("5. Attack Cards Deck (Focus on attack cards)");

        loop {
            print!("Enter your choice (1-5): ");
            std::io::stdout().flush().ok();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();

            match input.trim() {
                "1" => {
                    println!("Selected Starter Deck");
                    return starter_deck();
                },
                "2" => {
                    println!("Selected Implemented Cards Deck");
                    return create_implemented_cards_deck();
                },
                "3" => {
                    println!("Selected New Cards Deck");
                    return create_new_cards_deck();
                },
                "4" => {
                    println!("Selected Power Cards Deck");
                    return create_power_cards_deck();
                },
                "5" => {
                    println!("Selected Attack Cards Deck");
                    return create_attack_cards_deck();
                },
                _ => {
                    println!("Invalid choice! Please enter 1, 2, 3, 4, or 5.");
                }
            }
        }
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
                GameState::CardRewardSelection => {
                    if let Err(e) = self.handle_card_reward_selection() {
                        println!("Error during card reward selection: {:?}", e);
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
        let neighbors = self.game.get_map().get_neighbors(self.game.current_node_position);

        if neighbors.is_empty() {
            println!("\n🎉 VICTORY! You've reached the end of your journey!");
            return Ok(());
        }

        println!("\n--- Map Navigation ---");

        // Display visual map
        self.display_visual_map();

        if let Some((location_name, floor)) = current_node_info {
            println!("\nCurrent location: {} (Floor {})", location_name, floor);
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
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            let path_index = match input.as_str() {
                "1" | "left" | "l" => 0,
                "2" | "middle" | "m" => 1,
                "3" | "right" | "r" => 2,
                _ => {
                    println!("Invalid choice. Please try again.");
                    continue;
                }
            };

            match self.game.eval_action(GameAction::ChoosePath(path_index), &mut self.rng) {
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

            if battle_won {
                println!("\n🎊 Battle Won! Moving forward...");
                self.game.global_info.current_floor += 1;

                // Trigger card reward selection through the Game's proper method
                self.game.start_card_reward_selection(&mut self.rng);
            } else if player_hp == 0 {
                println!("\n💀 Battle Lost! Game Over.");
                return Ok(());
            }
        } else {
            return Err(GameError::NoBattle);
        }

        Ok(())
    }

    /// Handle card reward selection phase
    fn handle_card_reward_selection(&mut self) -> Result<(), GameError> {
        println!("\n--- Card Reward Selection ---");
        println!("Choose one of the following cards to add to your deck:");

        let reward_options = self.game.get_card_reward_options().to_vec();
        if reward_options.is_empty() {
            println!("No card rewards available. This shouldn't happen - returning to map...");
            // Return error instead of directly setting state
            return Err(GameError::InvalidState);
        }

        // Display card options
        for (i, card) in reward_options.iter().enumerate() {
            println!("\n[{}] {} - Cost: {}", i + 1, card.get_name(), card.get_cost());
            println!("    Type: {:?}", card.get_card_type());

            // Show card effects in a simplified way
            for effect in card.get_effects() {
                match effect {
                    crate::game::effect::Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                        if *num_attacks == 1 {
                            if *strength_multiplier == 1 {
                                println!("    - Deal {} damage", amount);
                            } else {
                                println!("    - Deal {} damage ({}x Strength)", amount, strength_multiplier);
                            }
                        } else {
                            if *strength_multiplier == 1 {
                                println!("    - Deal {} damage {} times", amount, num_attacks);
                            } else {
                                println!("    - Deal {} damage {} times ({}x Strength)", amount, num_attacks, strength_multiplier);
                            }
                        }
                    },
                    crate::game::effect::Effect::GainDefense(amount) => {
                        println!("    - Gain {} Block", amount);
                    },
                    crate::game::effect::Effect::DrawCard(count) => {
                        println!("    - Draw {} card(s)", count);
                    },
                    _ => {
                        println!("    - {:?}", effect);
                    }
                }
            }
        }

        loop {
            print!("Choose your card (1-{}): ", reward_options.len());
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input.parse::<usize>() {
                Ok(choice) if choice >= 1 && choice <= reward_options.len() => {
                    let card_index = choice - 1;
                    match self.game.eval_action(GameAction::SelectCardReward(card_index), &mut self.rng) {
                        Ok(GameResult::Continue) => {
                            println!("\n✅ Card added to your deck!");
                            break;
                        },
                        Ok(GameResult::Victory) => {
                            println!("\n🎉 VICTORY! You've completed the spire!");
                            return Ok(());
                        },
                        Ok(GameResult::Defeat) => {
                            println!("\n💀 DEFEAT! Your journey ends here.");
                            return Ok(());
                        },
                        Err(e) => {
                            println!("Invalid choice: {:?}. Please try again.", e);
                            continue;
                        }
                    }
                },
                _ => {
                    println!("Invalid choice. Please enter a number between 1 and {}.", reward_options.len());
                    continue;
                }
            }
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

    /// Display a visual representation of the map
    fn display_visual_map(&self) {
        let map = self.game.get_map();
        let current_node_position = self.game.current_node_position;

        // Get all nodes and organize them by floor
        let mut nodes_by_floor: std::collections::HashMap<u32, Vec<_>> = std::collections::HashMap::new();
        let mut max_floor = 0;
        let mut max_position = 0;

        // Collect all nodes and find dimensions
        for node in map.get_all_nodes() {
            nodes_by_floor.entry(node.floor).or_insert_with(Vec::new).push(node);
            max_floor = max_floor.max(node.floor);
            max_position = max_position.max(node.position);
        }

        // Build the visual map
        println!("\n┌─────────────────────────────────────────────────┐");
        println!("│                    MAP                        │");
        println!("└─────────────────────────────────────────────────┘");
        println!();

        for floor in (0..=max_floor).rev() {
            if let Some(nodes_on_floor) = nodes_by_floor.get(&floor) {
                // Sort nodes by position
                let mut sorted_nodes = nodes_on_floor.clone();
                sorted_nodes.sort_by_key(|n| n.position);

                // Display nodes on this floor
                self.display_floor_nodes(&sorted_nodes, current_node_position);

                // Display connections to next floor (except for the top floor)
                if floor < max_floor {
                    self.display_floor_connections(&sorted_nodes, map, floor);
                }
            }
        }

        println!("\nLegend: 🟢 Current  🟤 Visited  ⭕ Unavailable");
    }

    /// Display nodes on a single floor
    fn display_floor_nodes(&self, nodes: &[&crate::game::map::MapNode], current_node_position: (u32, u32)) {
        // Find the maximum position to determine the width of the floor
        let max_position = nodes.iter().map(|n| n.position).max().unwrap_or(0);

        // Create a map from position to node for easy lookup
        let position_map: std::collections::HashMap<u32, _> = nodes.iter()
            .map(|node| (node.position, node))
            .collect();

        // Calculate total width needed (8 characters per position + spacing)
        let total_width = ((max_position + 1) * 8) as usize;

        // Build the three lines of the floor
        let mut line1 = String::with_capacity(total_width);
        let mut line2 = String::with_capacity(total_width);
        let mut line3 = String::with_capacity(total_width);

        for pos in 0..=max_position {
            // Add spacing for this position
            if pos > 0 {
                line1.push_str("        ");
                line2.push_str("        ");
                line3.push_str("        ");
            }

            if let Some(node) = position_map.get(&pos) {
                let (icon, is_current) = if node.id() == current_node_position {
                    (self.get_node_icon(&node.node_type, true), true)
                } else {
                    (self.get_node_icon(&node.node_type, false), false)
                };

                // Just show the icon, no box
                line1.push_str("   ");
                line2.push_str(&format!(" {} ", icon));
                line3.push_str("   ");
            } else {
                // Empty space for positions without nodes
                line1.push_str("     ");
                line2.push_str("     ");
                line3.push_str("     ");
            }
        }

        println!("{}", line1);
        println!("{}", line2);
        println!("{}", line3);
    }

    /// Display connections between floors using proper horizontal lines between positions
    fn display_floor_connections(&self, current_floor_nodes: &[&crate::game::map::MapNode], map: &crate::game::map::Map, current_floor: u32) {
        // Find the maximum position to determine the width
        let max_position = current_floor_nodes.iter().map(|n| n.position).max().unwrap_or(0);

        // Get all connections from current floor to next floor
        let mut connections: Vec<(u32, u32)> = Vec::new();
        for node in current_floor_nodes {
            let neighbors = map.get_neighbors(node.id());
            for &neighbor_id in &neighbors {
                if let Some(neighbor) = map.get_node(neighbor_id) {
                    if neighbor.floor == current_floor + 1 {
                        connections.push((node.position, neighbor.position));
                    }
                }
            }
        }

        if connections.is_empty() {
            return;
        }

        // Create 3 lines: vertical, horizontal, vertical
        for line_idx in 0..3 {
            let mut line = String::new();

            for pos in 0..=max_position {
                if pos > 0 {
                    line.push_str("        ");
                }

                match line_idx {
                    0 | 2 => {
                        // Top and bottom - vertical lines for any position with connections
                        if connections.iter().any(|(from, to)| *from == pos || *to == pos) {
                            line.push_str("   |    ");
                        } else {
                            line.push_str("        ");
                        }
                    },
                    1 => {
                        // Middle - draw the horizontal lines
                        // Check if this position connects to any other position
                        let connected_positions: Vec<u32> = connections.iter()
                            .filter(|(from, to)| *from == pos || *to == pos)
                            .map(|(from, to)| if *from == pos { *to } else { *from })
                            .collect();

                        if connected_positions.is_empty() {
                            line.push_str("        ");
                        } else {
                            // Build the horizontal line segment
                            let mut segment = String::new();

                            // Start with a vertical line
                            segment.push_str("|");

                            // Add horizontal line to the right if needed
                            if connected_positions.iter().any(|&p| p > pos) {
                                segment.push_str("------");
                            }

                            // Add horizontal line to the left if needed
                            if connected_positions.iter().any(|&p| p < pos) {
                                // This would need a more complex approach to show left connections
                                // For now, just show right connections
                            }

                            // Ensure 8 character width
                            while segment.len() < 8 {
                                segment.push(' ');
                            }

                            line.push_str(&segment[..8]);
                        }
                    },
                    _ => {
                        line.push_str("        ");
                    }
                }
            }

            if !line.trim().is_empty() {
                println!("{}", line);
            }
        }

        // Add spacing before next floor
        println!();
    }

    /// Get icon for node type
    fn get_node_icon(&self, node_type: &crate::game::map::NodeType, is_current: bool) -> &'static str {
        if is_current {
            return "🟢";
        }

        match node_type {
            crate::game::map::NodeType::Start => "🏠",
            crate::game::map::NodeType::Combat => "⚔️",
            crate::game::map::NodeType::Elite => "👹",
            crate::game::map::NodeType::Boss => "🐉",
            crate::game::map::NodeType::RestSite => "🔥",
            crate::game::map::NodeType::Shop => "🏪",
            crate::game::map::NodeType::Event => "❓",
            crate::game::map::NodeType::Treasure => "💰",
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