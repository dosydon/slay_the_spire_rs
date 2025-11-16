use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

/// Bludgeon - Rare Attack Card
/// Cost: 3
/// Effect: Deal 32 damage
pub fn bludgeon() -> Card {
    Card::new(CardEnum::Bludgeon, 3, CardType::Attack, vec![
        Effect::AttackToTarget {
            amount: 32,
            num_attacks: 1,
            strength_multiplier: 1,
        },
    ], false, true)
}

/// Bludgeon+ (Upgraded)
/// Cost: 2
/// Effect: Deal 42 damage
pub fn bludgeon_upgraded() -> Card {
    Card::new(CardEnum::Bludgeon, 2, CardType::Attack, vec![
        Effect::AttackToTarget {
            amount: 42,
            num_attacks: 1,
            strength_multiplier: 1,
        },
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bludgeon_creation() {
        let card = bludgeon();

        assert_eq!(card.get_name(), "Bludgeon");
        assert_eq!(card.get_cost(), 3);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::AttackToTarget {
            amount: 32,
            num_attacks: 1,
            strength_multiplier: 1,
        });
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_bludgeon_upgraded_creation() {
        let card = bludgeon_upgraded();

        assert_eq!(card.get_name(), "Bludgeon+");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::AttackToTarget {
            amount: 42,
            num_attacks: 1,
            strength_multiplier: 1,
        });
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }
}