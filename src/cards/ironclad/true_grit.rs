use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// True Grit - Gain 7 Block. Exhaust 1 card from hand
pub fn true_grit() -> Card {
    Card::new(
        CardEnum::TrueGrit,
        1,
        CardType::Skill,
        vec![Effect::GainDefense { amount: 7 }, Effect::EnterSelectCardInHand],
        false, // not upgraded
        true,  // playable
    )
}

/// True Grit+ (upgraded version)
pub fn true_grit_upgraded() -> Card {
    Card::new(
        CardEnum::TrueGrit,
        1,
        CardType::Skill,
        vec![Effect::GainDefense { amount: 9 }, Effect::EnterSelectCardInHand],
        true,  // upgraded
        true,  // playable
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    
    use crate::game::enemy::EnemyTrait;

    #[test]
    fn test_true_grit_card_creation() {
        let true_grit_card = true_grit();
        assert_eq!(true_grit_card.get_name(), "True Grit");
        assert_eq!(true_grit_card.get_cost(), 1);
        assert_eq!(true_grit_card.get_card_type(), &CardType::Skill);

        let effects = true_grit_card.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::GainDefense { amount: 7 })));
        assert!(effects.contains(&Effect::EnterSelectCardInHand));
    }

    #[test]
    fn test_true_grit_upgraded_card_creation() {
        let true_grit_plus = true_grit_upgraded();
        assert_eq!(true_grit_plus.get_name(), "True Grit+");
        assert_eq!(true_grit_plus.get_cost(), 1);
        assert_eq!(true_grit_plus.get_card_type(), &CardType::Skill);

        let effects = true_grit_plus.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::GainDefense { amount: 9 })));
        assert!(effects.contains(&Effect::EnterSelectCardInHand));
        assert!(true_grit_plus.is_upgraded());
    }

    #[test]
    fn test_true_grit_card_enum() {
        let true_grit_card = true_grit();
        let card_enum = true_grit_card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::TrueGrit));
    }

}