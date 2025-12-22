use crate::game::{card::{Card, CardClass}, card_enum::CardEnum, effect::Condition};
#[cfg(test)]
use crate::game::card_type::CardType;

/// Dazed - Status Card
/// Cost: -
/// Effect: Cannot be played. Ethereal (exhausts at end of turn)
pub fn dazed() -> Card {
    Card::new(
        CardEnum::Dazed,
        1, // Cost 1 but unplayable due to condition
        CardClass::Status,
        vec![]
    )
        .set_play_condition(Condition::False)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dazed_creation() {
        let card = dazed();
        assert_eq!(card.get_name(), "Dazed");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Status);
        assert!(!card.is_upgraded());
        assert!(!card.is_playable()); // Should not be playable
    }

    #[test]
    fn test_dazed_effects() {
        let card = dazed();
        let effects = card.get_effects();
        assert_eq!(effects.len(), 0); // Dazed has no effects
    }

    #[test]
    fn test_dazed_condition() {
        let card = dazed();
        let play_condition = card.get_play_condition();
        assert!(matches!(play_condition, Condition::False)); // Cannot be played
    }

    #[test]
    fn test_dazed_card_enum() {
        let card = dazed();
        let card_enum = card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::Dazed));
    }
}