use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

/// Sword Boomerang - Common Attack Card
/// Cost: 1
/// Effect: Deal 3 damage 3 times
pub fn sword_boomerang() -> Card {
    Card::new(CardEnum::SwordBoomerang, 1, CardType::Attack, vec![
        Effect::AttackToTarget {
            amount: 3,
            num_attacks: 3,
            strength_multiplier: 1,
        },
    ], false, true)
}

/// Sword Boomerang+ (Upgraded)
/// Cost: 1
/// Effect: Deal 4 damage 3 times
pub fn sword_boomerang_upgraded() -> Card {
    Card::new(CardEnum::SwordBoomerang, 1, CardType::Attack, vec![
        Effect::AttackToTarget {
            amount: 4,
            num_attacks: 3,
            strength_multiplier: 1,
        },
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sword_boomerang_creation() {
        let card = sword_boomerang();

        assert_eq!(card.get_name(), "Sword Boomerang");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::AttackToTarget {
            amount: 3,
            num_attacks: 3,
            strength_multiplier: 1,
        });
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_sword_boomerang_upgraded_creation() {
        let card = sword_boomerang_upgraded();

        assert_eq!(card.get_name(), "Sword Boomerang+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::AttackToTarget {
            amount: 4,
            num_attacks: 3,
            strength_multiplier: 1,
        });
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }
}