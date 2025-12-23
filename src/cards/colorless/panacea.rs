use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

/// Panacea - Colorless Skill Card (Uncommon)
/// Cost: 0 (0 when upgraded)
/// Effect: Gain 1 Artifact. Exhaust
pub fn panacea() -> Card {
    Card::new(
        CardEnum::Panacea,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Skill),
        vec![
            BattleEffect::GainArtifact { amount: 1 },
            BattleEffect::Exhaust,
        ]
    )
        .set_playable(true)
}

pub fn panacea_upgraded() -> Card {
    Card::new(
        CardEnum::Panacea,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Skill),
        vec![
            BattleEffect::GainArtifact { amount: 2 },
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
    fn test_panacea_creation() {
        let card = panacea();

        assert_eq!(card.get_name(), "Panacea");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_panacea_upgraded_creation() {
        let card = panacea_upgraded();

        assert_eq!(card.get_name(), "Panacea+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_panacea_effects() {
        let card = panacea();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be GainArtifact(1)
        match &effects[0] {
            BattleEffect::GainArtifact { amount } => {
                assert_eq!(*amount, 1);
            }
            _ => panic!("Expected GainArtifact effect as first effect"),
        }

        // Second effect should be Exhaust
        assert_eq!(effects[1], BattleEffect::Exhaust);
    }

    #[test]
    fn test_panacea_upgraded_effects() {
        let card = panacea_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be GainArtifact(2) - upgraded
        match &effects[0] {
            BattleEffect::GainArtifact { amount } => {
                assert_eq!(*amount, 2);
            }
            _ => panic!("Expected GainArtifact effect as first effect"),
        }

        // Second effect should be Exhaust (same as base)
        assert_eq!(effects[1], BattleEffect::Exhaust);
    }
}
