use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

/// Finesse - Colorless Skill Card
/// Cost: 0 (0 when upgraded)
/// Effect: Gain 2 Block. Draw 1 card
pub fn finesse() -> Card {
    Card::new(
        CardEnum::Finesse,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Skill),
        vec![
            BattleEffect::GainDefense { amount: 2 },
            BattleEffect::DrawCard { count: 1 },
        ]
    )
        .set_playable(true)
}

pub fn finesse_upgraded() -> Card {
    Card::new(
        CardEnum::Finesse,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Skill),
        vec![
            BattleEffect::GainDefense { amount: 4 },
            BattleEffect::DrawCard { count: 1 },
        ]
    )
        .set_upgraded(true)
        .set_playable(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finesse_creation() {
        let card = finesse();

        assert_eq!(card.get_name(), "Finesse");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_finesse_upgraded_creation() {
        let card = finesse_upgraded();

        assert_eq!(card.get_name(), "Finesse+");
        assert_eq!(card.get_cost(), 0); // Still costs 0
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_finesse_effects() {
        let card = finesse();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be GainDefense(2)
        match &effects[0] {
            BattleEffect::GainDefense { amount } => {
                assert_eq!(*amount, 2);
            }
            _ => panic!("Expected GainDefense effect as first effect"),
        }

        // Second effect should be DrawCard(1)
        match &effects[1] {
            BattleEffect::DrawCard { count } => {
                assert_eq!(*count, 1);
            }
            _ => panic!("Expected DrawCard effect as second effect"),
        }
    }

    #[test]
    fn test_finesse_upgraded_effects() {
        let card = finesse_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be GainDefense(4) - upgraded
        match &effects[0] {
            BattleEffect::GainDefense { amount } => {
                assert_eq!(*amount, 4);
            }
            _ => panic!("Expected GainDefense effect as first effect"),
        }

        // Second effect should be DrawCard(1) - unchanged
        match &effects[1] {
            BattleEffect::DrawCard { count } => {
                assert_eq!(*count, 1);
            }
            _ => panic!("Expected DrawCard effect as second effect"),
        }
    }
}