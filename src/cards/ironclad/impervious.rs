use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

/// Impervious - Rare Skill Card
/// Cost: 2
/// Effect: Gain 30 Block. Exhaust.
pub fn impervious() -> Card {
    Card::new(CardEnum::Impervious, 2, CardClass::IronClad(Rarity::Rare, CardType::Skill), vec![
        BattleEffect::GainDefense { amount: 30 },
        BattleEffect::Exhaust,
    ])
        .set_playable(true)
}

/// Impervious+ (Upgraded)
/// Cost: 2
/// Effect: Gain 40 Block. Exhaust.
pub fn impervious_upgraded() -> Card {
    Card::new(CardEnum::Impervious, 2, CardClass::IronClad(Rarity::Rare, CardType::Skill), vec![
        BattleEffect::GainDefense { amount: 40 },
        BattleEffect::Exhaust,
    ])
        .set_upgraded(true)
        .set_playable(true)
}

#[cfg(test)]
mod tests {
    use crate::game::PlayerRunState;

    use super::*;

    #[test]
    fn test_impervious_creation() {
        let card = impervious();

        assert_eq!(card.get_name(), "Impervious");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert_eq!(card.get_effects()[0], BattleEffect::GainDefense { amount: 30 });
        assert_eq!(card.get_effects()[1], BattleEffect::Exhaust);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_impervious_upgraded_creation() {
        let card = impervious_upgraded();

        assert_eq!(card.get_name(), "Impervious+");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert_eq!(card.get_effects()[0], BattleEffect::GainDefense { amount: 40 });
        assert_eq!(card.get_effects()[1], BattleEffect::Exhaust);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_impervious_block_values() {
        let normal_card = impervious();
        let upgraded_card = impervious_upgraded();

        let normal_effects = normal_card.get_effects();
        let upgraded_effects = upgraded_card.get_effects();

        // Both should have Exhaust effect
        assert_eq!(normal_effects[1], BattleEffect::Exhaust);
        assert_eq!(upgraded_effects[1], BattleEffect::Exhaust);

        // Normal should grant 30 block, upgraded should grant 40
        assert_eq!(normal_effects[0], BattleEffect::GainDefense { amount: 30 });
        assert_eq!(upgraded_effects[0], BattleEffect::GainDefense { amount: 40 });
    }

    #[test]
    fn test_impervious_battle_integration() {
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

        // Create battle with Impervious in hand
        let deck = Deck::new(vec![impervious()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_block = battle.get_player().get_block();
        assert_eq!(initial_block, 0);

        // Play Impervious targeting the player
        let impervious_idx = 0;
        let result = battle.play_card(impervious_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify player gained 30 block
        let block_after_impervious = battle.get_player().get_block();
        assert_eq!(block_after_impervious, 30);

        // Verify Impervious is exhausted (not in hand, not in discard)
        let hand = battle.get_hand();
        let discard = battle.get_discard_pile();
        assert_eq!(hand.len(), 0); // Hand should be empty after playing
        // Card should be exhausted (not in discard pile)
        assert!(!discard.iter().any(|card| card.get_name() == "Impervious"));
    }

    #[test]
    fn test_impervious_upgraded_battle_integration() {
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

        // Create battle with Impervious+ in hand
        let deck = Deck::new(vec![impervious_upgraded()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_block = battle.get_player().get_block();
        assert_eq!(initial_block, 0);

        // Play Impervious+ targeting the player
        let impervious_idx = 0;
        let result = battle.play_card(impervious_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify player gained 40 block
        let block_after_impervious = battle.get_player().get_block();
        assert_eq!(block_after_impervious, 40);

        // Verify Impervious+ is exhausted
        let hand = battle.get_hand();
        let discard = battle.get_discard_pile();
        assert_eq!(hand.len(), 0);
        assert!(!discard.iter().any(|card| card.get_name() == "Impervious+"));
    }
}