use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

/// Good Instincts - Colorless Skill Card
/// Cost: 0 (0 when upgraded)
/// Effect: Gain 6 Block
pub fn good_instincts() -> Card {
    Card::new(
        CardEnum::GoodInstincts,
        0,
        CardType::Skill,
        vec![Effect::GainDefense { amount: 6 }],
        false, // not upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

pub fn good_instincts_upgraded() -> Card {
    Card::new(
        CardEnum::GoodInstincts,
        0,
        CardType::Skill,
        vec![Effect::GainDefense { amount: 9 }],
        true,  // upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_instincts_creation() {
        let card = good_instincts();

        assert_eq!(card.get_name(), "Good Instincts");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_good_instincts_upgraded_creation() {
        let card = good_instincts_upgraded();

        assert_eq!(card.get_name(), "Good Instincts+");
        assert_eq!(card.get_cost(), 0); // Still costs 0
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_good_instincts_effects() {
        let card = good_instincts();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::GainDefense { amount } => {
                assert_eq!(*amount, 6);
            }
            _ => panic!("Expected GainDefense effect"),
        }
    }

    #[test]
    fn test_good_instincts_upgraded_effects() {
        let card = good_instincts_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::GainDefense { amount } => {
                assert_eq!(*amount, 9);
            }
            _ => panic!("Expected GainDefense effect"),
        }
    }
}