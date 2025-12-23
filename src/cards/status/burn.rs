use crate::game::{card::{Card, CardClass}, effect::BattleEffect, card_enum::CardEnum};
#[cfg(test)]
use crate::game::card_type::CardType;

pub fn burn() -> Card {
    Card::new(CardEnum::Burn, 0, CardClass::Status, vec![])
        .set_playable(false)
        .set_end_of_turn(vec![BattleEffect::LoseHp(2)]) // Deal 2 damage at end of turn
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burn_creation() {
        let card = burn();

        assert_eq!(card.get_name(), "Burn");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Status);
        assert_eq!(card.get_effects().len(), 0); // No direct effects
        assert!(!card.is_upgraded());
        assert!(!card.is_playable()); // Status cards are not playable

        // Check end of turn effects
        let end_of_turn_effects = card.get_end_of_turn().unwrap();
        assert_eq!(end_of_turn_effects.len(), 1);
        assert!(matches!(end_of_turn_effects[0], BattleEffect::LoseHp(2)));
    }

    #[test]
    fn test_burn_end_of_turn_effect() {
        let card = burn();
        let end_of_turn_effects = card.get_end_of_turn().unwrap();

        assert_eq!(end_of_turn_effects.len(), 1);
        assert!(matches!(end_of_turn_effects[0], BattleEffect::LoseHp(2)));
    }

    #[test]
    fn test_burn_upgraded_name() {
        let card = burn();
        assert_eq!(card.get_card_enum().upgraded_name(), "Burn+");
    }
}