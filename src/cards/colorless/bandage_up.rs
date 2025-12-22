use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

/// Bandage Up - Colorless Skill Card
/// Cost: 0 (0 when upgraded)
/// Effect: Heal 4 HP. Exhaust
pub fn bandage_up() -> Card {
    Card::new(
        CardEnum::BandageUp,
        0,
        CardType::Skill,
        vec![
            Effect::Heal(4),
            Effect::Exhaust,
        ],
        Rarity::Uncommon
    )
        .set_playable(true)
}

pub fn bandage_up_upgraded() -> Card {
    Card::new(
        CardEnum::BandageUp,
        0,
        CardType::Skill,
        vec![
            Effect::Heal(6),
            Effect::Exhaust,
        ],
        Rarity::Uncommon
    )
        .set_upgraded(true)
        .set_playable(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bandage_up_creation() {
        let card = bandage_up();

        assert_eq!(card.get_name(), "Bandage Up");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_bandage_up_upgraded_creation() {
        let card = bandage_up_upgraded();

        assert_eq!(card.get_name(), "Bandage Up+");
        assert_eq!(card.get_cost(), 0); // Still costs 0
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_bandage_up_effects() {
        let card = bandage_up();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be Heal(4)
        match &effects[0] {
            Effect::Heal(amount) => {
                assert_eq!(*amount, 4);
            }
            _ => panic!("Expected Heal effect as first effect"),
        }

        // Second effect should be Exhaust
        assert_eq!(effects[1], Effect::Exhaust);
    }

    #[test]
    fn test_bandage_up_upgraded_effects() {
        let card = bandage_up_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be Heal(6) - upgraded
        match &effects[0] {
            Effect::Heal(amount) => {
                assert_eq!(*amount, 6);
            }
            _ => panic!("Expected Heal effect as first effect"),
        }

        // Second effect should be Exhaust (same as base)
        assert_eq!(effects[1], Effect::Exhaust);
    }
}