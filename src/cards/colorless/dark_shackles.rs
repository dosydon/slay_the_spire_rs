use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::{Rarity, CardClass}};

/// Dark Shackles - Colorless Skill Card (Uncommon)
/// Cost: 0 (0 when upgraded)
/// Effect: Enemy loses 9 Strength this turn. Exhaust
pub fn dark_shackles() -> Card {
    Card::new(
        CardEnum::DarkShackles,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Skill),
        vec![
            Effect::LoseStrengthTarget(9),
            Effect::LoseStrengthAtEndOfTurn(9),
            Effect::Exhaust,
        ]
    )
        .set_playable(true)
}

pub fn dark_shackles_upgraded() -> Card {
    Card::new(
        CardEnum::DarkShackles,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Skill),
        vec![
            Effect::LoseStrengthTarget(15),
            Effect::LoseStrengthAtEndOfTurn(15),
            Effect::Exhaust,
        ]
    )
        .set_upgraded(true)
        .set_playable(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_shackles_creation() {
        let card = dark_shackles();

        assert_eq!(card.get_name(), "Dark Shackles");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 3);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_dark_shackles_upgraded_creation() {
        let card = dark_shackles_upgraded();

        assert_eq!(card.get_name(), "Dark Shackles+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 3);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_dark_shackles_effects() {
        let card = dark_shackles();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 3);

        // First effect should be LoseStrengthTarget(9)
        match &effects[0] {
            Effect::LoseStrengthTarget(amount) => {
                assert_eq!(*amount, 9);
            }
            _ => panic!("Expected LoseStrengthTarget effect as first effect"),
        }

        // Second effect should be LoseStrengthAtEndOfTurn(9)
        match &effects[1] {
            Effect::LoseStrengthAtEndOfTurn(amount) => {
                assert_eq!(*amount, 9);
            }
            _ => panic!("Expected LoseStrengthAtEndOfTurn effect as second effect"),
        }

        // Third effect should be Exhaust
        assert_eq!(effects[2], Effect::Exhaust);
    }

    #[test]
    fn test_dark_shackles_upgraded_effects() {
        let card = dark_shackles_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 3);

        // First effect should be LoseStrengthTarget(15) - upgraded
        match &effects[0] {
            Effect::LoseStrengthTarget(amount) => {
                assert_eq!(*amount, 15);
            }
            _ => panic!("Expected LoseStrengthTarget effect as first effect"),
        }

        // Second effect should be LoseStrengthAtEndOfTurn(15) - upgraded
        match &effects[1] {
            Effect::LoseStrengthAtEndOfTurn(amount) => {
                assert_eq!(*amount, 15);
            }
            _ => panic!("Expected LoseStrengthAtEndOfTurn effect as second effect"),
        }

        // Third effect should be Exhaust (same as base)
        assert_eq!(effects[2], Effect::Exhaust);
    }
}
