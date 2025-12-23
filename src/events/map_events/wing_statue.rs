use crate::game::effect::Effect;
use crate::events::map_events::{EventChoice, EventOutcome};

/// Wing Statue event choices
/// Based on: https://slay-the-spire.fandom.com/wiki/Wing_Statue
/// - Place the wing: Obtain a random relic
/// - Destroy: Lose 7 Max HP (gain nothing)
/// - Leave: No effect
pub fn wing_statue_choices() -> Vec<EventChoice> {
    vec![
        EventChoice {
            text: "Place the wing (Obtain a random relic)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::ObtainRandomRelic,
            ]),
        },
        EventChoice {
            text: "Destroy the statue (Lose 7 Max HP)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::LoseHp(7),
            ]),
        },
        EventChoice {
            text: "Leave (No effect)".to_string(),
            outcome: EventOutcome::Effects(vec![]),
        },
    ]
}

pub fn wing_statue_description() -> &'static str {
    "An imposing statue looms before you, but it's missing one of its wings. \
     You notice a wing-shaped slot and a matching stone wing nearby..."
}
