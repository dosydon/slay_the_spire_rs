use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

pub fn feed() -> Card {
    Card::new(
        CardEnum::Feed,
        1,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: 10, num_attacks: 1, strength_multiplier: 1 },
            Effect::HealAndIncreaseMaxHp(3),
            Effect::Exhaust,
        ],
        false, // not upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

pub fn feed_upgraded() -> Card {
    Card::new(
        CardEnum::Feed,
        1,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: 12, num_attacks: 1, strength_multiplier: 1 },
            Effect::HealAndIncreaseMaxHp(4),
            Effect::Exhaust,
        ],
        true,  // upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feed_creation() {
        let card = feed();
        assert_eq!(card.get_name(), "Feed");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert!(!card.is_upgraded());
        assert_eq!(card.get_effects().len(), 3);
        assert!(card.is_playable());
    }

    #[test]
    fn test_feed_upgraded_creation() {
        let card = feed_upgraded();
        assert_eq!(card.get_name(), "Feed+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert!(card.is_upgraded());
        assert_eq!(card.get_effects().len(), 3);
        assert!(card.is_playable());
    }

    #[test]
    fn test_feed_effects() {
        let card = feed();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 3);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 10);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
        match &effects[1] {
            Effect::HealAndIncreaseMaxHp(amount) => {
                assert_eq!(*amount, 3);
            }
            _ => panic!("Expected HealAndIncreaseMaxHp effect"),
        }
        assert_eq!(effects[2], Effect::Exhaust);
    }

    #[test]
    fn test_feed_upgraded_effects() {
        let card = feed_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 3);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 12);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
        match &effects[1] {
            Effect::HealAndIncreaseMaxHp(amount) => {
                assert_eq!(*amount, 4);
            }
            _ => panic!("Expected HealAndIncreaseMaxHp effect"),
        }
        assert_eq!(effects[2], Effect::Exhaust);
    }
}

