use super::Battle;
use crate::battle::{target::Entity, battle_events::{BattleEvent, EventListener}, event_listener_enum::EventListenerEnum};
use crate::game::effect::BaseEffect;

impl Battle {
    /// Add an event listener to the battle
    pub fn add_listener(&mut self, listener: EventListenerEnum) {
        self.event_listeners.push(listener);
    }

    
    /// Emit an event to all active listeners and collect their effects
    pub(crate) fn emit_event(&mut self, event: BattleEvent) {
        // Store the event for GUI to read
        self.battle_events.push(event.clone());

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

        // Queue all new effects instead of processing immediately
        for effect in new_effects {
            self.queue_effect(effect);
        }

        // Process the effect queue immediately
        self.process_effect_queue();
    }

    /// Get and clear all battle events that occurred since last call
    pub fn take_battle_events(&mut self) -> Vec<BattleEvent> {
        std::mem::take(&mut self.battle_events)
    }
}