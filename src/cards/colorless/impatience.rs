use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::{Effect, Condition}, card::{Rarity, CardClass}};

/// Impatience - Colorless Skill Card (Uncommon)
/// Cost: 0 (0 when upgraded)
/// Effect: If you have no Attack cards in hand, draw 2 cards
pub fn impatience() -> Card {
    Card::new(
        CardEnum::Impatience,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Skill),
        vec![
            Effect::ConditionalEffect(
                Condition::HandNoAttacks,
                Box::new(Effect::DrawCard { count: 2 }),
            ),
        ]
    )
        .set_playable(true)
}

pub fn impatience_upgraded() -> Card {
    Card::new(
        CardEnum::Impatience,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Skill),
        vec![
            Effect::ConditionalEffect(
                Condition::HandNoAttacks,
                Box::new(Effect::DrawCard { count: 3 }),
            ),
        ]
    )
        .set_upgraded(true)
        .set_playable(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impatience_creation() {
        let card = impatience();

        assert_eq!(card.get_name(), "Impatience");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_impatience_upgraded_creation() {
        let card = impatience_upgraded();

        assert_eq!(card.get_name(), "Impatience+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_impatience_effects() {
        let card = impatience();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);

        // Should be a ConditionalEffect
        match &effects[0] {
            Effect::ConditionalEffect(condition, effect) => {
                assert_eq!(*condition, Condition::HandNoAttacks);
                match **effect {
                    Effect::DrawCard { count } => {
                        assert_eq!(count, 2);
                    }
                    _ => panic!("Expected DrawCard effect inside ConditionalEffect"),
                }
            }
            _ => panic!("Expected ConditionalEffect"),
        }
    }

    #[test]
    fn test_impatience_upgraded_effects() {
        let card = impatience_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);

        // Should be a ConditionalEffect
        match &effects[0] {
            Effect::ConditionalEffect(condition, effect) => {
                assert_eq!(*condition, Condition::HandNoAttacks);
                match **effect {
                    Effect::DrawCard { count } => {
                        assert_eq!(count, 3);
                    }
                    _ => panic!("Expected DrawCard effect inside ConditionalEffect"),
                }
            }
            _ => panic!("Expected ConditionalEffect"),
        }
    }
}
