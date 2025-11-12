use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

pub fn bash() -> Card {
    Card::new(CardEnum::Bash, 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 8, num_attacks: 1 },
        Effect::ApplyVulnerable(2)
    ], false)
}

pub fn bash_upgraded() -> Card {
    Card::new(CardEnum::Bash, 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 10, num_attacks: 1 }, // +2 damage
        Effect::ApplyVulnerable(3) // +1 vulnerable duration
    ], true)
}