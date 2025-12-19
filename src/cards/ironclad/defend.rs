use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum, card::Rarity};

pub fn defend() -> Card {
    Card::new_with_condition(CardEnum::Defend, 1, CardType::Skill, vec![Effect::GainDefense { amount: 5 }], false, Condition::True, Rarity::Basic)
}

pub fn defend_upgraded() -> Card {
    Card::new_with_condition(CardEnum::Defend, 1, CardType::Skill, vec![Effect::GainDefense { amount: 8 }], true, Condition::True, Rarity::Basic)
}