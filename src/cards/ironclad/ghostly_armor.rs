use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::{Effect, Condition}};

pub fn ghostly_armor() -> Card {
    Card::new(CardEnum::GhostlyArmor, 1, CardType::Skill, vec![
        Effect::GainDefense { amount: 10 }
        // Note: Ethereal is a card property, not an effect that needs to be in effects list
        // The card should be marked as ethereal via card properties
    ], false, true)
}

pub fn ghostly_armor_upgraded() -> Card {
    Card::new(CardEnum::GhostlyArmor, 1, CardType::Skill, vec![
        Effect::GainDefense { amount: 13 }
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghostly_armor_basic() {
        let card = ghostly_armor();
        assert_eq!(card.get_name(), "Ghostly Armor");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(!card.is_upgraded());

        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        assert!(effects.contains(&Effect::GainDefense { amount: 10 }));
    }

    #[test]
    fn test_ghostly_armor_upgraded() {
        let card = ghostly_armor_upgraded();
        assert_eq!(card.get_name(), "Ghostly Armor+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(card.is_upgraded());

        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        assert!(effects.contains(&Effect::GainDefense { amount: 13 }));
    }
}
