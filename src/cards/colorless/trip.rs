use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

/// Trip - Colorless Skill Card
/// Cost: 0 (0 when upgraded)
/// Effect: Apply 2 Vulnerable to ALL enemies. Exhaust
pub fn trip() -> Card {
    Card::new(
        CardEnum::Trip,
        0,
        CardType::Skill,
        vec![
            Effect::ApplyVulnerableAll { duration: 2 },
            Effect::Exhaust,
        ],
        false, // not upgraded
        true,  // playable
    )
}

pub fn trip_upgraded() -> Card {
    Card::new(
        CardEnum::Trip,
        0,
        CardType::Skill,
        vec![
            Effect::ApplyVulnerableAll { duration: 2 },
            Effect::Exhaust,
        ],
        true,  // upgraded
        true,  // playable
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trip_creation() {
        let card = trip();

        assert_eq!(card.get_name(), "Trip");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_trip_upgraded_creation() {
        let card = trip_upgraded();

        assert_eq!(card.get_name(), "Trip+");
        assert_eq!(card.get_cost(), 0); // Still costs 0
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_trip_effects() {
        let card = trip();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be ApplyVulnerableAll
        match &effects[0] {
            Effect::ApplyVulnerableAll { duration } => {
                assert_eq!(*duration, 2);
            }
            _ => panic!("Expected ApplyVulnerableAll effect as first effect"),
        }

        // Second effect should be Exhaust
        assert_eq!(effects[1], Effect::Exhaust);
    }

    #[test]
    fn test_trip_upgraded_effects() {
        let card = trip_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be ApplyVulnerableAll (same as base)
        match &effects[0] {
            Effect::ApplyVulnerableAll { duration } => {
                assert_eq!(*duration, 2);
            }
            _ => panic!("Expected ApplyVulnerableAll effect as first effect"),
        }

        // Second effect should be Exhaust (same as base)
        assert_eq!(effects[1], Effect::Exhaust);
    }
}