use super::Battle;
use crate::battle::{action::Action, target::Entity, BattleResult, BattleError, events::BattleEvent};
use crate::game::{effect::{BaseEffect, Effect}, card::Card, card_type::CardType};

impl Battle {
    /// Evaluate a player action and return the battle result
    pub fn eval_action(&mut self, action: Action, rng: &mut impl rand::Rng) -> Result<BattleResult, BattleError> {
        if self.is_battle_over() {
            return Err(BattleError::GameAlreadyOver);
        }

        match action {
            Action::PlayCard(idx, target) => {
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
                
                self.play_card(idx, target);
            }
            Action::EndTurn => {
                let global_info = self.global_info;
                self.at_end_of_player_turn();
        
                self.at_start_of_enemy_turn();
                self.process_enemy_effects(rng, &global_info);
                self.at_end_of_enemy_turn();

                self.start_of_player_turn(rng);
            }
        }
        
        // Check if battle is over after the action
        if !self.player.is_alive() {
            Ok(BattleResult::Lost)
        } else if self.enemies.iter().all(|e| !e.battle_info.is_alive()) {
            Ok(BattleResult::Won)
        } else {
            Ok(BattleResult::Continued)
        }
    }

    /// Play a card from hand targeting a specific entity
    pub(in crate::battle) fn play_card(&mut self, idx: usize, target: Entity) {
        if idx >= self.cards.hand_size() { return; }
        
        let hand = self.cards.get_hand();
        let card = &hand[idx];
        if !self.player.spend_energy(card.get_cost()) { return; }
        
        let card_effects = card.get_effects().clone();
        let has_exhaust = card_effects.contains(&crate::game::effect::Effect::Exhaust);
        let is_skill_card = card.get_card_type() == &CardType::Skill;
        
        // Emit SkillCardPlayed event if this is a Skill card
        if is_skill_card {
            let skill_event = BattleEvent::SkillCardPlayed {
                source: Entity::Player,
            };
            self.emit_event(skill_event);
        }
        
        // Remove card from hand - exhaust it if it has Exhaust effect
        if has_exhaust {
            if let Some(_played_card) = self.cards.exhaust_card_from_hand(idx) {
                // Execute non-exhaust effects
                for effect in card_effects {
                    if effect != crate::game::effect::Effect::Exhaust {
                        self.eval_base_effect(&BaseEffect::from_effect(effect, Entity::Player, target));
                    }
                }
            }
        } else {
            if let Some(_played_card) = self.cards.play_card_from_hand(idx) {
                for effect in card_effects {
                    self.eval_base_effect(&BaseEffect::from_effect(effect, Entity::Player, target));
                }
            }
        }
    }
    
    /// List all available actions the player can take in the current battle state
    pub fn list_available_actions(&self) -> Vec<Action> {
        let mut available_actions = Vec::new();
        
        // Battle is over - no actions available
        if self.is_battle_over() {
            return available_actions;
        }
        
        // Check each card in hand
        let hand = self.cards.get_hand();
        for (card_index, card) in hand.iter().enumerate() {
            // Check if player has enough energy to play this card
            if self.player.get_energy() >= card.get_cost() {
                // Determine valid targets for this card based on its type and effects
                let valid_targets = self.get_valid_targets_for_card(card);
                
                // Add PlayCard action for each valid target
                for target in valid_targets {
                    available_actions.push(Action::PlayCard(card_index, target));
                }
            }
        }
        
        // EndTurn is always available when battle is not over
        available_actions.push(Action::EndTurn);
        
        available_actions
    }
    
    /// Get valid targets for a specific card based on its effects
    pub(in crate::battle) fn get_valid_targets_for_card(&self, card: &Card) -> Vec<Entity> {
        let mut valid_targets = Vec::new();
        
        // Check if any effect targets enemies
        let targets_enemies = card.get_effects().iter().any(|effect| {
            matches!(effect, 
                Effect::AttackToTarget { .. } |
                Effect::ApplyVulnerable { .. } |
                Effect::ApplyWeak { .. }
            )
        });
        
        // Check if any effect targets self/player  
        let targets_self = card.get_effects().iter().any(|effect| {
            matches!(effect,
                Effect::GainDefense(_) |
                Effect::GainStrength(_)
            )
        });
        
        // Add valid enemy targets
        if targets_enemies {
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
            Entity::None => false,   // None is not a valid target
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
            let action = Action::PlayCard(idx, Entity::Enemy(0));
            let result = battle.eval_action(action, &mut rng);
            assert!(matches!(result, Ok(BattleResult::Continued)));
            
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
        assert!(available_actions.contains(&Action::EndTurn));
        
        // Check that we have PlayCard actions
        let play_card_actions: Vec<_> = available_actions.iter()
            .filter(|action| matches!(action, Action::PlayCard(_, _)))
            .collect();
        
        assert!(!play_card_actions.is_empty(), "Should have at least some playable cards");
        
        // Verify all card actions are for cards with sufficient energy
        let hand = battle.get_hand();
        let player_energy = battle.get_player().get_energy();
        
        for action in &play_card_actions {
            if let Action::PlayCard(card_idx, target) = action {
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
        assert_eq!(available_actions[0], Action::EndTurn);
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
        assert!(available_actions.contains(&Action::EndTurn));
        
        // Check that PlayCard actions only target living enemies
        let play_card_actions: Vec<_> = available_actions.iter()
            .filter_map(|action| match action {
                Action::PlayCard(idx, Entity::Enemy(enemy_idx)) => Some((*idx, *enemy_idx)),
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
                if let Action::PlayCard(0, target) = action {
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
                if let Action::PlayCard(1, target) = action {
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
                if let Action::PlayCard(2, target) = action {
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
    fn test_exhaust_card_functionality() {
        use crate::cards::ironclad::{strike::strike, defend::defend};
        
        let mut deck_cards = vec![strike(), defend(), strike(), defend(), strike()];
        // Add a Slimed card to the deck
        deck_cards.push(crate::cards::status::slimed::slimed());
        let deck = Deck::new(deck_cards);
        
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Draw the hand
        battle.cards.draw_n(5);
        
        // Find the Slimed card in hand (if any) or add one
        let mut slimed_index = None;
        let hand = battle.cards.get_hand();
        for (i, card) in hand.iter().enumerate() {
            if card.get_name() == "Slimed" {
                slimed_index = Some(i);
                break;
            }
        }
        
        // If no Slimed card in hand, add one manually for testing
        if slimed_index.is_none() {
            battle.cards.add_card_to_hand(crate::cards::status::slimed::slimed());
            slimed_index = Some(battle.cards.hand_size() - 1);
        }
        
        let slimed_idx = slimed_index.unwrap();
        let initial_hand_size = battle.cards.hand_size();
        let initial_discard_size = battle.cards.discard_pile_size();
        let initial_exhausted_size = battle.cards.exhausted_size();
        let initial_energy = battle.player.get_energy();
        
        // Play the Slimed card
        battle.play_card(slimed_idx, Entity::Player);
        
        // Verify the effects:
        // 1. Card should be removed from hand
        assert_eq!(battle.cards.hand_size(), initial_hand_size - 1);
        // 2. Card should NOT go to discard pile (it's exhausted)
        assert_eq!(battle.cards.discard_pile_size(), initial_discard_size);
        // 3. Card should go to exhausted pile
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted_size + 1);
        // 4. Energy should be reduced by 1 (Slimed costs 1)
        assert_eq!(battle.player.get_energy(), initial_energy - 1);
        
        // Check that the exhausted card is Slimed
        let exhausted_cards = battle.cards.get_exhausted();
        assert_eq!(exhausted_cards.last().unwrap().get_name(), "Slimed");
    }
}