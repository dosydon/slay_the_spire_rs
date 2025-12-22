use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, card::Rarity};

/// Ascender's Curse - A curse card that deals damage to the player at the end of combat
/// This card is typically gained in higher ascension levels
/// Ethereal: Exhausts at end of turn if not played
pub fn ascenders_curse() -> Card {
    Card::new(CardEnum::AscendersCurse, 1, CardType::Curse, vec![], Rarity::Curse)
        .set_ethereal(true)
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
        assert_eq!(card.get_card_type(), &CardType::Curse);
        assert_eq!(card.get_rarity(), Rarity::Curse);
        assert!(!card.is_upgraded());
        assert!(!card.is_playable()); // Curse cards are typically unplayable
        assert!(card.is_ethereal()); // Ascender's Curse is ethereal

        // Curse cards usually have no direct effects
        let effects = card.get_effects();
        assert_eq!(effects.len(), 0);
    }
}