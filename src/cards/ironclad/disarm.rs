use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

/// Disarm - Skill Card
/// Cost: 1
/// Effect: Target enemy loses 2 Strength.
pub fn disarm() -> Card {
    Card::new(CardEnum::Disarm, 1, CardClass::IronClad(Rarity::Uncommon, CardType::Skill), vec![
        BattleEffect::LoseStrengthTarget(2),
    ])
}

/// Disarm+ (Upgraded)
/// Cost: 1
/// Effect: Target enemy loses 3 Strength.
pub fn disarm_upgraded() -> Card {
    Card::new(CardEnum::Disarm, 1, CardClass::IronClad(Rarity::Uncommon, CardType::Skill), vec![
        BattleEffect::LoseStrengthTarget(3),
    ])
        .set_upgraded(true)
}

#[cfg(test)]
mod tests {
    use crate::game::PlayerRunState;

    use super::*;

    #[test]
    fn test_disarm_creation() {
        let card = disarm();

        assert_eq!(card.get_name(), "Disarm");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], BattleEffect::LoseStrengthTarget(2));
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_disarm_upgraded_creation() {
        let card = disarm_upgraded();

        assert_eq!(card.get_name(), "Disarm+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], BattleEffect::LoseStrengthTarget(3));
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_disarm_effect_values() {
        let normal_card = disarm();
        let upgraded_card = disarm_upgraded();

        let normal_effects = normal_card.get_effects();
        let upgraded_effects = upgraded_card.get_effects();

        assert_eq!(normal_effects[0], BattleEffect::LoseStrengthTarget(2));
        assert_eq!(upgraded_effects[0], BattleEffect::LoseStrengthTarget(3));
    }

    #[test]
    fn test_disarm_battle_integration() {
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

        // Create battle with Disarm in hand
        let deck = Deck::new(vec![disarm()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Play Disarm targeting the enemy
        let disarm_idx = 0;
        let result = battle.play_card(disarm_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify Disarm went to discard pile (not a power card)
        let discard = battle.cards.get_discard_pile();
        assert_eq!(discard.len(), 1);
        assert_eq!(discard[0].get_name(), "Disarm");

        // TODO: Add test for strength loss once the enemy strength system is fully implemented
        // This would involve checking that the enemy lost 2 Strength
    }

    #[test]
    fn test_disarm_upgraded_battle_integration() {
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

        // Create battle with Disarm+ in hand
        let deck = Deck::new(vec![disarm_upgraded()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Play Disarm+ targeting the enemy
        let disarm_idx = 0;
        let result = battle.play_card(disarm_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify Disarm+ went to discard pile
        let discard = battle.cards.get_discard_pile();
        assert_eq!(discard.len(), 1);
        assert_eq!(discard[0].get_name(), "Disarm+");

        // TODO: Add test for strength loss once the enemy strength system is fully implemented
        // This would involve checking that the enemy lost 3 Strength
    }
}