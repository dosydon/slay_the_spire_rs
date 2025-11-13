use super::Battle;
use crate::enemies::enemy_enum::{EnemyEnum, EnemyMove};
use crate::game::{effect::Effect, global_info::GlobalInfo};
use crate::battle::{target::Entity, listeners::CurlUpListener, events::{BattleEvent, EventListener}};
use crate::game::effect::BaseEffect;

impl Battle {
    /// Initialize event listeners for enemies based on their type
    pub(in crate::battle) fn initialize_enemy_listeners(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) {
        for (i, enemy) in self.enemies.iter().enumerate() {
            match &enemy.enemy {
                EnemyEnum::RedLouse(_) => {
                    // Red Louse gets a curl up listener with randomly generated block amount
                    let curl_up = CurlUpListener::new(Entity::Enemy(i), global_info.ascention, rng);
                    self.event_listeners.push(Box::new(curl_up));
                }
                EnemyEnum::GreenLouse(_) => {
                    // Green Louse also gets a curl up listener with randomly generated block amount
                    let curl_up = CurlUpListener::new(Entity::Enemy(i), global_info.ascention, rng);
                    self.event_listeners.push(Box::new(curl_up));
                }
                EnemyEnum::JawWorm(_) => {
                    // Jaw Worm has no special listeners
                }
                EnemyEnum::Cultist(_) => {
                    // Cultist has no special listeners
                }
                EnemyEnum::SpikeSlimeS(_) => {
                    // Spike Slime (S) has no special listeners
                }
                EnemyEnum::SpikeSlimeM(_) => {
                    // Spike Slime (M) has no special listeners
                }
                EnemyEnum::AcidSlimeS(_) => {
                    // Acid Slime (S) has no special listeners
                }
                EnemyEnum::AcidSlimeM(_) => {
                    // Acid Slime (M) has no special listeners
                }
            }
        }
    }
    
    /// Emit a battle event to all listeners
    pub(in crate::battle) fn emit_event(&mut self, event: BattleEvent) {
        let mut effects_to_apply = Vec::new();
        
        for listener in &mut self.event_listeners {
            if listener.is_active() {
                let triggered_effects = listener.on_event(&event);
                for effect in triggered_effects {
                    let base_effect = BaseEffect::from_effect(effect, listener.get_owner(), listener.get_owner());
                    effects_to_apply.push(base_effect);
                }
            }
        }
        
        // Apply all triggered effects
        for effect in effects_to_apply {
            self.eval_base_effect(&effect);
        }
    }

    /// Sample and store the next action and effects for all enemies
    pub(crate) fn sample_enemy_actions(&mut self, rng: &mut impl rand::Rng) {
        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            let (enemy_move, effects) = enemy.enemy.sample_move_and_effects(&self.global_info, rng);
            self.enemy_actions[i] = Some((enemy_move, effects));
        }
    }
    
    /// Get the stored move for a specific enemy
    pub fn get_enemy_move(&self, enemy_index: usize) -> Option<&EnemyMove> {
        self.enemy_actions.get(enemy_index).and_then(|pair| pair.as_ref().map(|(enemy_move, _)| enemy_move))
    }
    
    /// Get the stored move and effects for a specific enemy
    pub fn get_enemy_move_and_effects(&self, enemy_index: usize) -> Option<(&EnemyMove, &Vec<Effect>)> {
        self.enemy_actions.get(enemy_index).and_then(|pair| pair.as_ref().map(|(enemy_move, effects)| (enemy_move, effects)))
    }
    
    /// Get all stored enemy moves
    pub(in crate::battle) fn get_all_enemy_moves(&self) -> Vec<Option<&EnemyMove>> {
        self.enemy_actions.iter().map(|pair| pair.as_ref().map(|(enemy_move, _)| enemy_move)).collect()
    }
}