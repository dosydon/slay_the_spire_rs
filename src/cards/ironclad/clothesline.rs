use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

pub fn clothesline() -> Card {
    Card::new_with_condition(CardEnum::Clothesline, 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 12, num_attacks: 1, strength_multiplier: 1 },
        Effect::ApplyWeak { duration: 2 }
    ], false, Condition::True)
}

pub fn clothesline_upgraded() -> Card {
    Card::new_with_condition(CardEnum::Clothesline, 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 14, num_attacks: 1, strength_multiplier: 1 }, // +2 damage
        Effect::ApplyWeak { duration: 3 } // +1 weak duration
    ], true, Condition::True)
}