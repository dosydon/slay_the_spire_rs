use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum, card::Rarity};

pub fn defend() -> Card {
    Card::new(CardEnum::Defend, 1, CardType::Skill, vec![Effect::GainDefense { amount: 5 }], Rarity::Basic)
        .set_play_condition(Condition::True)
}

pub fn defend_upgraded() -> Card {
    Card::new(CardEnum::Defend, 1, CardType::Skill, vec![Effect::GainDefense { amount: 8 }], Rarity::Basic)
        .set_play_condition(Condition::True)
        .set_upgraded(true)
}