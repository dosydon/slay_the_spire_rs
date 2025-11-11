use crate::game::{card::Card, card_type::CardType, effect::Effect};

pub fn defend () -> Card {
    Card::new("Defend".to_string(), 1, CardType::Skill, vec![Effect::GainDefense(5)])
}