use crate::events::{encounter_events::EncounterEvent, map_events::MapEvent};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
pub enum SLSEvent {
    EncounterEvent(EncounterEvent),
    MapEvent(MapEvent),
    // Other event types can be added here
}