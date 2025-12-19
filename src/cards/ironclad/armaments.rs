use crate::game::{card::{Card, Rarity}, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// Armaments - Gain 5 Block. Upgrade a card in your hand for the rest of combat.
pub fn armaments() -> Card {
    Card::new_with_condition(
        CardEnum::Armaments,
        1,
        CardType::Skill,
        vec![Effect::GainDefense { amount: 5 }, Effect::EnterSelectCardInHand],
        false, // not upgraded
        Condition::True,
        Rarity::Uncommon)
}

/// Armaments+ (Upgraded version) - Gain 5 Block. Upgrade ALL cards in your hand for the rest of combat.
pub fn armaments_upgraded() -> Card {
    Card::new_with_condition(
        CardEnum::Armaments,
        1,
        CardType::Skill,
        vec![Effect::GainDefense { amount: 5 }, Effect::UpgradeAllCardsInHand],
        true,  // upgraded
        Condition::True,
        Rarity::Uncommon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_armaments_creation() {
        let card = armaments();

        assert_eq!(card.get_name(), "Armaments");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert_eq!(card.get_effects()[0], Effect::GainDefense { amount: 5 });
        assert_eq!(card.get_effects()[1], Effect::EnterSelectCardInHand);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_armaments_upgraded_creation() {
        let card = armaments_upgraded();

        assert_eq!(card.get_name(), "Armaments+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert_eq!(card.get_effects()[0], Effect::GainDefense { amount: 5 });
        assert_eq!(card.get_effects()[1], Effect::UpgradeAllCardsInHand);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_armaments_effect_values() {
        let normal_card = armaments();
        let upgraded_card = armaments_upgraded();

        let normal_effects = normal_card.get_effects();
        let upgraded_effects = upgraded_card.get_effects();

        // Both should have same number of effects
        assert_eq!(normal_effects.len(), 2);
        assert_eq!(upgraded_effects.len(), 2);

        // Both should gain 5 defense
        assert_eq!(normal_effects[0], Effect::GainDefense { amount: 5 });
        assert_eq!(upgraded_effects[0], Effect::GainDefense { amount: 5 });

        // Normal should enter select card state, upgraded should upgrade all cards
        assert_eq!(normal_effects[1], Effect::EnterSelectCardInHand);
        assert_eq!(upgraded_effects[1], Effect::UpgradeAllCardsInHand);
    }

    #[test]
    fn test_armaments_battle_integration() {
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

        // Create battle with Armaments and Strike in hand
        let deck = Deck::new(vec![armaments(), strike()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        let initial_block = battle.get_player().get_block();
        assert_eq!(initial_block, 0);
        assert_eq!(battle.get_battle_state(), crate::battle::action::BattleState::PlayerTurn);

        // Play Armaments targeting the player
        let armaments_idx = 0;
        let result = battle.play_card(armaments_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify player gained 5 block
        let block_after_armaments = battle.get_player().get_block();
        assert_eq!(block_after_armaments, 5);

        // Verify battle is now in SelectCardInHand state
        assert_eq!(battle.get_battle_state(), crate::battle::action::BattleState::SelectCardInHand);

        // Verify Strike is still in hand and not upgraded
        let hand = battle.get_hand();
        assert_eq!(hand.len(), 1);
        assert_eq!(hand[0].get_name(), "Strike");
        assert!(!hand[0].is_upgraded());

        // Select the Strike card to upgrade it
        let result = battle.eval_action(crate::battle::action::Action::SelectCardInHand(0), &mut rng);
        assert!(result.is_ok());

        // Verify battle returned to PlayerTurn state
        assert_eq!(battle.get_battle_state(), crate::battle::action::BattleState::PlayerTurn);

        // Verify Strike is now upgraded
        let hand = battle.get_hand();
        assert_eq!(hand.len(), 1);
        assert_eq!(hand[0].get_name(), "Strike+");
        assert!(hand[0].is_upgraded());
    }

    #[test]
    fn test_armaments_skip_upgrading_upgraded_card() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike_upgraded;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Armaments and already-upgraded Strike in hand
        let deck = Deck::new(vec![armaments(), strike_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Play Armaments
        let armaments_idx = 0;
        let result = battle.play_card(armaments_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify battle is in SelectCardInHand state
        assert_eq!(battle.get_battle_state(), crate::battle::action::BattleState::SelectCardInHand);

        // Try to upgrade the already-upgraded Strike
        let result = battle.eval_action(crate::battle::action::Action::SelectCardInHand(0), &mut rng);
        assert!(result.is_ok());

        // Verify Strike is still upgraded (no change)
        let hand = battle.get_hand();
        assert_eq!(hand.len(), 1);
        assert_eq!(hand[0].get_name(), "Strike+");
        assert!(hand[0].is_upgraded());

        // Verify battle returned to PlayerTurn state
        assert_eq!(battle.get_battle_state(), crate::battle::action::BattleState::PlayerTurn);
    }

    #[test]
    fn test_armaments_invalid_card_index() {
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

        // Create battle with only Armaments in hand
        let deck = Deck::new(vec![armaments()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Play Armaments
        let armaments_idx = 0;
        let result = battle.play_card(armaments_idx, crate::battle::target::Entity::Player);
        assert!(result.is_ok());

        // Verify battle is in SelectCardInHand state
        assert_eq!(battle.get_battle_state(), crate::battle::action::BattleState::SelectCardInHand);

        // Try to select an invalid card index (hand should be empty after playing Armaments)
        let result = battle.eval_action(crate::battle::action::Action::SelectCardInHand(0), &mut rng);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), crate::battle::BattleError::CardNotInHand);
    }

    #[test]
    fn test_armaments_upgraded_upgrades_all_cards_in_hand() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike;
        use crate::cards::ironclad::defend::defend;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Armaments+ and multiple cards in hand
        let deck = Deck::new(vec![armaments_upgraded(), strike(), defend()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Play Armaments+ (index 0)
        let armaments_idx = 0;
        let result = battle.play_card(armaments_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify player gained 5 block
        let block_after_armaments = battle.get_player().get_block();
        assert_eq!(block_after_armaments, 5);

        // Verify battle should still be in PlayerTurn state (no card selection needed)
        assert_eq!(battle.get_battle_state(), crate::battle::action::BattleState::PlayerTurn);

        // Verify all cards in hand are now upgraded
        let hand = battle.get_hand();
        assert_eq!(hand.len(), 2); // Strike and Defend should remain

        // Check Strike is upgraded
        let strike_card = hand.iter().find(|c| c.get_name() == "Strike+").unwrap();
        assert!(strike_card.is_upgraded());

        // Check Defend is upgraded
        let defend_card = hand.iter().find(|c| c.get_name() == "Defend+").unwrap();
        assert!(defend_card.is_upgraded());
    }
}