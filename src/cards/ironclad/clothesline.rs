use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

pub fn clothesline() -> Card {
    Card::new(CardEnum::Clothesline, 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 12, num_attacks: 1, strength_multiplier: 1 },
        Effect::ApplyWeak(2)
    ], false, true)
}

pub fn clothesline_upgraded() -> Card {
    Card::new(CardEnum::Clothesline, 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 14, num_attacks: 1, strength_multiplier: 1 }, // +2 damage
        Effect::ApplyWeak(3) // +1 weak duration
    ], true, true)
}