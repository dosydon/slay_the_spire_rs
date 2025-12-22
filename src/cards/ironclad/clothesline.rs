use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum, card::{Rarity, CardClass}};

pub fn clothesline() -> Card {
    Card::new(CardEnum::Clothesline, 2, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        Effect::AttackToTarget { amount: 12, num_attacks: 1, strength_multiplier: 1 },
        Effect::ApplyWeak { duration: 2 }
    ])
        .set_play_condition(Condition::True)
}

pub fn clothesline_upgraded() -> Card {
    Card::new(CardEnum::Clothesline, 2, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        Effect::AttackToTarget { amount: 14, num_attacks: 1, strength_multiplier: 1 }, // +2 damage
        Effect::ApplyWeak { duration: 3 } // +1 weak duration
    ])
        .set_play_condition(Condition::True)
        .set_upgraded(true)
}