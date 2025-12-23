use crate::game::effect::{Effect, BattleEffect, GameEffect};
use crate::events::map_events::{EventChoice, EventOutcome};
use crate::game::card_enum::CardEnum;

/// World of Goop event choices
/// Based on: https://slay-the-spire.fandom.com/wiki/World_of_Goop
/// - Take the gold: Gain 75 gold, add 5 Slimed cards to deck
/// - Leave: No effect
pub fn world_of_goop_choices() -> Vec<EventChoice> {
    vec![
        EventChoice {
            text: "Take the gold (75 gold, gain 5 Slimed cards)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::Game(GameEffect::GainGold { amount: 75 }),
                Effect::Battle(BattleEffect::AddCardToDrawPile(CardEnum::Slimed)),
                Effect::Battle(BattleEffect::AddCardToDrawPile(CardEnum::Slimed)),
                Effect::Battle(BattleEffect::AddCardToDrawPile(CardEnum::Slimed)),
                Effect::Battle(BattleEffect::AddCardToDrawPile(CardEnum::Slimed)),
                Effect::Battle(BattleEffect::AddCardToDrawPile(CardEnum::Slimed)),
            ]),
        },
        EventChoice {
            text: "Leave".to_string(),
            outcome: EventOutcome::Effects(vec![]),
        },
    ]
}

pub fn world_of_goop_description() -> &'static str {
    "You step into a strange, slimy chamber. Everything is coated in translucent goop. \
     You notice a pile of gold coins stuck in the slime..."
}
