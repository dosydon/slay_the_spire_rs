use super::Battle;
use crate::battle::{target::Entity, events::{BattleEvent, EventListener}};
use crate::game::effect::{BaseEffect, Effect};

impl Battle {
    /// Add an event listener to the battle
    pub fn add_listener(&mut self, listener: Box<dyn EventListener>) {
        self.event_listeners.push(listener);
    }

    
    /// Emit an event to all active listeners and collect their effects
    pub(crate) fn emit_event(&mut self, event: BattleEvent) {
        let mut new_effects = Vec::new();

        // Process all active listeners
        for listener in &mut self.event_listeners {
            if listener.is_active() {
                let effects = listener.on_event(&event);
                for effect in effects {
                    // Convert effects to base effects and evaluate them
                    let base_effect = BaseEffect::from_effect(effect, listener.get_owner(), Entity::Player);
                    new_effects.push(base_effect);
                }
            }
        }

        // Remove inactive listeners
        self.event_listeners.retain(|listener| listener.is_active());

        // Process all new effects
        for effect in new_effects {
            self.eval_base_effect(&effect);
        }
    }
}