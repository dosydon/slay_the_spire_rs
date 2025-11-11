use crate::game::{card::Card, card_type::CardType, effect::Effect};

pub fn bash() -> Card {
    Card::new("Bash".to_string(), 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 8, num_attacks: 1 },
        Effect::Vulnerable(2)     ])
}