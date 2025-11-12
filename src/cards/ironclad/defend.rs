use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

pub fn defend() -> Card {
    Card::new(CardEnum::Defend, 1, CardType::Skill, vec![Effect::GainDefense(5)], false)
}

pub fn defend_upgraded() -> Card {
    Card::new(CardEnum::Defend, 1, CardType::Skill, vec![Effect::GainDefense(8)], true)
}