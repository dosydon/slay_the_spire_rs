use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

/// Flash of Steel - Colorless Attack Card
/// Cost: 0 (0 when upgraded)
/// Effect: Deal 3 damage. Draw 1 card
pub fn flash_of_steel() -> Card {
    Card::new(
        CardEnum::FlashOfSteel,
        0,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: 3, num_attacks: 1, strength_multiplier: 0 },
            Effect::DrawCard { count: 1 },
        ],
        false, // not upgraded
        true,  // playable
    )
}

pub fn flash_of_steel_upgraded() -> Card {
    Card::new(
        CardEnum::FlashOfSteel,
        0,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 0 },
            Effect::DrawCard { count: 1 },
        ],
        true,  // upgraded
        true,  // playable
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flash_of_steel_creation() {
        let card = flash_of_steel();

        assert_eq!(card.get_name(), "Flash of Steel");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_flash_of_steel_upgraded_creation() {
        let card = flash_of_steel_upgraded();

        assert_eq!(card.get_name(), "Flash of Steel+");
        assert_eq!(card.get_cost(), 0); // Still costs 0
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_flash_of_steel_effects() {
        let card = flash_of_steel();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be AttackToTarget(3)
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 3);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 0);
            }
            _ => panic!("Expected AttackToTarget effect as first effect"),
        }

        // Second effect should be DrawCard(1)
        match &effects[1] {
            Effect::DrawCard { count } => {
                assert_eq!(*count, 1);
            }
            _ => panic!("Expected DrawCard effect as second effect"),
        }
    }

    #[test]
    fn test_flash_of_steel_upgraded_effects() {
        let card = flash_of_steel_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be AttackToTarget(6) - upgraded
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 6);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 0);
            }
            _ => panic!("Expected AttackToTarget effect as first effect"),
        }

        // Second effect should be DrawCard(1) - unchanged
        match &effects[1] {
            Effect::DrawCard { count } => {
                assert_eq!(*count, 1);
            }
            _ => panic!("Expected DrawCard effect as second effect"),
        }
    }
}