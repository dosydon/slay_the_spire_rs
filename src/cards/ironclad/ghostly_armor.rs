use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

pub fn ghostly_armor() -> Card {
    Card::new(CardEnum::GhostlyArmor, 1, CardClass::IronClad(Rarity::Uncommon, CardType::Skill), vec![
        BattleEffect::GainDefense { amount: 10 }
        // Note: Ethereal is a card property, not an effect that needs to be in effects list
        // The card should be marked as ethereal via card properties
    ])
        .set_ethereal(true)
        .set_playable(true)
}

pub fn ghostly_armor_upgraded() -> Card {
    Card::new(CardEnum::GhostlyArmor, 1, CardClass::IronClad(Rarity::Uncommon, CardType::Skill), vec![
        BattleEffect::GainDefense { amount: 13 }
    ])
        .set_upgraded(true)
        .set_ethereal(true)
        .set_playable(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghostly_armor_basic() {
        let card = ghostly_armor();
        assert_eq!(card.get_name(), "Ghostly Armor");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert!(!card.is_upgraded());

        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        assert!(effects.contains(&BattleEffect::GainDefense { amount: 10 }));
    }

    #[test]
    fn test_ghostly_armor_upgraded() {
        let card = ghostly_armor_upgraded();
        assert_eq!(card.get_name(), "Ghostly Armor+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert!(card.is_upgraded());

        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        assert!(effects.contains(&BattleEffect::GainDefense { amount: 13 }));
    }
}
