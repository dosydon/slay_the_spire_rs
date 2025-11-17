use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

pub fn bash() -> Card {
    Card::new_with_condition(CardEnum::Bash, 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 8, num_attacks: 1, strength_multiplier: 1 },
        Effect::ApplyVulnerable { duration: 2 }
    ], false, Condition::True)
}

pub fn bash_upgraded() -> Card {
    Card::new_with_condition(CardEnum::Bash, 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 10, num_attacks: 1, strength_multiplier: 1 }, // +2 damage
        Effect::ApplyVulnerable { duration: 3 } // +1 vulnerable duration
    ], true, Condition::True)
}