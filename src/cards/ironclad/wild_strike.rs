use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::{Rarity, CardClass}};

pub fn wild_strike() -> Card {
    Card::new(CardEnum::WildStrike, 1, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        Effect::AttackToTarget { amount: 12, num_attacks: 1, strength_multiplier: 1 },
        Effect::AddCardToDrawPile(CardEnum::Wound)
    ])
}

pub fn wild_strike_upgraded() -> Card {
    Card::new(CardEnum::WildStrike, 1, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        Effect::AttackToTarget { amount: 17, num_attacks: 1, strength_multiplier: 1 }, // +5 damage
        Effect::AddCardToDrawPile(CardEnum::Wound)
    ])
        .set_upgraded(true)
}