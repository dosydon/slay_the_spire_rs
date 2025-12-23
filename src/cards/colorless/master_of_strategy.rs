use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

/// Master of Strategy - Colorless Skill Card (Rare)
/// Cost: 0 (0 when upgraded)
/// Effect: Draw 3 cards. Exhaust
pub fn master_of_strategy() -> Card {
    Card::new(
        CardEnum::MasterOfStrategy,
        0,
        CardClass::Colorless(Rarity::Rare, CardType::Skill),
        vec![
            BattleEffect::DrawCard { count: 3 },
            BattleEffect::Exhaust,
        ]
    )
        .set_playable(true)
}

pub fn master_of_strategy_upgraded() -> Card {
    Card::new(
        CardEnum::MasterOfStrategy,
        0,
        CardClass::Colorless(Rarity::Rare, CardType::Skill),
        vec![
            BattleEffect::DrawCard { count: 4 },
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
    fn test_master_of_strategy_creation() {
        let card = master_of_strategy();

        assert_eq!(card.get_name(), "Master of Strategy");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_master_of_strategy_upgraded_creation() {
        let card = master_of_strategy_upgraded();

        assert_eq!(card.get_name(), "Master of Strategy+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_master_of_strategy_effects() {
        let card = master_of_strategy();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be DrawCard(3)
        match &effects[0] {
            BattleEffect::DrawCard { count } => {
                assert_eq!(*count, 3);
            }
            _ => panic!("Expected DrawCard effect as first effect"),
        }

        // Second effect should be Exhaust
        assert_eq!(effects[1], BattleEffect::Exhaust);
    }

    #[test]
    fn test_master_of_strategy_upgraded_effects() {
        let card = master_of_strategy_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be DrawCard(4) - upgraded
        match &effects[0] {
            BattleEffect::DrawCard { count } => {
                assert_eq!(*count, 4);
            }
            _ => panic!("Expected DrawCard effect as first effect"),
        }

        // Second effect should be Exhaust (same as base)
        assert_eq!(effects[1], BattleEffect::Exhaust);
    }
}
