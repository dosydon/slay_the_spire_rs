use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum, card::Rarity};

pub fn bash() -> Card {
    Card::new(CardEnum::Bash, 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 8, num_attacks: 1, strength_multiplier: 1 },
        Effect::ApplyVulnerable { duration: 2 }
    ], Rarity::Basic)
        .set_play_condition(Condition::True)
}

pub fn bash_upgraded() -> Card {
    Card::new(CardEnum::Bash, 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 10, num_attacks: 1, strength_multiplier: 1 }, // +2 damage
        Effect::ApplyVulnerable { duration: 3 } // +1 vulnerable duration
    ], Rarity::Basic)
        .set_play_condition(Condition::True)
        .set_upgraded(true)
}