use crate::game::{card::{Card, Rarity}, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// Dual Wield - Duplicate a card in your hand into the discard pile
pub fn dual_wield() -> Card {
    Card::new(
        CardEnum::DualWield,
        1,
        CardType::Skill,
        vec![Effect::EnterSelectCardToDuplicate { copies: 1 }],
        Rarity::Common
    )
        .set_play_condition(Condition::True)
}

/// Dual Wield+ (Upgraded version) - Duplicate a card in your hand twice into the discard pile
pub fn dual_wield_upgraded() -> Card {
    Card::new(
        CardEnum::DualWield,
        1,
        CardType::Skill,
        vec![Effect::EnterSelectCardToDuplicate { copies: 2 }],
        Rarity::Common
    )
        .set_upgraded(true)
        .set_play_condition(Condition::True)
}

#[cfg(test)]
mod tests {
    use crate::game::PlayerRunState;

    use super::*;

    #[test]
    fn test_dual_wield_creation() {
        let card = dual_wield();

        assert_eq!(card.get_name(), "Dual Wield");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::EnterSelectCardToDuplicate { copies: 1 });
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_dual_wield_upgraded_creation() {
        let card = dual_wield_upgraded();

        assert_eq!(card.get_name(), "Dual Wield+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::EnterSelectCardToDuplicate { copies: 2 });
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_dual_wield_effect_values() {
        let normal_card = dual_wield();
        let upgraded_card = dual_wield_upgraded();

        let normal_effects = normal_card.get_effects();
        let upgraded_effects = upgraded_card.get_effects();

        // Both should have same number of effects
        assert_eq!(normal_effects.len(), 1);
        assert_eq!(upgraded_effects.len(), 1);

        // Normal version duplicates 1 copy
        assert_eq!(normal_effects[0], Effect::EnterSelectCardToDuplicate { copies: 1 });

        // Upgraded version duplicates 2 copies
        assert_eq!(upgraded_effects[0], Effect::EnterSelectCardToDuplicate { copies: 2 });
    }

    #[test]
    fn test_dual_wield_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Dual Wield and Strike in hand
        let deck = Deck::new(vec![dual_wield(), strike()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_discard_size = battle.cards.discard_pile_size();
        assert_eq!(battle.get_battle_state(), crate::battle::battle_state::BattleState::PlayerTurn);

        // Play Dual Wield targeting the player
        let dual_wield_idx = 0;
        let result = battle.play_card(dual_wield_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify battle is now in SelectCardToDuplicate state with copies=1
        assert_eq!(battle.get_battle_state(), crate::battle::battle_state::BattleState::SelectCardToDuplicate { copies: 1 });

        // Verify Strike is still in hand
        let hand = battle.get_hand();
        assert_eq!(hand.len(), 1);
        assert_eq!(hand[0].get_name(), "Strike");

        // Select the Strike card to duplicate it
        let result = battle.eval_action(crate::battle::battle_action::BattleAction::SelectCardToDuplicate(0), &mut rng);
        assert!(result.is_ok());

        // Verify battle returned to PlayerTurn state
        assert_eq!(battle.get_battle_state(), crate::battle::battle_state::BattleState::PlayerTurn);

        // Verify Strike is still in hand
        let hand = battle.get_hand();
        assert_eq!(hand.len(), 1);
        assert_eq!(hand[0].get_name(), "Strike");

        // Verify one copy of Strike was added to discard pile
        // (plus Dual Wield itself which goes to discard as a Skill card)
        let discard_size = battle.cards.discard_pile_size();
        assert_eq!(discard_size, initial_discard_size + 2);
    }

    #[test]
    fn test_dual_wield_upgraded_duplicates_twice() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with upgraded Dual Wield and Strike in hand
        let deck = Deck::new(vec![dual_wield_upgraded(), strike()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_discard_size = battle.cards.discard_pile_size();

        // Play Dual Wield+ targeting the player
        let dual_wield_idx = 0;
        let result = battle.play_card(dual_wield_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify battle is now in SelectCardToDuplicate state with copies=2
        assert_eq!(battle.get_battle_state(), crate::battle::battle_state::BattleState::SelectCardToDuplicate { copies: 2 });

        // Select the Strike card to duplicate it
        let result = battle.eval_action(crate::battle::battle_action::BattleAction::SelectCardToDuplicate(0), &mut rng);
        assert!(result.is_ok());

        // Verify battle returned to PlayerTurn state
        assert_eq!(battle.get_battle_state(), crate::battle::battle_state::BattleState::PlayerTurn);

        // Verify Strike is still in hand
        let hand = battle.get_hand();
        assert_eq!(hand.len(), 1);
        assert_eq!(hand[0].get_name(), "Strike");

        // Verify two copies of Strike were added to discard pile
        // (plus Dual Wield+ itself which goes to discard as a Skill card)
        let discard_size = battle.cards.discard_pile_size();
        assert_eq!(discard_size, initial_discard_size + 3);
    }

    #[test]
    fn test_dual_wield_invalid_card_index() {
        use crate::battle::Battle;
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

        // Create battle with only Dual Wield in hand
        let deck = Deck::new(vec![dual_wield()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Play Dual Wield
        let dual_wield_idx = 0;
        let result = battle.play_card(dual_wield_idx, crate::battle::target::Entity::Player);
        assert!(result.is_ok());

        // Verify battle is in SelectCardToDuplicate state
        assert_eq!(battle.get_battle_state(), crate::battle::battle_state::BattleState::SelectCardToDuplicate { copies: 1 });

        // Try to select an invalid card index (hand should be empty after playing Dual Wield)
        let result = battle.eval_action(crate::battle::battle_action::BattleAction::SelectCardToDuplicate(0), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), crate::battle::BattleError::CardNotInHand);
    }
}
