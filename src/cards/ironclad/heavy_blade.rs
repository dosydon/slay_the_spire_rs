use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::{Rarity, CardClass}};

pub fn heavy_blade() -> Card {
    Card::new(CardEnum::HeavyBlade, 2, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        Effect::AttackToTarget { amount: 14, num_attacks: 1, strength_multiplier: 3 }
    ])
}

pub fn heavy_blade_upgraded() -> Card {
    Card::new(CardEnum::HeavyBlade, 2, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        Effect::AttackToTarget { amount: 22, num_attacks: 1, strength_multiplier: 3 } // +8 damage
    ])
        .set_upgraded(true)
}