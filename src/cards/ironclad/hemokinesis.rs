use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

/// Hemokinesis - Uncommon Attack Card
/// Cost: 1
/// Effect: Lose 2 HP. Deal 15 damage
pub fn hemokinesis() -> Card {
    Card::new(CardEnum::Hemokinesis, 1, CardType::Attack, vec![
        Effect::LoseHp(2),
        Effect::AttackToTarget {
            amount: 15,
            num_attacks: 1,
            strength_multiplier: 1,
        },
    ], false, true)
}

/// Hemokinesis+ (Upgraded)
/// Cost: 1
/// Effect: Lose 2 HP. Deal 22 damage
pub fn hemokinesis_upgraded() -> Card {
    Card::new(CardEnum::Hemokinesis, 1, CardType::Attack, vec![
        Effect::LoseHp(2),
        Effect::AttackToTarget {
            amount: 22,
            num_attacks: 1,
            strength_multiplier: 1,
        },
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hemokinesis_creation() {
        let card = hemokinesis();

        assert_eq!(card.get_name(), "Hemokinesis");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 2);
        assert_eq!(card.get_effects()[0], Effect::LoseHp(2));
        assert_eq!(card.get_effects()[1], Effect::AttackToTarget {
            amount: 15,
            num_attacks: 1,
            strength_multiplier: 1,
        });
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_hemokinesis_upgraded_creation() {
        let card = hemokinesis_upgraded();

        assert_eq!(card.get_name(), "Hemokinesis+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 2);
        assert_eq!(card.get_effects()[0], Effect::LoseHp(2));
        assert_eq!(card.get_effects()[1], Effect::AttackToTarget {
            amount: 22,
            num_attacks: 1,
            strength_multiplier: 1,
        });
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_hemokinesis_effect_order() {
        let card = hemokinesis();
        let effects = card.get_effects();

        // Verify that HP loss comes before damage
        assert_eq!(effects[0], Effect::LoseHp(2));
        assert_eq!(effects[1], Effect::AttackToTarget {
            amount: 15,
            num_attacks: 1,
            strength_multiplier: 1,
        });
    }
}