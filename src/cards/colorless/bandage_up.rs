use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

/// Bandage Up - Colorless Skill Card
/// Cost: 0 (0 when upgraded)
/// Effect: Heal 4 HP. Exhaust
pub fn bandage_up() -> Card {
    Card::new(
        CardEnum::BandageUp,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Skill),
        vec![
            BattleEffect::Heal(4),
            BattleEffect::Exhaust,
        ]
    )
        .set_playable(true)
}

pub fn bandage_up_upgraded() -> Card {
    Card::new(
        CardEnum::BandageUp,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Skill),
        vec![
            BattleEffect::Heal(6),
            BattleEffect::Exhaust,
        ]
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
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_bandage_up_upgraded_creation() {
        let card = bandage_up_upgraded();

        assert_eq!(card.get_name(), "Bandage Up+");
        assert_eq!(card.get_cost(), 0); // Still costs 0
        assert_eq!(card.get_card_type(), CardType::Skill);
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
            BattleEffect::Heal(amount) => {
                assert_eq!(*amount, 4);
            }
            _ => panic!("Expected Heal effect as first effect"),
        }

        // Second effect should be Exhaust
        assert_eq!(effects[1], BattleEffect::Exhaust);
    }

    #[test]
    fn test_bandage_up_upgraded_effects() {
        let card = bandage_up_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be Heal(6) - upgraded
        match &effects[0] {
            BattleEffect::Heal(amount) => {
                assert_eq!(*amount, 6);
            }
            _ => panic!("Expected Heal effect as first effect"),
        }

        // Second effect should be Exhaust (same as base)
        assert_eq!(effects[1], BattleEffect::Exhaust);
    }
}