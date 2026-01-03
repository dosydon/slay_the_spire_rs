use serde::{Deserialize, Serialize};

use crate::{
    effects::BattleEffect,
    relics::{
        burning_blood::BurningBloodRelic,
        mango::MangoRelic,
        pear::PearRelic,
        strawberry::StrawberryRelic,
    },
};

use super::game_event::{GameEvent, GameEventListener};

/// Enum wrapper for all types that implement GameEventListener
/// This allows us to avoid Box<dyn GameEventListener> for better performance and serializability
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum GameEventListenerEnum {
    BurningBlood(BurningBloodRelic),
    Strawberry(StrawberryRelic),
    Pear(PearRelic),
    Mango(MangoRelic),
}

impl GameEventListener for GameEventListenerEnum {
    fn on_game_event(&mut self, event: &GameEvent) -> Vec<BattleEffect> {
        match self {
            GameEventListenerEnum::BurningBlood(listener) => listener.on_game_event(event),
            GameEventListenerEnum::Strawberry(listener) => listener.on_game_event(event),
            GameEventListenerEnum::Pear(listener) => listener.on_game_event(event),
            GameEventListenerEnum::Mango(listener) => listener.on_game_event(event),
        }
    }

    fn is_active(&self) -> bool {
        match self {
            GameEventListenerEnum::BurningBlood(listener) => listener.is_active(),
            GameEventListenerEnum::Strawberry(listener) => listener.is_active(),
            GameEventListenerEnum::Pear(listener) => listener.is_active(),
            GameEventListenerEnum::Mango(listener) => listener.is_active(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_event_listener_enum_is_clonable() {
        let listener = GameEventListenerEnum::BurningBlood(BurningBloodRelic::new());
        let _cloned = listener.clone();
    }

    #[test]
    fn test_game_event_listener_enum_is_hashable() {
        use std::collections::HashSet;
        let listener = GameEventListenerEnum::BurningBlood(BurningBloodRelic::new());
        let mut set = HashSet::new();
        set.insert(listener);
    }

    #[test]
    fn test_game_event_listener_enum_serialization() {
        let listener = GameEventListenerEnum::BurningBlood(BurningBloodRelic::new());
        let serialized = serde_json::to_string(&listener).expect("Should serialize");
        let _deserialized: GameEventListenerEnum =
            serde_json::from_str(&serialized).expect("Should deserialize");
    }
}
