use crate::game::{card::Card, effect::{BattleEffect, Condition}, card_type::CardType, card_enum::CardEnum, card::{Rarity, CardClass}};

pub fn bash() -> Card {
    Card::new(CardEnum::Bash, 2, CardClass::IronClad(Rarity::Basic, CardType::Attack), vec![
        BattleEffect::AttackToTarget { amount: 8, num_attacks: 1, strength_multiplier: 1 },
        BattleEffect::ApplyVulnerable { duration: 2 }
    ])
        .set_play_condition(Condition::True)
}

pub fn bash_upgraded() -> Card {
    Card::new(CardEnum::Bash, 2, CardClass::IronClad(Rarity::Basic, CardType::Attack), vec![
        BattleEffect::AttackToTarget { amount: 10, num_attacks: 1, strength_multiplier: 1 }, // +2 damage
        BattleEffect::ApplyVulnerable { duration: 3 } // +1 vulnerable duration
    ])
        .set_play_condition(Condition::True)
        .set_upgraded(true)
}