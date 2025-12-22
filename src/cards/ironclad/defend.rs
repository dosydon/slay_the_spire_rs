use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum, card::{Rarity, CardClass}};

pub fn defend() -> Card {
    Card::new(CardEnum::Defend, 1, CardClass::IronClad(Rarity::Basic, CardType::Skill), vec![Effect::GainDefense { amount: 5 }])
        .set_play_condition(Condition::True)
}

pub fn defend_upgraded() -> Card {
    Card::new(CardEnum::Defend, 1, CardClass::IronClad(Rarity::Basic, CardType::Skill), vec![Effect::GainDefense { amount: 8 }])
        .set_play_condition(Condition::True)
        .set_upgraded(true)
}