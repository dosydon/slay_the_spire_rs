use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

pub fn seeing_red() -> Card {
    Card::new(CardEnum::SeeingRed, 1, CardType::Skill, vec![
        Effect::GainEnergy(2),
        Effect::Exhaust
    ], false, true)
}

pub fn seeing_red_upgraded() -> Card {
    Card::new(CardEnum::SeeingRed, 0, CardType::Skill, vec![
        Effect::GainEnergy(2),
        Effect::Exhaust
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeing_red_basic() {
        let card = seeing_red();
        assert_eq!(card.get_name(), "Seeing Red");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(!card.is_upgraded());

        let effects = card.get_effects();
        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&Effect::GainEnergy(2)));
        assert!(effects.contains(&Effect::Exhaust));
    }

    #[test]
    fn test_seeing_red_upgraded() {
        let card = seeing_red_upgraded();
        assert_eq!(card.get_name(), "Seeing Red+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(card.is_upgraded());

        let effects = card.get_effects();
        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&Effect::GainEnergy(2)));
        assert!(effects.contains(&Effect::Exhaust));
    }
}
