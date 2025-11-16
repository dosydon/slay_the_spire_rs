use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

/// Anger - Common Attack Card
/// Cost: 0
/// Effect: Deal 6 damage. Add a copy of this card to your discard pile.
pub fn anger() -> Card {
    Card::new(CardEnum::Anger, 0, CardType::Attack, vec![
        Effect::AttackToTarget {
            amount: 6,
            num_attacks: 1,
            strength_multiplier: 1,
        },
        Effect::AddCardToDiscard(CardEnum::Anger),
    ], false, true)
}

/// Anger+ (Upgraded)
/// Cost: 0
/// Effect: Deal 8 damage. Add a copy of this card to your discard pile.
pub fn anger_upgraded() -> Card {
    Card::new(CardEnum::Anger, 0, CardType::Attack, vec![
        Effect::AttackToTarget {
            amount: 8,
            num_attacks: 1,
            strength_multiplier: 1,
        },
        Effect::AddCardToDiscard(CardEnum::Anger),
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anger_creation() {
        let card = anger();

        assert_eq!(card.get_name(), "Anger");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 2);
        assert_eq!(card.get_effects()[0], Effect::AttackToTarget {
            amount: 6,
            num_attacks: 1,
            strength_multiplier: 1,
        });
        assert_eq!(card.get_effects()[1], Effect::AddCardToDiscard(CardEnum::Anger));
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_anger_upgraded_creation() {
        let card = anger_upgraded();

        assert_eq!(card.get_name(), "Anger+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 2);
        assert_eq!(card.get_effects()[0], Effect::AttackToTarget {
            amount: 8,
            num_attacks: 1,
            strength_multiplier: 1,
        });
        assert_eq!(card.get_effects()[1], Effect::AddCardToDiscard(CardEnum::Anger));
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }
}