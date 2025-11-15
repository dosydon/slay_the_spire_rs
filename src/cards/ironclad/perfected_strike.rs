use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

pub fn perfected_strike() -> Card {
    Card::new(CardEnum::PerfectedStrike, 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }
        // TODO: Add 2 additional damage per Strike card in deck
        // This requires dynamic effect calculation based on deck composition
    ], false, true)
}

pub fn perfected_strike_upgraded() -> Card {
    Card::new(CardEnum::PerfectedStrike, 2, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 10, num_attacks: 1, strength_multiplier: 1 }
        // TODO: Add 3 additional damage per Strike card in deck (upgraded)
        // This requires dynamic effect calculation based on deck composition
    ], true, true)
}