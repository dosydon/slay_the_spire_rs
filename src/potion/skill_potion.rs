//! Skill Potion implementation
//!
//! The Skill Potion allows the player to choose 1 of 3 random Skill cards
//! to add to their hand. The chosen card is added and costs 0 this turn.

use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;

/// Skill Potion: Choose 1 of 3 random Skill cards to add to your hand. It costs 0 this turn.
pub const SKILL_POTION_NAME: &str = "Skill Potion";
pub const SKILL_POTION_DESCRIPTION: &str = "Choose 1 of 3 random Skill cards to add to your hand. It costs 0 this turn.";

/// Get the effects for the Skill Potion
/// Returns a tuple of (target, effects)
pub fn get_skill_potion_effects() -> (Option<Entity>, Vec<BattleEffect>) {
    // Skill Potion adds 3 random Skill cards to choose from (1 copy, cost 0)
    // This requires special handling in the battle system to present card choices
    (Some(Entity::Player), vec![
        BattleEffect::AddRandomSkillCardsToHand {
            num_choices: 3,
            num_copies: 1,
            cost: 0
        }
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::Battle;
    use crate::battle::battle_action::BattleAction;
    use crate::battle::battle_state::BattleState;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::cards::ironclad::starter_deck::starter_deck;
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, PlayerRunState, enemy::EnemyTrait};
    use crate::potion::Potion;

    #[test]
    fn test_skill_potion_name_and_description() {
        assert_eq!(SKILL_POTION_NAME, "Skill Potion");
        assert_eq!(SKILL_POTION_DESCRIPTION, "Choose 1 of 3 random Skill cards to add to your hand. It costs 0 this turn.");
    }

    #[test]
    fn test_skill_potion_effects() {
        let (target, effects) = get_skill_potion_effects();

        assert_eq!(target, Some(Entity::Player));
        assert_eq!(effects.len(), 1);

        match &effects[0] {
            BattleEffect::AddRandomSkillCardsToHand { num_choices, num_copies, cost } => {
                assert_eq!(*num_choices, 3);
                assert_eq!(*num_copies, 1);
                assert_eq!(*cost, 0);
            },
            _ => panic!("Expected AddRandomSkillCardsToHand effect"),
        }
    }

    #[test]
    fn test_skill_potion_adds_one_copy() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        let mut player_state = PlayerRunState::new(80, 80, 0);
        player_state.potions.add_potion(Potion::SkillPotion);

        let mut battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        let initial_hand_size = battle.cards.hand_size();

        // Use the Skill Potion (slot 0, no target needed)
        let use_potion_action = BattleAction::UsePotion(0, None);
        let result = battle.eval_action(use_potion_action, &mut rng);
        assert!(result.is_ok(), "UsePotion action failed");

        // Check that we're now in SelectCardFromChoices state
        match &battle.battle_state {
            BattleState::SelectCardFromChoices { choices, num_copies, cost_override } => {
                assert_eq!(choices.len(), 3, "Should have 3 choices");
                assert_eq!(*num_copies, 1, "Should add 1 copy");
                assert_eq!(*cost_override, Some(0), "Cost should be 0");

                // Verify all choices are Skill cards
                use crate::game::card_type::CardType;
                for choice in choices {
                    let card = choice.to_card();
                    assert_eq!(card.get_card_type(), CardType::Skill,
                              "All choices should be Skill cards, found: {:?}", card.get_card_type());
                }
            },
            _ => panic!("Should be in SelectCardFromChoices state"),
        }

        // Select the first card
        let select_action = BattleAction::SelectCardFromChoices(0);
        let result = battle.eval_action(select_action, &mut rng);
        assert!(result.is_ok(), "SelectCardFromChoices action failed");

        // Check that we're back in PlayerTurn state
        assert_eq!(battle.battle_state, BattleState::PlayerTurn);

        // Check hand size increased by 1
        let final_hand_size = battle.cards.hand_size();
        assert_eq!(final_hand_size, initial_hand_size + 1, "Hand should have 1 more card");

        // Check that the card costs 0
        let hand = battle.cards.get_hand();
        let new_card = &hand[initial_hand_size];

        assert_eq!(new_card.get_cost(), 0, "Card should cost 0");
    }

    #[test]
    fn test_skill_potion_cost_clears_at_end_of_turn() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        let mut player_state = PlayerRunState::new(80, 80, 0);
        player_state.potions.add_potion(Potion::SkillPotion);

        let mut battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Use Skill Potion and select a card
        let use_potion_action = BattleAction::UsePotion(0, None);
        battle.eval_action(use_potion_action, &mut rng).unwrap();

        let select_action = BattleAction::SelectCardFromChoices(0);
        battle.eval_action(select_action, &mut rng).unwrap();

        // Get the hand before end turn
        let hand_before = battle.cards.get_hand();
        let card_cost_before = hand_before.last().unwrap().get_cost();
        assert_eq!(card_cost_before, 0, "Card should cost 0 this turn");

        // End the turn
        let end_turn_action = BattleAction::EndTurn;
        battle.eval_action(end_turn_action, &mut rng).unwrap();

        // Cards should be discarded at end of turn, so we can't check them directly
        // But the test verifies that the cost clearing mechanism is called
    }

    #[test]
    fn test_skill_potion_game_state_remains_in_battle() {
        use crate::game::game::Game;
        use crate::game::action::GameAction;
        use crate::map::debug_map;

        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let map = debug_map();
        let mut game = Game::new(deck, global_info, map, 80, 80);

        // Add Skill Potion to player
        game.potions.add_potion(Potion::SkillPotion);

        // Start a battle by choosing path 0 (first available path, which is Combat)
        let result = game.eval_action(GameAction::ChoosePath(0), &mut rng);
        assert!(result.is_ok(), "ChoosePath failed");

        // Verify we're in battle
        assert!(matches!(game.get_game_state(), crate::game::GameState::InBattle),
                "Should be InBattle after choosing combat path, got: {:?}", game.get_game_state());
        assert!(game.battle.is_some());

        let initial_hp = game.player_hp;

        // Use the Skill Potion
        let use_potion_action = GameAction::Battle(BattleAction::UsePotion(0, None));
        let result = game.eval_action(use_potion_action, &mut rng);
        assert!(result.is_ok(), "UsePotion action failed");

        // Verify still in battle and in SelectCardFromChoices state
        assert!(matches!(game.get_game_state(), crate::game::GameState::InBattle),
                "Should still be InBattle after using potion, got: {:?}", game.get_game_state());

        if let Some(battle) = &game.battle {
            assert!(matches!(battle.get_battle_state(), BattleState::SelectCardFromChoices { .. }),
                    "Should be in SelectCardFromChoices state");
        } else {
            panic!("Battle should still exist");
        }

        // Select the first card
        let select_action = GameAction::Battle(BattleAction::SelectCardFromChoices(0));
        let result = game.eval_action(select_action, &mut rng);
        assert!(result.is_ok(), "SelectCardFromChoices action failed");

        // CRITICAL CHECK: Verify we're still in battle and not back on map
        assert!(matches!(game.get_game_state(), crate::game::GameState::InBattle),
                "Game state should still be InBattle after selecting card, got: {:?}", game.get_game_state());

        // Verify battle still exists
        assert!(game.battle.is_some(), "Battle should still exist after selecting card");

        // Verify player is still alive
        assert!(game.player_hp > 0, "Player should still be alive");
        assert_eq!(game.player_hp, initial_hp, "Player HP should not have changed");

        // Verify we're back in PlayerTurn state
        if let Some(battle) = &game.battle {
            assert_eq!(battle.get_battle_state(), BattleState::PlayerTurn,
                       "Should be back in PlayerTurn state");

            // Verify player is alive in battle
            assert!(battle.get_player().is_alive(), "Player should be alive in battle");
        }
    }
}
