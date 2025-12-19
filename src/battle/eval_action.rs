use super::Battle;
use crate::battle::{battle_action::BattleAction, target::Entity, BattleResult, BattleError};
use crate::game::{effect::Effect, card::Card, card_type::CardType};

impl Battle {
    /// Evaluate a player action and return the battle result
    pub fn eval_action(&mut self, action: BattleAction, rng: &mut impl rand::Rng) -> Result<BattleResult, BattleError> {
        if self.is_battle_over() {
            return Err(BattleError::GameAlreadyOver);
        }

        // Validate action based on current battle state
        match &self.battle_state {
            crate::battle::battle_state::BattleState::PlayerTurn => {
                match action {
                    BattleAction::SelectCardInHand(_) => return Err(BattleError::InvalidAction),
                    _ => {}
                }
            }
            crate::battle::battle_state::BattleState::SelectCardInHand => {
                match action {
                    BattleAction::PlayCard(_, _) => return Err(BattleError::InvalidAction),
                    BattleAction::EndTurn => return Err(BattleError::InvalidAction),
                    _ => {}
                }
            }
            crate::battle::battle_state::BattleState::SelectCardInDiscard => {
                match action {
                    BattleAction::PlayCard(_, _) => return Err(BattleError::InvalidAction),
                    BattleAction::EndTurn => return Err(BattleError::InvalidAction),
                    BattleAction::SelectCardInHand(_) => return Err(BattleError::InvalidAction),
                    _ => {}
                }
            }
            crate::battle::battle_state::BattleState::SelectCardInHandToPutOnDeck => {
                match action {
                    BattleAction::PlayCard(_, _) => return Err(BattleError::InvalidAction),
                    BattleAction::EndTurn => return Err(BattleError::InvalidAction),
                    BattleAction::SelectCardInDiscard(_) => return Err(BattleError::InvalidAction),
                    _ => {}
                }
            }
            crate::battle::battle_state::BattleState::SelectCardToDuplicate { .. } => {
                match action {
                    BattleAction::PlayCard(_, _) => return Err(BattleError::InvalidAction),
                    BattleAction::EndTurn => return Err(BattleError::InvalidAction),
                    BattleAction::SelectCardInHand(_) => return Err(BattleError::InvalidAction),
                    BattleAction::SelectCardInDiscard(_) => return Err(BattleError::InvalidAction),
                    _ => {}
                }
            }
            crate::battle::battle_state::BattleState::SelectCardInExhaust => {
                match action {
                    BattleAction::PlayCard(_, _) => return Err(BattleError::InvalidAction),
                    BattleAction::EndTurn => return Err(BattleError::InvalidAction),
                    BattleAction::SelectCardInHand(_) => return Err(BattleError::InvalidAction),
                    BattleAction::SelectCardInDiscard(_) => return Err(BattleError::InvalidAction),
                    _ => {}
                }
            }
        }

        match action {
            BattleAction::PlayCard(idx, target) => {
                if idx >= self.cards.hand_size() {
                    return Err(BattleError::CardNotInHand);
                }

                if !self.is_valid_target(&target) {
                    return Err(BattleError::InvalidTarget);
                }

                let hand = self.cards.get_hand();
                let card = &hand[idx];
                if !self.player.spend_energy(card.get_cost()) {
                    return Err(BattleError::NotEnoughEnergy);
                }

                // Restore energy since we're checking but not actually spending yet
                self.player.battle_info.gain_energy(card.get_cost());

                self.play_card(idx, target)?;
            }
            BattleAction::UsePotion(slot_index, target) => {
                // Use the potion from the specified slot
                self.use_potion(slot_index, target)?;
            }
            BattleAction::EndTurn => {
                self.at_end_of_player_turn();

                self.at_start_of_enemy_turn();
                let global_info_clone = self.global_info.clone();
                self.process_enemy_effects(rng, &global_info_clone);
                self.at_end_of_enemy_turn();

                self.at_start_of_player_turn(rng);
            }
            BattleAction::SelectCardInHand(card_index) => {
                if card_index >= self.cards.hand_size() {
                    return Err(BattleError::CardNotInHand);
                }

                // Check which state we're in to determine behavior
                match &self.battle_state {
                    crate::battle::battle_state::BattleState::SelectCardInHand => {
                        // Get the selected card and upgrade it
                        let hand = self.cards.get_hand();
                        let card = &hand[card_index];

                        // Only upgrade if not already upgraded
                        if !card.is_upgraded() {
                            let upgraded_card = card.clone().upgrade();

                            // Replace the card in hand with the upgraded version
                            self.cards.replace_card_in_hand(card_index, upgraded_card);
                        }
                    }
                    crate::battle::battle_state::BattleState::SelectCardInHandToPutOnDeck => {
                        // Get the selected card from hand and put it on top of draw pile
                        if let Some(card_to_move) = self.cards.remove_card_from_hand(card_index) {
                            // Put on top of draw pile
                            self.cards.put_card_on_top_of_deck(card_to_move);
                        }
                    }
                    _ => {
                        return Err(BattleError::InvalidAction);
                    }
                }

                // Return to player turn state
                self.battle_state = crate::battle::battle_state::BattleState::PlayerTurn;
            }
            BattleAction::SelectCardInDiscard(card_index) => {
                if card_index >= self.cards.discard_pile_size() {
                    return Err(BattleError::CardNotInDiscardPile);
                }

                // Get the selected card from discard pile and put it on top of draw pile
                if let Some(card_to_move) = self.cards.remove_from_discard_pile(card_index) {
                    // Put on top of draw pile
                    self.cards.put_card_on_top_of_deck(card_to_move);
                }

                // Return to player turn state
                self.battle_state = crate::battle::battle_state::BattleState::PlayerTurn;
            }
            BattleAction::SelectCardToDuplicate(card_index) => {
                if card_index >= self.cards.hand_size() {
                    return Err(BattleError::CardNotInHand);
                }

                // Get the number of copies from the current battle state
                if let crate::battle::battle_state::BattleState::SelectCardToDuplicate { copies } = &self.battle_state {
                    // Get the selected card from hand
                    let hand = self.cards.get_hand();
                    let card_to_duplicate = hand[card_index].clone();

                    // Add the specified number of copies to the discard pile
                    for _ in 0..*copies {
                        self.cards.add_card_to_discard(card_to_duplicate.clone());
                    }
                }

                // Return to player turn state
                self.battle_state = crate::battle::battle_state::BattleState::PlayerTurn;
            }
            BattleAction::SelectCardInExhaust(card_index) => {
                // Check if we're in the SelectCardInExhaust state
                if !matches!(self.battle_state, crate::battle::battle_state::BattleState::SelectCardInExhaust) {
                    return Err(BattleError::InvalidAction);
                }

                // Get the card from the exhaust pile and add it to hand
                let exhausted = self.cards.get_exhausted();
                if card_index >= exhausted.len() {
                    return Err(BattleError::InvalidAction);
                }

                // Remove the card from exhaust pile and add to hand
                let card = exhausted[card_index].clone();
                self.cards.remove_card_from_exhausted(card_index);
                self.cards.add_card_to_hand(card);

                // Return to player turn state
                self.battle_state = crate::battle::battle_state::BattleState::PlayerTurn;
            }
        }
        
        // Check if battle is over after the action
        let battle_events = self.take_battle_events();
        if !self.player.is_alive() {
            Ok(BattleResult::Lost(battle_events))
        } else if self.enemies.iter().all(|e| !e.battle_info.is_alive()) {
            Ok(BattleResult::Won(battle_events))
        } else {
            Ok(BattleResult::Continued(battle_events))
        }
    }

    /// List all available actions the player can take in the current battle state
    pub fn list_available_actions(&self) -> Vec<BattleAction> {
        let mut available_actions = Vec::new();
        
        // Battle is over - no actions available
        if self.is_battle_over() {
            return available_actions;
        }
        
        // Check each card in hand
        let hand = self.cards.get_hand();
        for (card_index, card) in hand.iter().enumerate() {
            // Check if card is playable, player has enough energy, and card is not an Attack while Entangled
            let is_attack_while_entangled = self.player.battle_info.is_entangled()
                && card.get_card_type() == &CardType::Attack;

            if card.is_playable() && self.player.get_energy() >= card.get_cost() && !is_attack_while_entangled {
                // Determine valid targets for this card based on its type and effects
                let valid_targets = self.get_valid_targets_for_card(card);

                // Add PlayCard action for each valid target
                for target in valid_targets {
                    available_actions.push(BattleAction::PlayCard(card_index, target));
                }
            }
        }

        // Add UsePotion actions for each filled potion slot
        let potions = self.get_potions();
        for (slot_index, potion) in potions.get_all_potions() {
            let (default_target, _effects) = potion.get_effects();

            // If potion has a default target, add action with None (will use default)
            if default_target.is_some() {
                available_actions.push(BattleAction::UsePotion(slot_index, None));
            } else {
                // Potion requires target selection - add action for each valid enemy
                for (enemy_index, enemy) in self.enemies.iter().enumerate() {
                    if enemy.battle_info.is_alive() {
                        available_actions.push(BattleAction::UsePotion(slot_index, Some(Entity::Enemy(enemy_index))));
                    }
                }
            }
        }

        // EndTurn is always available when battle is not over
        available_actions.push(BattleAction::EndTurn);

        available_actions
    }
    
    /// Get valid targets for a specific card based on its effects
    pub(in crate::battle) fn get_valid_targets_for_card(&self, card: &Card) -> Vec<Entity> {
        let mut valid_targets = Vec::new();
        
        // Check if any effect attacks all enemies (doesn't need specific targeting)
        let attacks_all_enemies = card.get_effects().iter().any(|effect| {
            matches!(effect, Effect::AttackAllEnemies { .. })
        });
        
        // Check if any effect targets specific enemies
        let targets_specific_enemies = card.get_effects().iter().any(|effect| {
            matches!(effect, 
                Effect::AttackToTarget { .. } |
                Effect::ApplyVulnerable { .. } |
                Effect::ApplyWeak { .. }
            )
        });
        
        // Check if any effect targets self/player  
        let targets_self = card.get_effects().iter().any(|effect| {
            matches!(effect,
                Effect::GainDefense { amount: _ } |
                Effect::GainStrength { amount: _ }
            )
        });
        
        // AttackAllEnemies cards use Entity::None as target (no specific targeting needed)
        if attacks_all_enemies {
            valid_targets.push(Entity::None);
        }
        
        // Add valid enemy targets for specific targeting
        if targets_specific_enemies {
            for (enemy_index, enemy) in self.enemies.iter().enumerate() {
                if enemy.battle_info.is_alive() {
                    valid_targets.push(Entity::Enemy(enemy_index));
                }
            }
        }
        
        // Add player target
        if targets_self {
            valid_targets.push(Entity::Player);
        }
        
        // If no specific targeting logic applies, default to allowing both enemy and player targets
        // This handles cards with mixed effects or unknown effect types
        if valid_targets.is_empty() {
            // Add all alive enemies as potential targets
            for (enemy_index, enemy) in self.enemies.iter().enumerate() {
                if enemy.battle_info.is_alive() {
                    valid_targets.push(Entity::Enemy(enemy_index));
                }
            }
            // Also add player as target
            valid_targets.push(Entity::Player);
        }
        
        valid_targets
    }
    
    /// Check if a target is valid for the current battle state
    pub(in crate::battle) fn is_valid_target(&self, target: &Entity) -> bool {
        match target {
            Entity::Enemy(idx) => *idx < self.enemies.len(),
            Entity::Player => true,  // Player is always a valid target
            Entity::None => true,    // None is valid for AttackAllEnemies cards
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::ironclad::starter_deck::starter_deck;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};

    #[test]
    fn test_play_card_with_target() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        let initial_energy = battle.player.get_energy();
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        
        // Find a Strike card in the hand
        let strike_idx = battle.cards.get_hand().iter().position(|card| card.get_name() == "Strike");
        
        if let Some(idx) = strike_idx {
            // Play the Strike card targeting enemy 0
            let action = BattleAction::PlayCard(idx, Entity::Enemy(0));
            let result = battle.eval_action(action, &mut rng);
            assert!(matches!(result, Ok(BattleResult::Continued(_))));
            
            // Check that energy was spent and enemy took damage
            assert!(battle.player.get_energy() < initial_energy);
            assert!(battle.enemies[0].battle_info.get_hp() < initial_enemy_hp);
        } else {
            panic!("No Strike card found in hand");
        }
    }

    #[test]
    fn test_list_available_actions_basic() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        let available_actions = battle.list_available_actions();
        
        // Should have actions for playable cards + EndTurn
        assert!(!available_actions.is_empty());
        
        // Should always have EndTurn available
        assert!(available_actions.contains(&BattleAction::EndTurn));
        
        // Check that we have PlayCard actions
        let play_card_actions: Vec<_> = available_actions.iter()
            .filter(|action| matches!(action, BattleAction::PlayCard(_, _)))
            .collect();
        
        assert!(!play_card_actions.is_empty(), "Should have at least some playable cards");
        
        // Verify all card actions are for cards with sufficient energy
        let hand = battle.get_hand();
        let player_energy = battle.get_player().get_energy();
        
        for action in &play_card_actions {
            if let BattleAction::PlayCard(card_idx, target) = action {
                assert!(*card_idx < hand.len(), "Card index should be valid");
                assert!(hand[*card_idx].get_cost() <= player_energy, "Should only suggest affordable cards");
                assert!(battle.is_valid_target(target), "Target should be valid");
            }
        }
        
        println!("Available actions: {}", available_actions.len());
        println!("Play card actions: {}", play_card_actions.len());
    }
    
    #[test]
    fn test_list_available_actions_no_energy() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Spend all energy
        battle.player.battle_info.spend_energy(battle.player.get_energy());
        assert_eq!(battle.player.get_energy(), 0);
        
        let available_actions = battle.list_available_actions();
        
        // Should only have EndTurn available (no energy for cards)
        assert_eq!(available_actions.len(), 1);
        assert_eq!(available_actions[0], BattleAction::EndTurn);
    }
    
    #[test]
    fn test_list_available_actions_battle_over() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Kill all enemies to end battle
        let enemy_hp = battle.enemies[0].battle_info.get_hp();
        battle.enemies[0].battle_info.take_damage(enemy_hp);
        
        assert!(battle.is_battle_over());
        
        let available_actions = battle.list_available_actions();
        
        // Should have no available actions when battle is over
        assert!(available_actions.is_empty());
    }
    
    #[test]
    fn test_get_valid_targets_for_card() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let hand = battle.get_hand();
        
        // Test targeting for different card types
        for card in hand {
            let targets = battle.get_valid_targets_for_card(card);
            assert!(!targets.is_empty(), "Every card should have at least one valid target");
            
            // Verify all returned targets are actually valid
            for target in &targets {
                assert!(battle.is_valid_target(target), "All returned targets should be valid");
            }
            
            println!("Card '{}' can target: {:?}", card.get_name(), targets);
        }
    }
    
    #[test]
    fn test_list_available_actions_with_dead_enemies() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create battle with two enemies
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2))
        ];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Kill the first enemy
        let first_enemy_hp = battle.enemies[0].battle_info.get_hp();
        battle.enemies[0].battle_info.take_damage(first_enemy_hp);
        assert!(!battle.enemies[0].battle_info.is_alive());
        assert!(battle.enemies[1].battle_info.is_alive());
        
        let available_actions = battle.list_available_actions();
        
        // Should still have actions available (second enemy is alive)
        assert!(!available_actions.is_empty());
        assert!(available_actions.contains(&BattleAction::EndTurn));
        
        // Check that PlayCard actions only target living enemies
        let play_card_actions: Vec<_> = available_actions.iter()
            .filter_map(|action| match action {
                BattleAction::PlayCard(idx, Entity::Enemy(enemy_idx)) => Some((*idx, *enemy_idx)),
                _ => None,
            })
            .collect();
        
        // All enemy-targeting actions should target the living enemy (index 1)
        for (_, enemy_idx) in play_card_actions {
            assert_eq!(enemy_idx, 1, "Should only target living enemy at index 1");
        }
    }
    
    #[test]
    fn test_list_available_actions_specific_cards() {
        use crate::cards::ironclad::{strike::strike, defend::defend, bash::bash};
        
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        
        // Create a deck with specific cards for testing
        let deck = Deck::new(vec![strike(), defend(), bash()]);
        let battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        let available_actions = battle.list_available_actions();
        
        // Strike should target enemies
        let strike_actions = available_actions.iter()
            .filter(|action| {
                if let BattleAction::PlayCard(0, target) = action {
                    matches!(target, Entity::Enemy(_))
                } else {
                    false
                }
            })
            .count();
        assert!(strike_actions > 0, "Strike should be able to target enemies");
        
        // Defend should target player 
        let defend_actions = available_actions.iter()
            .filter(|action| {
                if let BattleAction::PlayCard(1, target) = action {
                    matches!(target, Entity::Player)
                } else {
                    false
                }
            })
            .count();
        assert!(defend_actions > 0, "Defend should be able to target player");
        
        // Bash should target enemies (has attack + apply vulnerable effects)
        let bash_actions = available_actions.iter()
            .filter(|action| {
                if let BattleAction::PlayCard(2, target) = action {
                    matches!(target, Entity::Enemy(_))
                } else {
                    false
                }
            })
            .count();
        assert!(bash_actions > 0, "Bash should be able to target enemies");
        
        println!("Available actions for specific cards test: {}", available_actions.len());
    }

    #[test]
    fn test_burning_blood_relic_heals_at_end_of_combat() {
        use crate::cards::ironclad::strike::strike;
        use crate::game::{game::Game, game_event::GameEvent};
        use crate::map::{Map, MapNode, NodeType};
        use crate::relics::Relic;

        // Create a deck with high-damage cards to defeat enemy quickly
        let deck = Deck::new(vec![strike(), strike(), strike(), strike(), strike()]);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        // Create a simple test map
        let mut map = Map::new();
        let start_node = MapNode::new(0, 0, NodeType::Start);
        map.add_node(start_node);
        map.set_starting_position((0, 0)).unwrap();
        let start_node_position = (0, 0);

        // Create game with Burning Blood relic
        let mut game = Game::new(deck, global_info, map, 50, 80);
        if let Some(relic) = Relic::BurningBlood.to_game_event_listener() {
            game.add_game_event_listener(relic);
        }

        // Create a weak enemy that will die in one hit
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let mut enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        enemies[0].battle_info.current_hp = 1; // Reduce enemy HP for testing

        // Create battle and start it
        let battle_deck = crate::cards::ironclad::starter_deck::starter_deck();
        let mut battle = Battle::new(battle_deck, global_info, 50, 80, enemies, &mut rng);

        // Record initial HP
        let initial_hp = battle.player.battle_info.get_hp();

        // Defeat the enemy by dealing damage
        battle.enemies[0].battle_info.take_damage(100);

        // Battle should be won
        assert!(battle.is_battle_over());
        assert!(battle.player.is_alive());
        assert!(!battle.enemies[0].battle_info.is_alive());

        // Simulate what would happen in Game when battle is won
        game.player_hp = battle.get_final_player_hp();
        game.emit_game_event(GameEvent::CombatVictory);

        // Player should have healed 6 HP from Burning Blood
        let final_hp = game.player_hp;
        assert_eq!(final_hp, initial_hp + 6);
    }

    #[test]
    fn test_anchor_relic_starts_combat_with_10_block() {
        use crate::cards::ironclad::strike::strike;
        use crate::relics::Relic;
        

        // Create a deck for the battle
        let deck = Deck::new(vec![strike()]);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create an enemy for the battle
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Anchor relic listener
        let relics = vec![Relic::Anchor];
        let battle = Battle::new_with_relics(deck, global_info, 50, 80, enemies, relics, &mut rng);

        // Check that player now has 10 block from Anchor relic (CombatStart already emitted)
        assert_eq!(battle.get_player().battle_info.get_block(), 10);

        // Verify the anchor relic is now inactive (used once per combat)
        // Since we can't directly check listeners, we verify the effect was applied
        assert!(battle.player.battle_info.get_block() > 0);
    }

    #[test]
    fn test_blood_vial_relic_heals_2_hp_at_combat_start() {
        use crate::cards::ironclad::strike::strike;
        use crate::game::deck::Deck;
        use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::relics::Relic;

        let deck = Deck::new(vec![strike()]);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Blood Vial relic listener
        let relics = vec![Relic::BloodVial];
        let battle = Battle::new_with_relics(deck, global_info, 48, 80, enemies, relics, &mut rng);

        // Check that player now has 50 HP (48 + 2 from Blood Vial)
        assert_eq!(battle.get_player().battle_info.get_hp(), 50);

        // Verify the blood vial relic is now inactive (used once per combat)
        // Since we can't directly check listeners, we verify the effect was applied
        assert!(battle.player.battle_info.get_hp() > 48);
    }
}