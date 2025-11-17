use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::{Effect, Condition}};

/// Feel No Pain - Skill Card
/// Cost: 1
/// Effect: Gain 5 Plated Armor. Exhaust. (Gains 3 Block when exhausted)
pub fn feel_no_pain() -> Card {
    Card::new(CardEnum::FeelNoPain, 1, CardType::Skill, vec![
        Effect::GainPlatedArmor(5),
        Effect::Exhaust,
        Effect::GainDefense { amount: 3 }, // Gain 3 Block when exhausted
    ], false, true) // Initially playable
}

/// Feel No Pain+ (Upgraded)
/// Cost: 1
/// Effect: Gain 8 Plated Armor. Exhaust. (Gains 3 Block when exhausted)
pub fn feel_no_pain_upgraded() -> Card {
    Card::new(CardEnum::FeelNoPain, 1, CardType::Skill, vec![
        Effect::GainPlatedArmor(8),
        Effect::Exhaust,
        Effect::GainDefense { amount: 3 }, // Gain 3 Block when exhausted
    ], true, true) // Initially playable
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feel_no_pain_creation() {
        let card = feel_no_pain();

        assert_eq!(card.get_name(), "Feel No Pain");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 3);
        assert_eq!(card.get_effects()[0], Effect::GainPlatedArmor(5));
        assert_eq!(card.get_effects()[1], Effect::Exhaust);
        assert_eq!(card.get_effects()[2], Effect::GainDefense { amount: 3 });
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_feel_no_pain_upgraded_creation() {
        let card = feel_no_pain_upgraded();

        assert_eq!(card.get_name(), "Feel No Pain+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 3);
        assert_eq!(card.get_effects()[0], Effect::GainPlatedArmor(8));
        assert_eq!(card.get_effects()[1], Effect::Exhaust);
        assert_eq!(card.get_effects()[2], Effect::GainDefense { amount: 3 });
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_feel_no_pain_effect_values() {
        let normal_card = feel_no_pain();
        let upgraded_card = feel_no_pain_upgraded();

        let normal_effects = normal_card.get_effects();
        let upgraded_effects = upgraded_card.get_effects();

        // Check plated armor amounts
        assert_eq!(normal_effects[0], Effect::GainPlatedArmor(5));
        assert_eq!(upgraded_effects[0], Effect::GainPlatedArmor(8));

        // Both should have exhaust effect
        assert_eq!(normal_effects[1], Effect::Exhaust);
        assert_eq!(upgraded_effects[1], Effect::Exhaust);

        // Both should gain 3 defense when exhausted
        assert_eq!(normal_effects[2], Effect::GainDefense { amount: 3 });
        assert_eq!(upgraded_effects[2], Effect::GainDefense { amount: 3 });
    }

    #[test]
    fn test_feel_no_pain_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Feel No Pain in hand
        let deck = Deck::new(vec![feel_no_pain()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        let initial_exhausted_size = battle.cards.exhausted_size();

        // Play Feel No Pain targeting the player
        let feel_no_pain_idx = 0;
        let result = battle.play_card(feel_no_pain_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Feel No Pain was exhausted (should be in exhausted pile)
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted_size + 1);
        let exhausted_pile = battle.cards.get_exhausted();
        assert_eq!(exhausted_pile.len(), 1);
        assert_eq!(exhausted_pile[0].get_name(), "Feel No Pain");

        // TODO: Add test for plated armor gain once the system is implemented
        // This would involve checking that the player gained 5 plated armor
        // TODO: Add test for 3 defense gain when exhausted
        // This would involve checking that the player gained 3 block from the exhaustion effect
    }

    #[test]
    fn test_feel_no_pain_upgraded_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Feel No Pain+ in hand
        let deck = Deck::new(vec![feel_no_pain_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        let initial_exhausted_size = battle.cards.exhausted_size();

        // Play Feel No Pain+ targeting the player
        let feel_no_pain_idx = 0;
        let result = battle.play_card(feel_no_pain_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Feel No Pain+ was exhausted
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted_size + 1);
        let exhausted_pile = battle.cards.get_exhausted();
        assert_eq!(exhausted_pile.len(), 1);
        assert_eq!(exhausted_pile[0].get_name(), "Feel No Pain+");

        // TODO: Add test for plated armor gain once the system is implemented
        // This would involve checking that the player gained 8 plated armor
        // TODO: Add test for 3 defense gain when exhausted
        // This would involve checking that the player gained 3 block from the exhaustion effect
    }
}