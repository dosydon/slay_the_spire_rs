use crate::game::{card::{Card, Rarity}, card_type::CardType, card_enum::CardEnum};

pub fn wound() -> Card {
    Card::new(CardEnum::Wound, 0, CardType::Status, vec![
        // Wound is a pure status card with no effects
        // It's unplayable and should be automatically exhausted
    ], false, false, Rarity::Basic)
}

// Wound cannot be upgraded in the base game

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, action::Action, target::Entity, BattleError};
    use crate::game::enemy::EnemyTrait;
    use crate::game::global_info::GlobalInfo;
    use crate::game::deck::Deck;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};

    #[test]
    fn test_wound_returns_error_when_played() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create a deck with just a wound card
        let deck = Deck::new(vec![wound()]);
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        // Draw the wound card into hand
        battle.at_start_of_player_turn(&mut rng);

        // Try to play the wound card - it should be in hand at index 0
        let action = Action::PlayCard(0, Entity::Player);
        let result = battle.eval_action(action, &mut rng);

        // The action should fail with CardNotPlayable error
        assert!(matches!(result, Err(BattleError::CardNotPlayable)),
                "Playing wound card should return CardNotPlayable error, got: {:?}", result);

        // Verify that wound card is still unplayable
        let wound_card = &battle.cards.get_hand()[0];
        assert!(!wound_card.is_playable(), "Wound card should be marked as unplayable");

        // Verify that no actions are available for wound card
        let available_actions = battle.list_available_actions();
        let wound_actions: Vec<_> = available_actions.iter()
            .filter(|action| {
                if let Action::PlayCard(idx, _) = action {
                    *idx == 0  // wound card index
                } else {
                    false
                }
            })
            .collect();

        assert!(wound_actions.is_empty(), "No actions should be available for unplayable wound card");
    }
}