use crate::{events::SLSEvent, game::{action::{GameAction, RestSiteAction}, game_error::GameError, game_event::GameEvent, game_result::{GameResult, GameOutcome}, game_state::GameState}};
use crate::map::NodeType;
use crate::battle::{BattleResult, enemy_in_battle::EnemyInBattle, Battle};
use log::info;

use super::game::Game;

impl Game {
    pub fn eval_action(&mut self, action: GameAction, rng: &mut impl rand::Rng) -> Result<GameResult, GameError> {
        match action {
            GameAction::Battle(battle_action) => {
                // Delegate to battle if one is active
                if let Some(battle) = &mut self.battle {
                    match battle.eval_action(battle_action, rng) {
                        Ok(battle_result) => {
                            // Extract battle events from the result
                            let battle_events = match &battle_result {
                                BattleResult::Continued(events) |
                                BattleResult::Won(events) |
                                BattleResult::Lost(events) => events.clone(),
                            };

                            // Determine game outcome based on battle result
                            let outcome = match battle_result {
                                BattleResult::Continued(_) => GameOutcome::Continue,
                                BattleResult::Won(_) => {
                                    // Battle won, sync player state back (HP, gold, potions)
                                    if let Some(battle) = &self.battle {
                                        let final_state = battle.get_final_player_run_state(self.gold, self.relics.clone());
                                        self.set_player_hp(final_state.current_hp);
                                        self.player_max_hp = final_state.max_hp;
                                        self.gold = final_state.gold;
                                        self.potions = final_state.potions;
                                        // Note: relics remain unchanged as they are static during battle
                                    }

                                    self.battle = None;
                                    self.global_info.current_floor += 1;

                                    // Emit combat victory event for relic effects
                                    self.emit_game_event(GameEvent::CombatVictory);

                                    // Create reward state based on the node type that was just completed
                                    let reward_state = self.create_reward_state_for_current_node(rng);
                                    self.set_game_state(GameState::Reward(reward_state));

                                    GameOutcome::Continue
                                },
                                BattleResult::Lost(_) => {
                                    // Battle lost, sync player state back
                                    if let Some(battle) = &self.battle {
                                        let final_state = battle.get_final_player_run_state(self.gold, self.relics.clone());
                                        self.set_player_hp(final_state.current_hp);
                                        self.player_max_hp = final_state.max_hp;
                                        self.gold = final_state.gold;
                                        self.potions = final_state.potions;
                                    }
                                    self.battle = None;
                                    self.set_game_state(GameState::OnMap); // For now, just return to map
                                    GameOutcome::Defeat
                                },
                            };

                            // Convert battle events to game events
                            let game_events: Vec<GameEvent> = battle_events.into_iter()
                                .map(|battle_event| GameEvent::Battle(battle_event))
                                .collect();

                            Ok(GameResult {
                                outcome,
                                game_events,
                            })
                        },
                        Err(battle_error) => Err(GameError::Battle(battle_error)),
                    }
                } else {
                    Err(GameError::NoBattle)
                }
            },

            GameAction::ChoosePath(path_choice) => {
                // Only valid when on map
                if !matches!(self.current_state(), GameState::OnMap) {
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

                            // Create player state for battle
                            let player_state = crate::game::PlayerRunState::new_with_relics_and_potions(
                                self.player_hp,
                                self.player_max_hp,
                                self.gold,
                                self.relics.clone(),
                                self.potions.clone(),
                            );

                            // Start a battle
                            let battle = Battle::new_with_shuffle(self.deck.clone(), self.global_info, player_state, enemies, rng);
                            self.battle = Some(battle);
                            self.set_game_state(GameState::InBattle);
                        },
                        NodeType::Elite => {
                            // Elite encounters - sample from elite pool
                            let event = crate::events::encounter_events::sample_elite_encounter(&self.global_info, rng);
                            self.event_history.push(SLSEvent::EncounterEvent(event));

                            let enemy_enums = event.instantiate(rng, &self.global_info);
                            let enemies = enemy_enums.into_iter().map(|enemy| EnemyInBattle::new(enemy)).collect();

                            // Create player state for battle
                            let player_state = crate::game::PlayerRunState::new_with_relics_and_potions(
                                self.player_hp,
                                self.player_max_hp,
                                self.gold,
                                self.relics.clone(),
                                self.potions.clone(),
                            );

                            // Start a battle
                            let battle = Battle::new_with_shuffle(self.deck.clone(), self.global_info, player_state, enemies, rng);
                            self.battle = Some(battle);
                            self.set_game_state(GameState::InBattle);
                        },
                        NodeType::Boss => {
                            // Boss encounters - for now use regular encounters (TODO: implement boss)
                            let event = crate::events::encounter_events::sample_encounter_event(&self.global_info, &self.event_history, rng);
                            self.event_history.push(SLSEvent::EncounterEvent(event));

                            let enemy_enums = event.instantiate(rng, &self.global_info);
                            let enemies = enemy_enums.into_iter().map(|enemy| EnemyInBattle::new(enemy)).collect();

                            // Create player state for battle
                            let player_state = crate::game::PlayerRunState::new_with_relics_and_potions(
                                self.player_hp,
                                self.player_max_hp,
                                self.gold,
                                self.relics.clone(),
                                self.potions.clone(),
                            );

                            // Start a battle
                            let battle = Battle::new_with_shuffle(self.deck.clone(), self.global_info, player_state, enemies, rng);
                            self.battle = Some(battle);
                            self.set_game_state(GameState::InBattle);
                        },
                        NodeType::Event => {
                            // SLS Event - sample and start an event
                            let event = crate::events::map_events::sample_sls_event(&self.global_info, rng);
                            self.event_history.push(SLSEvent::MapEvent(event));

                            self.start_event(event);
                        },
                        NodeType::RestSite => {
                            // Rest site - enter rest site state
                            self.set_game_state(GameState::RestSite);
                        },
                        NodeType::Shop => {
                            // Shop - enter shop state with 5 cards for sale
                            self.start_shop(rng);
                        },
                        NodeType::Treasure => {
                            // Treasure chest - sample chest type and create reward state
                            use crate::game::reward_state::ChestType;
                            let chest_type = ChestType::sample(rng);
                            let reward_state = chest_type.create_reward_state(rng);
                            info!("Entered treasure room with {:?} chest", chest_type);
                            self.set_game_state(GameState::Reward(reward_state));
                        },
                        _ => {
                            // Other encounter types - for now just stay on map
                        }
                    }
                }

                Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
            },

            GameAction::ClaimGold => {
                // Only valid when in Reward state with unclaimed gold
                let mut reward_state = match self.current_state() {
                    GameState::Reward(reward) if !reward.gold_claimed => reward.clone(),
                    GameState::Reward(_) => return Err(GameError::InvalidState), // Gold already claimed
                    _ => return Err(GameError::InvalidState),
                };

                // Add gold to player
                self.gold += reward_state.gold_reward;
                info!("Claimed {} gold from combat reward", reward_state.gold_reward);

                // Mark gold as claimed
                reward_state.gold_claimed = true;
                self.set_game_state(GameState::Reward(reward_state));

                Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
            },

            GameAction::ClaimPotion => {
                // Only valid when in Reward state with unclaimed potion
                let mut reward_state = match self.current_state() {
                    GameState::Reward(reward) if !reward.potion_claimed && reward.potion_reward.is_some() => reward.clone(),
                    GameState::Reward(_) => return Err(GameError::InvalidState), // No potion or already claimed
                    _ => return Err(GameError::InvalidState),
                };

                // Get the potion and add it to player's inventory
                if let Some(potion) = reward_state.potion_reward {
                    if self.potions.is_full() {
                        // Inventory is full - player needs to discard or skip
                        // For now, just skip claiming (could add a discard potion action later)
                        info!("Potion inventory full, cannot claim potion");
                        return Err(GameError::InvalidState);
                    }

                    self.potions.add_potion(potion);
                    info!("Claimed {} from combat reward", potion.name());

                    // Mark potion as claimed
                    reward_state.potion_claimed = true;
                    self.set_game_state(GameState::Reward(reward_state));

                    Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
                } else {
                    Err(GameError::InvalidState)
                }
            },

            GameAction::ClaimRelic => {
                // Only valid when in Reward state with unclaimed relic
                let mut reward_state = match self.current_state() {
                    GameState::Reward(reward) if !reward.relic_claimed && reward.relic_reward.is_some() => reward.clone(),
                    GameState::Reward(_) => return Err(GameError::InvalidState), // No relic or already claimed
                    _ => return Err(GameError::InvalidState),
                };

                // Get the relic rarity and claim it
                if let Some(relic_rarity) = reward_state.claim_relic() {
                    // TODO: When relic system is implemented, sample an actual relic of the given rarity
                    // For now, just log the relic rarity that would be obtained
                    info!("Claimed {:?} rarity relic from treasure chest", relic_rarity);

                    self.set_game_state(GameState::Reward(reward_state));

                    Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
                } else {
                    Err(GameError::InvalidState)
                }
            },

            GameAction::RequestCardSelection => {
                // Only valid when in Reward state with card selection available
                match self.current_state() {
                    GameState::Reward(reward) if reward.card_selection_available => {
                        // Transition to card selection
                        self.start_card_reward_selection(rng, reward.clone());
                        Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
                    },
                    GameState::Reward(_) => Err(GameError::InvalidState), // Card selection not available
                    _ => Err(GameError::InvalidState),
                }
            },

            GameAction::Skip => {
                // Only valid when in Reward state or Shop state
                match self.current_state() {
                    GameState::Reward(_) => {
                        // Return to map without claiming remaining rewards
                        self.set_game_state(GameState::OnMap);
                        info!("Skipped remaining rewards, returning to map");
                        Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
                    },
                    GameState::Shop(_) => {
                        // Leave shop and return to map
                        self.set_game_state(GameState::OnMap);
                        info!("Left shop, returning to map");
                        Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
                    },
                    _ => return Err(GameError::InvalidState),
                }
            },

            GameAction::SelectCardReward(card_index) => {
                // Only valid when in CardRewardSelection state
                let reward_options = match self.current_state() {
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

                // Pop back to reward state
                if let Some(GameState::Reward(mut reward_state)) = self.pop_state() {
                    reward_state.card_selection_available = false;
                    self.set_game_state(GameState::Reward(reward_state));
                } else {
                    // Fallback if stack is empty
                    self.set_game_state(GameState::OnMap);
                }

                Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
            },

            GameAction::ChooseEvent(choice_index) => {
                // Only valid when in event state
                let (event, mut choices) = match self.current_state() {
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
                    crate::events::map_events::EventOutcome::Effects(effects) => {
                        // Apply all effects from the event choice
                        for effect in effects {
                            self.eval_effect(effect, rng);
                        }

                        // Event is complete, return to map
                        self.set_game_state(GameState::OnMap);
                        Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
                    },
                    crate::events::map_events::EventOutcome::NextChoices(new_choices) => {
                        // Transition to next set of choices
                        self.set_game_state(GameState::InEvent(event, new_choices));
                        Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
                    },
                }
            },

            GameAction::SelectCardFromDeck(card_index) => {
                // Only valid when in SelectingCardFromDeck state
                let card_operation = match self.current_state() {
                    GameState::SelectingCardFromDeck(operation) => operation.clone(),
                    _ => return Err(GameError::InvalidState),
                };

                // Validate card index
                if card_index >= self.deck.size() {
                    return Err(GameError::InvalidCardIndex);
                }

                match card_operation {
                    crate::game::game_state::CardFromDeckTo::Upgrade => {
                        // Get the card to upgrade
                        let card_to_upgrade = self.deck.get_card(card_index).cloned();
                        if let Some(card) = card_to_upgrade {
                            // Check if card is already upgraded
                            if card.is_upgraded() {
                                info!("Card '{}' is already upgraded", card.get_name());
                                return Err(GameError::InvalidCardIndex);
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
                        self.set_game_state(GameState::OnMap);
                        Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
                    },
                    crate::game::game_state::CardFromDeckTo::Remove => {
                        // Get the card to remove
                        let card_to_remove = self.deck.get_card(card_index).cloned();
                        if let Some(card) = card_to_remove {
                            info!("Removing '{}' from deck", card.get_name());

                            // Remove the card from deck
                            self.deck.remove_card(card_index);

                            info!("Card '{}' removed from deck. Deck size: {}", card.get_name(), self.deck.size());
                        } else {
                            return Err(GameError::InvalidCardIndex);
                        }

                        // Card removal is complete, return to shop
                        // Pop the SelectingCardFromDeck state to reveal the Shop state below
                        if let Some(GameState::Shop(shop_state)) = self.pop_state() {
                            self.set_game_state(GameState::Shop(shop_state));
                        } else {
                            // Fallback if shop state not found
                            info!("Shop state not found in stack, returning to map");
                            self.set_game_state(GameState::OnMap);
                        }
                        Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
                    },
                }
            },

            GameAction::RestSiteChoice(rest_site_action) => {
                // Only valid when in RestSite state
                if !matches!(self.current_state(), GameState::RestSite) {
                    return Err(GameError::InvalidState);
                }

                match rest_site_action {
                    RestSiteAction::Rest => {
                        // Heal 30% of max HP (minimum 15)
                        let heal_amount = ((self.player_max_hp as f32 * 0.3) as u32).max(15);
                        self.player_hp = (self.player_hp + heal_amount).min(self.player_max_hp);
                        info!("Player rested and healed {} HP", heal_amount);

                        // Rest site is complete, return to map
                        self.set_game_state(GameState::OnMap);
                    },
                    RestSiteAction::Upgrade => {
                        // Start card upgrade selection - don't return to map yet
                        self.set_game_state(GameState::SelectingCardFromDeck(crate::game::game_state::CardFromDeckTo::Upgrade));
                        info!("Card upgrade option chosen - select a card to upgrade");

                        // Don't return to map yet - wait for card selection
                        return Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() });
                    },
                }
                Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
            },

            GameAction::BuyCard(card_index) => {
                // Only valid when in Shop state
                let mut shop_state = match self.current_state() {
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
                self.set_game_state(GameState::Shop(shop_state));

                Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
            },

            GameAction::ShopAction(shop_action) => {
                // Only valid when in Shop state
                let mut shop_state = match self.current_state() {
                    GameState::Shop(shop_state) => shop_state.clone(),
                    _ => return Err(GameError::InvalidState),
                };

                match shop_action {
                    crate::game::action::ShopAction::BuyCard(card_index) => {
                        // This should use the existing BuyCard action instead
                        // For now, return an error
                        Err(GameError::InvalidState)
                    },
                    crate::game::action::ShopAction::EnterCardRemoval => {
                        // Check if player has enough gold
                        if self.gold < shop_state.card_removal_cost {
                            info!("Insufficient gold for card removal (have {}, need {})",
                                self.gold, shop_state.card_removal_cost);
                            return Err(GameError::NotEnoughGold);
                        }

                        // Check if card removal hasn't been used yet
                        if !shop_state.can_remove_card() {
                            info!("Card removal already used this shop visit");
                            return Err(GameError::InvalidState);
                        }

                        // Store the cost before moving shop_state
                        let removal_cost = shop_state.card_removal_cost;

                        // Deduct gold and mark removal as used
                        self.gold -= removal_cost;
                        shop_state.use_card_removal();

                        // Update shop state with removal marked as used
                        self.set_game_state(GameState::Shop(shop_state));

                        // Transition to card removal state
                        self.set_game_state(GameState::SelectingCardFromDeck(crate::game::game_state::CardFromDeckTo::Remove));
                        info!("Entered card removal from shop (paid {} gold)", removal_cost);
                        Ok(GameResult { outcome: GameOutcome::Continue, game_events: Vec::new() })
                    },
                }
            },
        }
    }
}
