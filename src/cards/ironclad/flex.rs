use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

pub fn flex() -> Card {
    Card::new(CardEnum::Flex, 0, CardType::Skill, vec![
        Effect::GainStrength(2),        // Gain 2 Strength immediately
        Effect::LoseStrengthAtEndOfTurn(2), // Lose 2 Strength at end of turn
    ], false)
}

pub fn flex_upgraded() -> Card {
    Card::new(CardEnum::Flex, 0, CardType::Skill, vec![
        Effect::GainStrength(3),        // Gain 3 Strength immediately (+1)
        Effect::LoseStrengthAtEndOfTurn(3), // Lose 3 Strength at end of turn
    ], true)
}