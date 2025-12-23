use crate::game::effect::{Effect, BattleEffect, GameEffect};
use crate::events::map_events::{EventChoice, EventOutcome};

/// Golden Idol event choices
/// Based on: https://slay-the-spire.fandom.com/wiki/Golden_Idol
/// - Take: 25-75% chance of damage, obtain Golden Idol relic
/// - Smash: 25 damage, obtain Bloody Idol relic (alternate version)
/// - Leave: No effect
pub fn golden_idol_choices() -> Vec<EventChoice> {
    vec![
        EventChoice {
            text: "Take the idol carefully (Obtain a relic, chance of damage)".to_string(),
            outcome: EventOutcome::Effects(vec![
                // TODO: 25-75% chance of taking damage based on dexterity
                // For now, we'll just give the relic
                Effect::Game(GameEffect::ObtainRandomRelic),  // Should specifically be Golden Idol
            ]),
        },
        EventChoice {
            text: "Destroy the trap (Take 25 damage, obtain a relic)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::Battle(BattleEffect::LoseHp(25)),
                Effect::Game(GameEffect::ObtainRandomRelic),  // Should specifically be Bloody Idol
            ]),
        },
        EventChoice {
            text: "Leave (No effect)".to_string(),
            outcome: EventOutcome::Effects(vec![]),
        },
    ]
}

pub fn golden_idol_description() -> &'static str {
    "A gleaming golden idol sits atop a pedestal, surrounded by suspicious tiles. \
     This is clearly a trap, but the idol looks valuable..."
}
