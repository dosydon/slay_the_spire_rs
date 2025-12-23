use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

pub fn wild_strike() -> Card {
    Card::new(CardEnum::WildStrike, 1, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        BattleEffect::AttackToTarget { amount: 12, num_attacks: 1, strength_multiplier: 1 },
        BattleEffect::AddCardToDrawPile(CardEnum::Wound)
    ])
}

pub fn wild_strike_upgraded() -> Card {
    Card::new(CardEnum::WildStrike, 1, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        BattleEffect::AttackToTarget { amount: 17, num_attacks: 1, strength_multiplier: 1 }, // +5 damage
        BattleEffect::AddCardToDrawPile(CardEnum::Wound)
    ])
        .set_upgraded(true)
}