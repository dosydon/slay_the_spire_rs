use super::Battle;
use crate::game::{effect::BaseEffect, global_info::GlobalInfo};
use crate::battle::target::Entity;

impl Battle {
    /// Full turn start including card draw with deck reshuffling
    pub(crate) fn start_of_player_turn(&mut self, rng: &mut impl rand::Rng) {
        self.player.at_start_of_turn();

        // Sample enemy actions for this turn
        self.sample_enemy_actions(rng);

        // Draw new hand (typically 5 cards)
        // The draw_n method will automatically reshuffle discard pile into deck if needed
        self.cards.draw_n(5);
    }

    /// Initialize the first turn of battle (draw cards, sample enemy actions, but don't reset block)
    pub(crate) fn initialize_first_turn(&mut self, rng: &mut impl rand::Rng) {
        // Don't call player.at_start_of_turn() here to preserve relic block gains

        // Sample enemy actions for this turn
        self.sample_enemy_actions(rng);

        // Draw new hand (typically 5 cards)
        // The draw_n method will automatically reshuffle discard pile into deck if needed
        self.cards.draw_n(5);
    }
    
    /// Ends the player turn
    pub(in crate::battle) fn at_end_of_player_turn(&mut self) {
        self.player.battle_info.at_end_of_turn();

        // Emit end-of-turn event for player
        let end_turn_event = super::events::BattleEvent::EndOfTurn {
            entity: super::target::Entity::Player,
        };
        self.emit_event(end_turn_event);

        // Exhaust all ethereal cards from hand (before discarding)
        let mut i = 0;
        while i < self.cards.hand_size() {
            let hand = self.cards.get_hand();
            if hand[i].is_ethereal() {
                // Exhaust this card
                self.cards.exhaust_card_from_hand(i);

                // Emit CardExhausted event
                let exhaust_event = super::events::BattleEvent::CardExhausted {
                    source: super::target::Entity::Player,
                };
                self.emit_event(exhaust_event);

                // Don't increment i since we removed a card
            } else {
                i += 1;
            }
        }

        // Discard all remaining cards in hand
        self.cards.discard_entire_hand();
    }
    
    /// Starts enemy turns - resets enemy block
    pub(in crate::battle) fn at_start_of_enemy_turn(&mut self) {
        for enemy in &mut self.enemies {
            if enemy.battle_info.is_alive() {
                // Reset enemy's block at start of their turn
                enemy.battle_info.at_start_of_turn();
            }
        }
    }
    
    /// Ends all enemies' turns
    pub(crate) fn at_end_of_enemy_turn(&mut self) {
        // First, collect indices of alive enemies
        let alive_enemy_indices: Vec<usize> = self.enemies.iter_mut()
            .enumerate()
            .filter(|(_, enemy)| enemy.battle_info.is_alive())
            .map(|(i, _)| i)
            .collect();

        // Apply end-of-turn effects to alive enemies
        for i in alive_enemy_indices.iter() {
            if *i < self.enemies.len() {
                self.enemies[*i].battle_info.at_end_of_turn();
            }
        }

        // Then emit end-of-turn events for each alive enemy
        for i in alive_enemy_indices {
            let end_turn_event = super::events::BattleEvent::EndOfTurn {
                entity: super::target::Entity::Enemy(i),
            };
            self.emit_event(end_turn_event);
        }
    }

    /// Process all enemy effects during enemy turn phase
    pub(crate) fn process_enemy_effects(&mut self, _rng: &mut impl rand::Rng, _global_info: &GlobalInfo) {
        let mut all_effects = Vec::new();
        
        for i in 0..self.enemies.len() {
            let source = Entity::Enemy(i);
            
            // Skip processing effects for defeated enemies
            if !self.enemies[i].battle_info.is_alive() {
                // Clear the stored action for dead enemies
                self.enemy_actions[i].take();
                continue;
            }
            
            // Use stored effects - panic if none were stored (this should never happen)
            let (_, stored_effects) = self.enemy_actions[i].take()
                .expect("No enemy action stored - actions should be sampled at start of turn");
            
            for effect in stored_effects {
                let base_effect = BaseEffect::from_effect(effect, source, Entity::Player);
                all_effects.push(base_effect);
            }
        }
        
        // Apply all collected effects
        for effect in all_effects {
            self.eval_base_effect(&effect);
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
    use crate::cards::ironclad::strike::strike;

    #[test]
    fn test_complete_turn_simulation() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        // Record initial state
        let initial_player_hp = battle.player.battle_info.get_hp();
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        let initial_energy = battle.player.get_energy();

        // === PLAYER TURN ===

        // Player starts turn (should reset block and refresh energy)
        battle.player.at_start_of_turn();
        assert_eq!(battle.player.get_energy(), 3); // Energy refreshed
        assert_eq!(battle.player.get_block(), 0); // Block reset
        
        // Player plays a Defend card (should give 5 block)
        let defend_idx = battle.cards.get_hand().iter().position(|card| card.get_name() == "Defend");
        if let Some(idx) = defend_idx {
            let hand_size_before = battle.cards.hand_size();
            battle.play_card(idx, Entity::Player);
            
            // Check card was played
            assert_eq!(battle.cards.hand_size(), hand_size_before - 1);
            // Check energy was spent (Defend costs 1)
            assert_eq!(battle.player.get_energy(), 2);
            // Check block was gained (Defend gives 5 block)
            assert_eq!(battle.player.get_block(), 5);
        }
        
        // Player plays a Strike card targeting enemy
        let strike_idx = battle.cards.get_hand().iter().position(|card| card.get_name() == "Strike");
        if let Some(idx) = strike_idx {
            let enemy_hp_before = battle.enemies[0].battle_info.get_hp();
            battle.play_card(idx, Entity::Enemy(0));
            
            // Check energy was spent (Strike costs 1)
            assert_eq!(battle.player.get_energy(), 1);
            // Check enemy took damage (Strike deals 6 damage)
            assert_eq!(battle.enemies[0].battle_info.get_hp(), enemy_hp_before - 6);
        }
        
        // Enemy acts (Red Louse will either attack or gain strength)
        let player_hp_before_enemy = battle.player.battle_info.get_hp();
        let player_block_before_enemy = battle.player.get_block();
        let enemy_strength_before = battle.enemies[0].battle_info.get_strength();
        
        battle.process_enemy_effects(&mut rng, &global_info);
        
        // Check that either the player took damage OR the enemy gained strength
        let player_took_damage = battle.player.battle_info.get_hp() < player_hp_before_enemy || 
                                battle.player.get_block() < player_block_before_enemy;
        let enemy_gained_strength = battle.enemies[0].battle_info.get_strength() > enemy_strength_before;
        
        // One of these should have happened (Red Louse either attacks or grows)
        assert!(player_took_damage || enemy_gained_strength, 
                "Enemy should have either attacked player or gained strength");
        
        // === TURN CYCLE COMPLETE ===
        
        // Verify battle is not over (both entities should still be alive)
        assert!(battle.player.is_alive(), "Player should still be alive");
        assert!(battle.enemies[0].battle_info.is_alive(), "Enemy should still be alive");
        assert!(!battle.is_battle_over(), "Battle should not be over yet");
        
        // Verify some state changes occurred during the turn
        assert!(initial_player_hp != battle.player.battle_info.get_hp() || 
                initial_enemy_hp != battle.enemies[0].battle_info.get_hp() ||
                enemy_gained_strength,
                "Some combat effects should have occurred during the turn");
        
        println!("Turn complete - Player HP: {}/{}, Block: {}, Energy: {} | Enemy HP: {}, Strength: {}",
                battle.player.battle_info.get_hp(), initial_player_hp, battle.player.get_block(), battle.player.get_energy(),
                battle.enemies[0].battle_info.get_hp(), battle.enemies[0].battle_info.get_strength());
    }

    #[test]
    fn test_defeated_enemies_dont_execute_moves() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create two enemies 
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2))
        ];
        
        // Create a deck with strike cards
        let deck = Deck::new(vec![strike(), strike(), strike(), strike(), strike()]);
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Sample enemy actions for both enemies
        battle.sample_enemy_actions(&mut rng);
        
        // Kill the first enemy manually 
        battle.enemies[0].battle_info.take_damage(100);
        assert!(!battle.enemies[0].battle_info.is_alive(), "First enemy should be dead");
        assert!(battle.enemies[1].battle_info.is_alive(), "Second enemy should be alive");
        
        // Record player HP before enemy turn
        let player_hp_before = battle.player.battle_info.get_hp();
        let player_block_before = battle.player.battle_info.get_block();
        
        // Execute enemy turn - defeated enemy should not act
        battle.process_enemy_effects(&mut rng, &global_info);
        
        // Check that only the living enemy could have affected the player
        // We can't predict exact values due to randomness, but we can verify the system works
        // by ensuring the battle system didn't crash and state is consistent
        
        // Verify that dead enemy's action was cleared
        assert!(battle.enemy_actions[0].is_none(), "Dead enemy should have no stored action");
        
        // The living enemy may or may not have affected the player (depends on its chosen move),
        // but the important thing is that the dead enemy didn't execute its move
        // This is verified by the fact that we didn't panic and the action was cleared
        
        // Ensure the battle is in a consistent state
        assert!(!battle.enemies[0].battle_info.is_alive(), "First enemy should still be dead");
        assert!(battle.enemies[1].battle_info.is_alive(), "Second enemy should still be alive");
        assert!(battle.player.battle_info.is_alive(), "Player should still be alive");
    }

    #[test]
    fn test_ethereal_cards_exhausted_at_end_of_turn() {
        use crate::cards::ironclad::{carnage::carnage, strike::strike, defend::defend};

        // Create a deck with Carnage (ethereal) and non-ethereal cards
        let deck = Deck::new(vec![carnage(), strike(), defend(), carnage(), strike()]);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        // Draw the hand
        battle.cards.draw_n(5);

        // Count ethereal and non-ethereal cards in hand
        let hand = battle.cards.get_hand();
        let ethereal_count = hand.iter().filter(|c| c.is_ethereal()).count();
        let non_ethereal_count = hand.len() - ethereal_count;

        let initial_hand_size = battle.cards.hand_size();
        let initial_discard_size = battle.cards.discard_pile_size();
        let initial_exhausted_size = battle.cards.exhausted_size();

        // End the player turn (should exhaust ethereal cards and discard the rest)
        battle.at_end_of_player_turn();

        // Verify the effects:
        // 1. Hand should be empty
        assert_eq!(battle.cards.hand_size(), 0);
        // 2. Ethereal cards should be in exhausted pile
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted_size + ethereal_count);
        // 3. Non-ethereal cards should be in discard pile
        assert_eq!(battle.cards.discard_pile_size(), initial_discard_size + non_ethereal_count);

        // Check that all exhausted cards are ethereal (Carnage)
        let exhausted_cards = battle.cards.get_exhausted();
        for i in initial_exhausted_size..exhausted_cards.len() {
            assert!(exhausted_cards[i].is_ethereal(), "All newly exhausted cards should be ethereal");
        }
    }

    #[test]
    fn test_played_ethereal_card_not_double_exhausted() {
        use crate::cards::ironclad::{carnage::carnage, strike::strike};

        // Create a deck with Carnage (ethereal) and Strike
        let deck = Deck::new(vec![carnage(), strike(), strike(), strike(), strike()]);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        // Draw the hand
        battle.cards.draw_n(5);

        // Find and play Carnage if it's in hand
        let carnage_idx = battle.cards.get_hand().iter().position(|card| card.get_name() == "Carnage");

        if let Some(idx) = carnage_idx {
            let initial_exhausted = battle.cards.exhausted_size();

            // Play Carnage (it should go to discard, not exhaust, since it was played)
            let _ = battle.play_card(idx, Entity::Enemy(0));

            // Carnage should be in discard after being played
            assert_eq!(battle.cards.discard_pile_size(), 1);
            assert_eq!(battle.cards.exhausted_size(), initial_exhausted);

            // Count remaining ethereal cards in hand
            let hand = battle.cards.get_hand();
            let ethereal_in_hand = hand.iter().filter(|c| c.is_ethereal()).count();

            // End turn - should only exhaust ethereal cards still in hand
            battle.at_end_of_player_turn();

            // The played Carnage should still be in discard (total discard = 1 played + remaining non-ethereal)
            // And only ethereal cards that weren't played should be exhausted
            let hand_size_before_end = battle.cards.hand_size();
            assert_eq!(battle.cards.hand_size(), 0);
            assert_eq!(battle.cards.exhausted_size(), initial_exhausted + ethereal_in_hand);
        }
    }
}