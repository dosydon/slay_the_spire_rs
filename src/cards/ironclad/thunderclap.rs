use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

pub fn thunderclap() -> Card {
    Card::new(CardEnum::Thunderclap, 1, CardType::Attack, vec![
        Effect::AttackAllEnemies { amount: 4, num_attacks: 1 },
        Effect::ApplyVulnerable(1)
    ], false, true)
}

pub fn thunderclap_upgraded() -> Card {
    Card::new(CardEnum::Thunderclap, 1, CardType::Attack, vec![
        Effect::AttackAllEnemies { amount: 6, num_attacks: 1 }, // +2 damage
        Effect::ApplyVulnerable(2) // +1 vulnerable duration
    ], true, true)
}