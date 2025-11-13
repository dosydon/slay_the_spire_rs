use super::Battle;
use crate::game::{effect::{BaseEffect, Effect}, global_info::GlobalInfo};
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
    
    /// Ends the player turn
    pub(in crate::battle) fn at_end_of_player_turn(&mut self) {
        self.player.battle_info.at_end_of_turn();
        
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
        for enemy in &mut self.enemies {
            if enemy.battle_info.is_alive() {
                // Apply enemy's end-of-turn effects
                enemy.battle_info.at_end_of_turn();
            }
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