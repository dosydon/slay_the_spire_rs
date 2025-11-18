use std::io::{self, Write};
use crate::battle::{Battle, BattleResult, BattleError, action::Action, target::Entity};
use crate::cards::ironclad::starter_deck::starter_deck;
use crate::cards::implemented_cards_deck::create_implemented_cards_deck;
use crate::enemies::enemy_enum::EnemyEnum;
use crate::battle::enemy_in_battle::EnemyInBattle;
use crate::game::global_info::GlobalInfo;
use crate::events::encounter_event::EncounterEvent;

pub struct BattleCli {
    pub(crate) battle: Battle,
}

impl BattleCli {
    /// Create a new battle CLI with a starter deck and selected encounter
    pub fn new() -> Self {
        Self::new_with_deck_choice(false)
    }

    /// Create a new battle CLI with optional test deck choice and selected encounter
    pub fn new_with_deck_choice(use_test_deck: bool) -> Self {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 20, current_floor: 1 };
        let deck = if use_test_deck {
            create_implemented_cards_deck()
        } else {
            starter_deck()
        };

        // Let user choose an encounter
        let encounter = Self::choose_encounter();
        let enemy_enums = encounter.instantiate(&mut rng, &global_info);
        let enemies = enemy_enums.into_iter().map(|enemy| EnemyInBattle::new(enemy)).collect();

        let battle = Battle::new_with_shuffle(deck, global_info, 80, 80, enemies, &mut rng);

        BattleCli { battle }
    }
    
    /// Let the user choose which encounter to fight
    fn choose_encounter() -> EncounterEvent {
        println!("\n=== Choose Your Encounter ===");
        println!("1. Two Louses (2 random louses)");
        println!("2. Jaw Worm (single tough enemy)");
        println!("3. Cultist (ritual caster)");
        println!("4. Small Slimes (mixed slime encounter)");
        println!("5. Gremlin Nob (elite with enrage)");
        print!("Enter your choice (1-5): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => EncounterEvent::TwoLouses,
            "2" => EncounterEvent::JawWorm,
            "3" => EncounterEvent::Cultist,
            "4" => EncounterEvent::SmallSlimes,
            "5" => EncounterEvent::GremlinNob,
            _ => {
                println!("Invalid choice, defaulting to Two Louses");
                EncounterEvent::TwoLouses
            }
        }
    }
    
    /// Start the battle simulation
    pub fn run(&mut self, rng: &mut impl rand::Rng) {
        println!("\n=== BATTLE START ===");
        self.display_battle_state();
        
        while !self.battle.is_battle_over() {
            match self.player_turn(rng) {
                Ok(BattleResult::Won) => {
                    println!("\n🎉 VICTORY! You defeated all enemies!");
                    break;
                },
                Ok(BattleResult::Lost) => {
                    println!("\n💀 DEFEAT! You have been slain!");
                    break;
                },
                Ok(BattleResult::Continued) => {
                    // Battle continues
                },
                Err(e) => {
                    println!("Error during battle: {:?}", e);
                    break;
                }
            }
        }
        
        println!("\n=== BATTLE END ===");
        self.display_final_state();
    }
    
    /// Handle a player turn
    fn player_turn(&mut self, rng: &mut impl rand::Rng) -> Result<BattleResult, BattleError> {
        println!("\n--- Your Turn ---");
        
        loop {
            self.display_available_actions();
            
            match self.get_player_action() {
                Some(action) => {
                    println!("Executing action: {:?}", action);
                    match self.battle.eval_action(action, rng) {
                        Ok(result) => {
                            self.display_battle_state();
                            return Ok(result);
                        },
                        Err(e) => {
                            println!("Invalid action: {:?}. Please try again.", e);
                            continue;
                        }
                    }
                },
                None => {
                    println!("Invalid input. Please try again.");
                    continue;
                }
            }
        }
    }
    
    /// Display the current battle state
    fn display_battle_state(&self) {
        println!("\n{}", "=".repeat(60));
        
        // Player state
        let player = self.battle.get_player();
        println!("🧙 PLAYER: HP {}/{} | Block {} | Energy {}", 
            player.battle_info.get_hp(),
            player.battle_info.get_max_hp(),
            player.get_block(),
            player.get_energy()
        );
        
        if player.battle_info.get_strength() != 0 {
            println!("   💪 Strength: {}", player.battle_info.get_strength());
        }
        if player.battle_info.is_vulnerable() {
            println!("   🔻 Vulnerable: {} turns", player.battle_info.get_vulnerable_turns());
        }
        if player.battle_info.is_weak() {
            println!("   😵‍💫 Weak: {} turns", player.battle_info.get_weak_turns());
        }
        if player.battle_info.is_frail() {
            println!("   🔻 Frail: {} turns", player.battle_info.get_frail_turns());
        }
        
        println!();
        
        // Enemy state
        for (i, enemy) in self.battle.get_enemies().iter().enumerate() {
            let name = match &enemy.enemy {
                EnemyEnum::RedLouse(_) => "Red Louse",
                EnemyEnum::GreenLouse(_) => "Green Louse", 
                EnemyEnum::JawWorm(_) => "Jaw Worm",
                EnemyEnum::Cultist(_) => "Cultist",
                EnemyEnum::SpikeSlimeS(_) => "Spike Slime (S)",
                EnemyEnum::SpikeSlimeM(_) => "Spike Slime (M)",
                EnemyEnum::AcidSlimeS(_) => "Acid Slime (S)",
                EnemyEnum::AcidSlimeM(_) => "Acid Slime (M)",
                EnemyEnum::GremlinNob(_) => "Gremlin Nob",
            };
            
            if enemy.battle_info.is_alive() {
                println!("👹 ENEMY {}: {} | HP {}/{} | Block {}", 
                    i + 1,
                    name,
                    enemy.battle_info.get_hp(),
                    enemy.battle_info.get_max_hp(),
                    enemy.battle_info.get_block()
                );
                
                // Display intended action if available
                if let Some((enemy_move, effects)) = self.battle.get_enemy_move_and_effects(i) {
                    let display_string = self.get_move_display_string(i, effects);
                    println!("   📋 Next: {}", display_string);
                }
                
                if enemy.battle_info.get_strength() != 0 {
                    println!("   💪 Strength: {}", enemy.battle_info.get_strength());
                }
                if enemy.battle_info.is_vulnerable() {
                    println!("   🔻 Vulnerable: {} turns", enemy.battle_info.get_vulnerable_turns());
                }
                if enemy.battle_info.is_weak() {
                    println!("   😵‍💫 Weak: {} turns", enemy.battle_info.get_weak_turns());
                }
                if enemy.battle_info.is_frail() {
                    println!("   🔻 Frail: {} turns", enemy.battle_info.get_frail_turns());
                }
            } else {
                println!("💀 ENEMY {}: {} | DEFEATED", i + 1, name);
            }
        }
        
        println!();
        
        // Hand
        println!("🃏 HAND:");
        let hand = self.battle.get_hand();
        for (i, card) in hand.iter().enumerate() {
            println!("   {}. {} (Cost: {})", i + 1, card.get_name(), card.get_cost());
        }
        
        println!("{}", "=".repeat(60));
    }
    
    /// Display available actions
    fn display_available_actions(&self) {
        let actions = self.battle.list_available_actions();
        println!("\n📋 Available Actions:");
        
        let mut action_index = 1;
        
        // Group actions by type for better display
        let mut card_actions = Vec::new();
        let mut end_turn_action = None;
        
        for action in &actions {
            match action {
                Action::PlayCard(card_idx, target) => {
                    card_actions.push((*card_idx, *target));
                },
                Action::SelectCardInHand(card_idx) => {
                    // Handle card selection for upgrade
                },
                Action::SelectCardInDiscard(card_idx) => {
                    // Handle card selection from discard pile
                },
                Action::EndTurn => {
                    end_turn_action = Some(action_index);
                }
            }
        }
        
        // Display card actions grouped by card
        let hand = self.battle.get_hand();
        for (card_idx, card) in hand.iter().enumerate() {
            let card_targets: Vec<_> = card_actions.iter()
                .filter(|(idx, _)| *idx == card_idx)
                .map(|(_, target)| *target)
                .collect();
            
            if !card_targets.is_empty() {
                println!("   {}. Play {} (Cost: {}) - Targets:", action_index, card.get_name(), card.get_cost());
                for (target_idx, target) in card_targets.iter().enumerate() {
                    match target {
                        Entity::Player => println!("      {}a. Target yourself", action_index),
                        Entity::Enemy(enemy_idx) => {
                            let enemy_name = match &self.battle.get_enemies()[*enemy_idx].enemy {
                                EnemyEnum::RedLouse(_) => "Red Louse",
                                EnemyEnum::GreenLouse(_) => "Green Louse",
                                EnemyEnum::JawWorm(_) => "Jaw Worm",
                                EnemyEnum::Cultist(_) => "Cultist",
                                EnemyEnum::SpikeSlimeS(_) => "Spike Slime (S)",
                                EnemyEnum::SpikeSlimeM(_) => "Spike Slime (M)",
                                EnemyEnum::AcidSlimeS(_) => "Acid Slime (S)",
                                EnemyEnum::AcidSlimeM(_) => "Acid Slime (M)",
                                EnemyEnum::GremlinNob(_) => "Gremlin Nob",
                            };
                            println!("      {}{}. Target {} {}", action_index, 
                                char::from(b'a' + target_idx as u8), enemy_name, enemy_idx + 1);
                        },
                        Entity::None => {} // Should not happen in available actions
                    }
                }
                action_index += 1;
            }
        }
        
        // Display end turn action
        if let Some(_) = end_turn_action {
            println!("   {}. End Turn", action_index);
        }
        
        println!();
    }
    
    /// Get player action input
    fn get_player_action(&self) -> Option<Action> {
        print!("Enter action (card number, action number, or 'end'): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        
        // Handle empty input
        if input.is_empty() {
            return None;
        }
        
        if input == "end" || input == "e" {
            return Some(Action::EndTurn);
        }
        
        // Calculate the EndTurn action number 
        let actions = self.battle.list_available_actions();
        let hand = self.battle.get_hand();
        let mut action_number = 1;
        
        // Count card actions to find EndTurn number
        for (card_idx, _) in hand.iter().enumerate() {
            let has_card_actions = actions.iter().any(|action| 
                matches!(action, Action::PlayCard(idx, _) if *idx == card_idx)
            );
            if has_card_actions {
                action_number += 1;
            }
        }
        
        // Check if input matches EndTurn action number
        if let Ok(num) = input.parse::<usize>() {
            if num == action_number && actions.contains(&Action::EndTurn) {
                return Some(Action::EndTurn);
            }
        }
        
        // Handle simple card number for single target cards
        if let Ok(card_num) = input.parse::<usize>() {
            if card_num == 0 { return None; }
            let card_idx = card_num - 1;
            
            // Get the first available target for this card
            let available_actions = self.battle.list_available_actions();
            let card_actions: Vec<_> = available_actions.iter()
                .filter_map(|action| match action {
                    Action::PlayCard(idx, target) if *idx == card_idx => Some(*target),
                    _ => None
                })
                .collect();
            
            if let Some(target) = card_actions.get(0) {
                return Some(Action::PlayCard(card_idx, *target));
            }
        }
        
        // Parse card action (e.g., "1a", "2b", etc.)
        if input.len() >= 2 {
            let card_part = &input[..input.len()-1];
            let target_part = input.chars().last()?;
            
            if let Ok(card_num) = card_part.parse::<usize>() {
                if card_num == 0 { return None; }
                let card_idx = card_num - 1;
                
                // Determine target from letter
                let target = match target_part {
                    'a' => {
                        // Need to check what 'a' means for this card
                        let available_actions = self.battle.list_available_actions();
                        let card_actions: Vec<_> = available_actions.iter()
                            .filter_map(|action| match action {
                                Action::PlayCard(idx, target) if *idx == card_idx => Some(*target),
                                _ => None
                            })
                            .collect();
                        
                        card_actions.get(0).copied()?
                    },
                    'b' => {
                        // Second target option
                        let available_actions = self.battle.list_available_actions();
                        let card_actions: Vec<_> = available_actions.iter()
                            .filter_map(|action| match action {
                                Action::PlayCard(idx, target) if *idx == card_idx => Some(*target),
                                _ => None
                            })
                            .collect();
                        
                        card_actions.get(1).copied()?
                    },
                    'c' => {
                        // Third target option
                        let available_actions = self.battle.list_available_actions();
                        let card_actions: Vec<_> = available_actions.iter()
                            .filter_map(|action| match action {
                                Action::PlayCard(idx, target) if *idx == card_idx => Some(*target),
                                _ => None
                            })
                            .collect();
                        
                        card_actions.get(2).copied()?
                    },
                    _ => return None,
                };
                
                return Some(Action::PlayCard(card_idx, target));
            }
        }
        
        None
    }
    
    /// Display final battle state
    fn display_final_state(&self) {
        self.display_battle_state();
        
        if self.battle.get_player().battle_info.get_hp() > 0 {
            println!("🎉 Congratulations! You survived the battle!");
        } else {
            println!("💀 Better luck next time!");
        }
    }
    
    /// Get display string for enemy move with calculated damage values
    fn get_move_display_string(&self, enemy_index: usize, effects: &[crate::game::effect::Effect]) -> String {
        use crate::battle::target::Entity;
        
        let mut parts = Vec::new();
        
        for effect in effects {
            match effect {
                crate::game::effect::Effect::AttackToTarget { amount, .. } => {
                    let calculated_damage = self.battle.calculate_incoming_damage(
                        Entity::Enemy(enemy_index), 
                        Entity::Player, 
                        *amount
                    );
                    
                    if calculated_damage != *amount {
                        parts.push(format!("🗡️ {} → {}", amount, calculated_damage));
                    } else {
                        parts.push(format!("🗡️ {}", amount));
                    }
                }
                crate::game::effect::Effect::AttackAllEnemies { amount, .. } => {
                    let calculated_damage = self.battle.calculate_incoming_damage(
                        Entity::Enemy(enemy_index), 
                        Entity::Player, 
                        *amount
                    );
                    
                    if calculated_damage != *amount {
                        parts.push(format!("🗡️ {} (all) → {}", amount, calculated_damage));
                    } else {
                        parts.push(format!("🗡️ {} (all)", amount));
                    }
                }
                crate::game::effect::Effect::GainDefense { amount } => {
                    parts.push(format!("🛡️ {}", amount));
                }
                crate::game::effect::Effect::GainStrength { amount } => {
                    parts.push(format!("💪 +{}", amount));
                }
                crate::game::effect::Effect::LoseStrengthSelf(amount) => {
                    parts.push(format!("💪 Self -{}", amount));
                }
                crate::game::effect::Effect::LoseStrengthTarget(amount) => {
                    parts.push(format!("💪 Target -{}", amount));
                }
                crate::game::effect::Effect::LoseStrengthAtEndOfTurn(amount) => {
                    parts.push(format!("⏰ -{} Strength (end turn)", amount));
                }
                crate::game::effect::Effect::GainRitual(amount) => {
                    parts.push(format!("✨ Ritual {}", amount));
                }
                crate::game::effect::Effect::ApplyWeak { duration } => {
                    parts.push(format!("🔻 Weak {}", duration));
                }
                crate::game::effect::Effect::ApplyVulnerable { duration } => {
                    parts.push(format!("🔻 Vulnerable {}", duration));
                }
                crate::game::effect::Effect::ApplyVulnerableAll { duration } => {
                    parts.push(format!("🔻 Vulnerable {} (all)", duration));
                }
                crate::game::effect::Effect::ApplyFrail { duration } => {
                    parts.push(format!("🔻 Frail {}", duration));
                }
                crate::game::effect::Effect::AddSlimed(count) => {
                    parts.push(format!("🐛 +{} Slimed", count));
                }
                crate::game::effect::Effect::AddCardToDrawPile(card) => {
                    parts.push(format!("➕ Add {} to Draw Pile", card.name()));
                }
                crate::game::effect::Effect::DrawCard { count } => {
                    parts.push(format!("🎴 Draw {}", count));
                }
                crate::game::effect::Effect::Exhaust => {
                    parts.push("💨 Exhaust".to_string());
                }
                crate::game::effect::Effect::ActivateEnrage(_) => {
                    parts.push("😤 Enrage".to_string());
                }
                crate::game::effect::Effect::ActivateEmbrace => {
                    parts.push("🤗 Embrace".to_string());
                }
                crate::game::effect::Effect::Heal(amount) => {
                    parts.push(format!("💚 Heal {}", amount));
                }
                crate::game::effect::Effect::LoseHp(amount) => {
                    parts.push(format!("💔 Lose {} HP", amount));
                }
                crate::game::effect::Effect::GainPlatedArmor(amount) => {
                    parts.push(format!("🛡️ +{} Plated Armor", amount));
                }
                crate::game::effect::Effect::DoubleBlock => {
                    parts.push("⚡ Double Block".to_string());
                }
                crate::game::effect::Effect::ActivateCombust(amount) => {
                    parts.push(format!("🔥 Combust ({} dmg/turn)", amount));
                }
                crate::game::effect::Effect::ApplyDamageReduction(percentage) => {
                    parts.push(format!("🛡️ -{}% Damage", percentage));
                }
                crate::game::effect::Effect::GainEnergy { amount } => {
                    parts.push(format!("⚡ Gain {} Energy", amount));
                }
                crate::game::effect::Effect::ApplyWeakAll { duration } => {
                    parts.push(format!("😵 Weak All ({} turns)", duration));
                }
                crate::game::effect::Effect::Ethereal => {
                    parts.push("👻 Ethereal".to_string());
                }
                crate::game::effect::Effect::AddCardToDiscard(card) => {
                    parts.push(format!("➕ Add {} to discard", card.name()));
                }
                crate::game::effect::Effect::EnterSelectCardInHand => {
                    parts.push("🔧 Select card to upgrade".to_string());
                }
                crate::game::effect::Effect::ActivateBrutality => {
                    parts.push("⚡ Brutality active".to_string());
                }
                crate::game::effect::Effect::PlayTopCard => {
                    parts.push("🎴 Play top card".to_string());
                }
                crate::game::effect::Effect::PlayTopCardAndExhaust => {
                    parts.push("🎴 Play & exhaust top card".to_string());
                }
                crate::game::effect::Effect::PutCardOnTopOfDrawPile(card) => {
                    parts.push(format!("⬆️ Put {} on top", card.name()));
                }
                crate::game::effect::Effect::PutRandomDiscardCardOnTop => {
                    parts.push("🔄 Put discard on top".to_string());
                }
                crate::game::effect::Effect::EnterSelectCardInDiscard => {
                    parts.push("📋 Select from discard".to_string());
                }
                crate::game::effect::Effect::ConditionalEffect(condition, effect) => {
                    parts.push(format!("❓If {:?}: {:?}", condition, effect));
                }
                crate::game::effect::Effect::EnterSelectCardInHandToPutOnDeck => {
                    parts.push("📋 Select card from hand to put on top of draw pile".to_string());
                }
                crate::game::effect::Effect::AttackToTargetWithBlock => {
                    parts.push("🗡️ Damage = Block".to_string());
                }
                crate::game::effect::Effect::ActivateCorruption => {
                    parts.push("🔥 Skills cost 0 and Exhaust".to_string());
                }
                crate::game::effect::Effect::ActivateMetallicize { amount } => {
                    parts.push(format!("🛡️ End of turn: Gain {} Block", amount));
                }
                crate::game::effect::Effect::ActivateFlameBarrier { damage } => {
                    parts.push(format!("🔥 When attacked: Deal {} damage", damage));
                }
                crate::game::effect::Effect::ActivateBurn { damage } => {
                    parts.push(format!("🔥 End of turn: Take {} damage", damage));
                }
                crate::game::effect::Effect::ActivateDemonForm { strength_per_turn } => {
                    parts.push(format!("😈 Start of turn: Gain {} Strength", strength_per_turn));
                }
                crate::game::effect::Effect::ActivateRage { block_per_attack } => {
                    parts.push(format!("🛡️ When Attack played: Gain {} Block", block_per_attack));
                }
                crate::game::effect::Effect::AddRandomAttackToHand => {
                    parts.push("⚔️ Add random Attack to hand".to_string());
                }
                crate::game::effect::Effect::ActivateEvolve => {
                    parts.push("🔄 Draw card when Status drawn".to_string());
                }
                crate::game::effect::Effect::DoubleStrength => {
                    parts.push("💪 Double Strength".to_string());
                }
            }
        }
        
        if parts.is_empty() {
            "Unknown Action".to_string()
        } else {
            parts.join(" ")
        }
    }
}