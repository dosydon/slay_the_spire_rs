use crate::game::effect::{Effect, BattleEffect, GameEffect};
use crate::events::map_events::{EventChoice, EventOutcome};

/// Dead Adventurer event choices
/// Based on: https://slay-the-spire.fandom.com/wiki/Dead_Adventurer
/// - Fight: Combat encounter (awakened Lagavulin)
/// - Flee: Leave safely
pub fn dead_adventurer_choices() -> Vec<EventChoice> {
    vec![
        EventChoice {
            text: "Fight the creature (Enter combat)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::Game(GameEffect::TriggerCombatEvent),  // This should trigger a Lagavulin fight
            ]),
        },
        EventChoice {
            text: "Flee (Leave safely)".to_string(),
            outcome: EventOutcome::Effects(vec![]),
        },
    ]
}

pub fn dead_adventurer_description() -> &'static str {
    "You find the corpse of an adventurer, their belongings scattered around them. \
     Strange sounds echo from deeper in the cave..."
}
