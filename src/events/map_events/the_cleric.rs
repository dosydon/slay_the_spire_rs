use crate::game::effect::{Effect, BattleEffect, GameEffect};
use crate::events::map_events::{EventChoice, EventOutcome};

/// The Cleric event choices
/// Based on: https://slay-the-spire.fandom.com/wiki/The_Cleric
/// - Heal for gold (35 gold at floor 0, scales up with floor number)
/// - Leave
pub fn the_cleric_choices(floor: u32, player_hp: u32, player_max_hp: u32) -> Vec<EventChoice> {
    // Gold cost scales with floor: 35 + floor * 0.4 (rounded down)
    let gold_cost = 35 + (floor * 2 / 5);  // Integer division approximates floor * 0.4

    // Calculate actual heal amount for display
    let heal_amount = player_max_hp.saturating_sub(player_hp);

    vec![
        EventChoice {
            text: format!("Heal {} HP (costs {} gold)", heal_amount, gold_cost),
            outcome: EventOutcome::Effects(vec![
                Effect::Game(GameEffect::SpendGold { amount: gold_cost }),
                Effect::Battle(BattleEffect::Heal(0)),  // 0 = full heal
            ]),
        },
        EventChoice {
            text: "Leave".to_string(),
            outcome: EventOutcome::Effects(vec![]),
        },
    ]
}

pub fn the_cleric_description() -> &'static str {
    "A hooded cleric kneels before a small shrine. They turn to you with a gentle smile. \
     'Child, you look weary. I can help heal your wounds... for a small donation.'"
}
