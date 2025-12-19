use crate::game::{card::{Card, Rarity}, card_type::CardType, card_enum::CardEnum, effect::Effect};

pub fn slimed() -> Card {
    Card::new(CardEnum::Slimed, 1, CardType::Status, vec![
        Effect::Exhaust
    ], false, true, Rarity::Basic)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slimed_basic() {
        let card = slimed();
        assert_eq!(card.get_name(), "Slimed");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Status);
        assert!(!card.is_upgraded());
        
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::Exhaust => {
                // Test passes
            }
            _ => panic!("Expected Exhaust effect"),
        }
    }

    #[test]
    fn test_slimed_properties() {
        let card = slimed();
        
        // Slimed is a status card that costs 1 and exhausts
        assert_eq!(card.get_name(), "Slimed");
        assert_eq!(card.cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Status);
        assert_eq!(card.get_effects(), &vec![Effect::Exhaust]);
    }

    #[test]
    fn test_slimed_upgrade_does_nothing() {
        let card = slimed();
        let upgraded = card.upgrade();
        
        // Status cards don't upgrade, should return the same card
        assert_eq!(upgraded.get_name(), "Slimed");
        assert_eq!(upgraded.get_cost(), 1);
        assert_eq!(upgraded.get_card_type(), &CardType::Status);
        assert!(!upgraded.is_upgraded());
    }
}