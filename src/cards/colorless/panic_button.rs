use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

/// Panic Button - Colorless Skill Card (Uncommon)
/// Cost: 0 (0 when upgraded)
/// Effect: Gain 30 Block. Exhaust
/// Note: In the full game, this also prevents gaining Block from cards for 2 turns,
/// but that debuff system is not yet implemented.
pub fn panic_button() -> Card {
    Card::new(
        CardEnum::PanicButton,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Skill),
        vec![
            BattleEffect::GainDefense { amount: 30 },
            BattleEffect::Exhaust,
        ]
    )
        .set_playable(true)
}

pub fn panic_button_upgraded() -> Card {
    Card::new(
        CardEnum::PanicButton,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Skill),
        vec![
            BattleEffect::GainDefense { amount: 40 },
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
    fn test_panic_button_creation() {
        let card = panic_button();

        assert_eq!(card.get_name(), "Panic Button");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_panic_button_upgraded_creation() {
        let card = panic_button_upgraded();

        assert_eq!(card.get_name(), "Panic Button+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_panic_button_effects() {
        let card = panic_button();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be GainDefense(30)
        match &effects[0] {
            BattleEffect::GainDefense { amount } => {
                assert_eq!(*amount, 30);
            }
            _ => panic!("Expected GainDefense effect as first effect"),
        }

        // Second effect should be Exhaust
        assert_eq!(effects[1], BattleEffect::Exhaust);
    }

    #[test]
    fn test_panic_button_upgraded_effects() {
        let card = panic_button_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be GainDefense(40) - upgraded
        match &effects[0] {
            BattleEffect::GainDefense { amount } => {
                assert_eq!(*amount, 40);
            }
            _ => panic!("Expected GainDefense effect as first effect"),
        }

        // Second effect should be Exhaust (same as base)
        assert_eq!(effects[1], BattleEffect::Exhaust);
    }
}
