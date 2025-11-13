use super::Battle;
use crate::battle::{action::Action, target::Entity, BattleResult, BattleError};
use crate::game::{effect::{BaseEffect, Effect}, card::Card};

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