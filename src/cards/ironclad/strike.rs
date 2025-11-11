use crate::game::{card::Card, card_type::CardType, effect::Effect};

pub fn strike() -> Card {
    Card::new("Strike".to_string(), 1, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 6, num_attacks: 1 }
    ])
}