use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

/// Swift Strike - Colorless Attack Card
/// Cost: 0 (0 when upgraded)
/// Effect: Deal 7 damage
pub fn swift_strike() -> Card {
    Card::new(
        CardEnum::SwiftStrike,
        0,
        CardType::Attack,
        vec![Effect::AttackToTarget { amount: 7, num_attacks: 1, strength_multiplier: 0 }],
        false, // not upgraded
        true,  // playable
    )
}

pub fn swift_strike_upgraded() -> Card {
    Card::new(
        CardEnum::SwiftStrike,
        0,
        CardType::Attack,
        vec![Effect::AttackToTarget { amount: 10, num_attacks: 1, strength_multiplier: 0 }],
        true,  // upgraded
        true,  // playable
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swift_strike_creation() {
        let card = swift_strike();

        assert_eq!(card.get_name(), "Swift Strike");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_swift_strike_upgraded_creation() {
        let card = swift_strike_upgraded();

        assert_eq!(card.get_name(), "Swift Strike+");
        assert_eq!(card.get_cost(), 0); // Still costs 0
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_swift_strike_effects() {
        let card = swift_strike();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 7);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 0);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_swift_strike_upgraded_effects() {
        let card = swift_strike_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 10);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 0);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }
}