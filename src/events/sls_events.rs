use crate::events::{encounter_events::EncounterEvent, map_events::MapEvent};
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SLSEvent {
    EncounterEvent(EncounterEvent),
    MapEvent(MapEvent),
    // Other event types can be added here
}