use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

/// Blind - Colorless Skill Card
/// Cost: 0 (0 when upgraded)
/// Effect: Apply 2 Weak to ALL enemies. Exhaust
pub fn blind() -> Card {
    Card::new(
        CardEnum::Blind,
        0,
        CardType::Skill,
        vec![
            Effect::ApplyWeakAll { duration: 2 },
            Effect::Exhaust,
        ],
        false, // not upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

pub fn blind_upgraded() -> Card {
    Card::new(
        CardEnum::Blind,
        0,
        CardType::Skill,
        vec![
            Effect::ApplyWeakAll { duration: 2 },
            Effect::Exhaust,
        ],
        true,  // upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blind_creation() {
        let card = blind();

        assert_eq!(card.get_name(), "Blind");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_blind_upgraded_creation() {
        let card = blind_upgraded();

        assert_eq!(card.get_name(), "Blind+");
        assert_eq!(card.get_cost(), 0); // Still costs 0
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_blind_effects() {
        let card = blind();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be ApplyWeakAll
        match &effects[0] {
            Effect::ApplyWeakAll { duration } => {
                assert_eq!(*duration, 2);
            }
            _ => panic!("Expected ApplyWeakAll effect as first effect"),
        }

        // Second effect should be Exhaust
        assert_eq!(effects[1], Effect::Exhaust);
    }

    #[test]
    fn test_blind_upgraded_effects() {
        let card = blind_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be ApplyWeakAll (same as base)
        match &effects[0] {
            Effect::ApplyWeakAll { duration } => {
                assert_eq!(*duration, 2);
            }
            _ => panic!("Expected ApplyWeakAll effect as first effect"),
        }

        // Second effect should be Exhaust (same as base)
        assert_eq!(effects[1], Effect::Exhaust);
    }
}