use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

/// Dramatic Entrance - Colorless Attack Card (Uncommon)
/// Cost: 0 (0 when upgraded)
/// Effect: Innate. Deal 8 damage to ALL enemies. Exhaust
pub fn dramatic_entrance() -> Card {
    Card::new(
        CardEnum::DramaticEntrance,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Attack),
        vec![
            BattleEffect::AttackAllEnemies { amount: 8, num_attacks: 1 },
            BattleEffect::Exhaust,
        ]
    )
        .set_playable(true)
        .set_innate(true) // This card starts in your opening hand
}

pub fn dramatic_entrance_upgraded() -> Card {
    Card::new(
        CardEnum::DramaticEntrance,
        0,
        CardClass::Colorless(Rarity::Uncommon, CardType::Attack),
        vec![
            BattleEffect::AttackAllEnemies { amount: 12, num_attacks: 1 }, // Increased to 12
            BattleEffect::Exhaust,
        ]
    )
        .set_upgraded(true)
        .set_playable(true)
        .set_innate(true) // This card starts in your opening hand
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dramatic_entrance_creation() {
        let card = dramatic_entrance();

        assert_eq!(card.get_name(), "Dramatic Entrance");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Attack); // Should be Attack, not Skill
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
        assert!(card.is_innate()); // Should be innate
    }

    #[test]
    fn test_dramatic_entrance_upgraded_creation() {
        let card = dramatic_entrance_upgraded();

        assert_eq!(card.get_name(), "Dramatic Entrance+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), CardType::Attack);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
        assert!(card.is_innate()); // Should be innate
    }

    #[test]
    fn test_dramatic_entrance_effects() {
        let card = dramatic_entrance();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be AttackAllEnemies(8, 1)
        match &effects[0] {
            BattleEffect::AttackAllEnemies { amount, num_attacks } => {
                assert_eq!(*amount, 8);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackAllEnemies effect as first effect"),
        }

        // Second effect should be Exhaust
        assert_eq!(effects[1], BattleEffect::Exhaust);
    }

    #[test]
    fn test_dramatic_entrance_upgraded_effects() {
        let card = dramatic_entrance_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);

        // First effect should be AttackAllEnemies(12, 1) - upgraded
        match &effects[0] {
            BattleEffect::AttackAllEnemies { amount, num_attacks } => {
                assert_eq!(*amount, 12);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackAllEnemies effect as first effect"),
        }

        // Second effect should be Exhaust (same as base)
        assert_eq!(effects[1], BattleEffect::Exhaust);
    }
}
