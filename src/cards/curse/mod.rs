use crate::game::{card::Card, card_enum::CardEnum, card::CardClass, effect::Effect};

#[cfg(test)]
use crate::game::card_type::CardType;

/// Ascender's Curse - A curse card that deals damage to the player at the end of combat
/// This card is typically gained in higher ascension levels
/// Ethereal: Exhausts at end of turn if not played
pub fn ascenders_curse() -> Card {
    Card::new(CardEnum::AscendersCurse, 1, CardClass::Curse, vec![])
        .set_ethereal(true)
        .set_playable(false)
}

/// Injury - A curse card that has no effect and is unplayable
/// Cost: 0, Effect: None (unplayable)
pub fn injury() -> Card {
    Card::new(CardEnum::Injury, 0, CardClass::Curse, vec![])
        .set_playable(false)
}

/// Clumsy - A curse card that has no effect and is ethereal
/// Cost: 0, Effect: None (unplayable), Ethereal: exhausts at end of turn if not played
pub fn clumsy() -> Card {
    Card::new(CardEnum::Clumsy, 0, CardClass::Curse, vec![])
        .set_playable(false)
        .set_ethereal(true)
}

/// Regret - A curse card that damages the player at end of turn for each card in hand
/// Cost: 0, Ethereal: exhausts at end of turn if not played
/// Effect: Deals damage to player equal to number of cards in hand at end of turn
pub fn regret() -> Card {
    Card::new(CardEnum::Regret, 0, CardClass::Curse, vec![])
        .set_playable(false)
        .set_ethereal(true)
        .set_end_of_turn(vec![Effect::LoseHpPerCardInHand { damage_per_card: 1 }])
}

/// Writhe - A curse card that is innate (starts in every hand)
/// Cost: 0, Effect: Unplayable. Innate.
/// Properties: Can be removed from deck, Innate (always starts in hand)
pub fn writhe() -> Card {
    Card::new(CardEnum::Writhe, 0, CardClass::Curse, vec![])
        .set_playable(false)
        .set_innate(true)
}

/// Note: Ascender's Curse doesn't have an upgraded version in the original game
/// The curse effects are typically handled by the game system rather than card effects
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascenders_curse_creation() {
        let card = ascenders_curse();
        assert_eq!(card.get_name(), "Ascender's Curse");
        assert_eq!(card.get_cost(), 1); // Unplayable cost
        assert_eq!(card.get_card_type(), CardType::Curse);
        assert!(!card.is_upgraded());
        assert!(!card.is_playable()); // Curse cards are typically unplayable
        assert!(card.is_ethereal()); // Ascender's Curse is ethereal

        // Curse cards usually have no direct effects
        let effects = card.get_effects();
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_injury_creation() {
        let card = injury();
        assert_eq!(card.get_name(), "Injury");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Curse);
        assert!(!card.is_upgraded());
        assert!(!card.is_playable()); // Injury is unplayable
        assert!(!card.is_ethereal()); // Injury is not ethereal

        // Injury should have no effects
        let effects = card.get_effects();
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_clumsy_creation() {
        let card = clumsy();
        assert_eq!(card.get_name(), "Clumsy");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Curse);
        assert!(!card.is_upgraded());
        assert!(!card.is_playable()); // Clumsy is unplayable
        assert!(card.is_ethereal()); // Clumsy is ethereal

        // Clumsy should have no effects
        let effects = card.get_effects();
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_regret_creation() {
        let card = regret();
        assert_eq!(card.get_name(), "Regret");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Curse);
        assert!(!card.is_upgraded());
        assert!(!card.is_playable()); // Regret is unplayable
        assert!(card.is_ethereal()); // Regret is ethereal

        // Regret's effect (damage for cards in hand) is now handled by end_of_turn effects
        let effects = card.get_effects();
        assert_eq!(effects.len(), 0);

        // Check end of turn effects
        let end_of_turn_effects = card.get_end_of_turn().unwrap();
        assert_eq!(end_of_turn_effects.len(), 1);
        assert!(matches!(end_of_turn_effects[0], Effect::LoseHpPerCardInHand { damage_per_card: 1 }));
    }

    #[test]
    fn test_writhe_creation() {
        let card = writhe();
        assert_eq!(card.get_name(), "Writhe");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Curse);
        assert!(!card.is_upgraded());
        assert!(!card.is_playable()); // Writhe is unplayable
        assert!(card.is_innate()); // Writhe is innate
        assert!(card.is_removable()); // Writhe can be removed from deck

        // Writhe should have no effects
        let effects = card.get_effects();
        assert_eq!(effects.len(), 0);
    }
}