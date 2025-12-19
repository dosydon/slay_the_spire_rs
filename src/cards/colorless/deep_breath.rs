use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

/// Deep Breath - Colorless Skill Card
/// Cost: 0 (0 when upgraded)
/// Effect: Shuffle discard pile into draw pile. Draw 1 card (2 cards when upgraded)
pub fn deep_breath() -> Card {
    Card::new(
        CardEnum::DeepBreath,
        0,
        CardType::Skill,
        vec![
            Effect::ShuffleDiscardIntoDraw,
            Effect::DrawCard { count: 1 },
        ],
        false, // not upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

pub fn deep_breath_upgraded() -> Card {
    Card::new(
        CardEnum::DeepBreath,
        0,
        CardType::Skill,
        vec![
            Effect::ShuffleDiscardIntoDraw,
            Effect::DrawCard { count: 2 },
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
    fn test_deep_breath_creation() {
        let card = deep_breath();

        assert_eq!(card.get_name(), "Deep Breath");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_deep_breath_upgraded_creation() {
        let card = deep_breath_upgraded();

        assert_eq!(card.get_name(), "Deep Breath+");
        assert_eq!(card.get_cost(), 0); // Still costs 0
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_deep_breath_effects() {
        let card = deep_breath();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be ShuffleDiscardIntoDraw
        match &effects[0] {
            Effect::ShuffleDiscardIntoDraw => {
                // No additional validation needed for this effect type
            }
            _ => panic!("Expected ShuffleDiscardIntoDraw effect as first effect"),
        }

        // Second effect should be DrawCard(1)
        match &effects[1] {
            Effect::DrawCard { count } => {
                assert_eq!(*count, 1);
            }
            _ => panic!("Expected DrawCard effect as second effect"),
        }
    }

    #[test]
    fn test_deep_breath_upgraded_effects() {
        let card = deep_breath_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be ShuffleDiscardIntoDraw (same as base)
        match &effects[0] {
            Effect::ShuffleDiscardIntoDraw => {
                // No additional validation needed for this effect type
            }
            _ => panic!("Expected ShuffleDiscardIntoDraw effect as first effect"),
        }

        // Second effect should be DrawCard(2) - upgraded
        match &effects[1] {
            Effect::DrawCard { count } => {
                assert_eq!(*count, 2);
            }
            _ => panic!("Expected DrawCard effect as second effect"),
        }
    }
}