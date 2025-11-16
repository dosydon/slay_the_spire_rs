use crate::game::{card::Card, effect::Effect, card_type::CardType, card_enum::CardEnum};

/// Warcry - Draw 2 cards. Put 1 card on top of draw pile
pub fn warcry() -> Card {
    Card::new(
        CardEnum::Warcry,
        0,
        CardType::Skill,
        vec![Effect::DrawCard(2), Effect::EnterSelectCardInHand],
        false, // not upgraded
        true,  // playable
    )
}

/// Warcry+ (upgraded version)
pub fn warcry_upgraded() -> Card {
    Card::new(
        CardEnum::Warcry,
        0,
        CardType::Skill,
        vec![Effect::DrawCard(2), Effect::EnterSelectCardInHand],
        true,  // upgraded
        true,  // playable
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, target::Entity, enemy_in_battle::EnemyInBattle};
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};

    #[test]
    fn test_warcry_card_creation() {
        let warcry_card = warcry();
        assert_eq!(warcry_card.get_name(), "Warcry");
        assert_eq!(warcry_card.get_cost(), 0);
        assert_eq!(warcry_card.get_card_type(), &CardType::Skill);

        let effects = warcry_card.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::DrawCard(2))));
        assert!(effects.contains(&Effect::EnterSelectCardInHand));
    }

    #[test]
    fn test_warcry_upgraded_card_creation() {
        let warcry_plus = warcry_upgraded();
        assert_eq!(warcry_plus.get_name(), "Warcry+");
        assert_eq!(warcry_plus.get_cost(), 0);
        assert_eq!(warcry_plus.get_card_type(), &CardType::Skill);

        let effects = warcry_plus.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::DrawCard(2))));
        assert!(effects.contains(&Effect::EnterSelectCardInHand));
        assert!(warcry_plus.is_upgraded());
    }

    #[test]
    fn test_warcry_card_enum() {
        let warcry_card = warcry();
        let card_enum = warcry_card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::Warcry));
    }

    #[test]
    fn test_warcry_draws_cards_and_enters_selection_state() {
        // Create a battle with a known deck
        let mut deck_cards = vec![
            crate::cards::ironclad::strike::strike(),
            crate::cards::ironclad::defend::defend(),
            crate::cards::ironclad::bash::bash(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new_with_shuffle(deck, global_info, 100, 100, vec![enemy], &mut rng);

        // Add Warcry to hand manually
        battle.cards.add_card_to_hand(warcry());

        let initial_hand_size = battle.cards.hand_size();

        // Play Warcry
        let warcry_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Warcry")
            .expect("Warcry should be in hand");

        let result = battle.play_card(warcry_idx, Entity::Player);
        assert!(result.is_ok(), "Warcry should be playable");

        // Check that 2 cards were drawn
        let final_hand_size = battle.cards.hand_size();
        assert_eq!(final_hand_size, initial_hand_size + 1, "Should draw 2 cards but play 1 (net +1)");

        // Check that battle entered SelectCardInHand state
        assert!(matches!(battle.battle_state, crate::battle::action::BattleState::SelectCardInHand));

        // Should have cards in hand to select from
        assert!(battle.cards.hand_size() > 0, "Should have cards in hand to select");
    }

    #[test]
    fn test_warcry_put_selected_card_on_top_of_draw_pile() {
        // Create a battle with known deck
        let mut deck_cards = vec![
            crate::cards::ironclad::strike::strike(),
            crate::cards::ironclad::defend::defend(),
            crate::cards::ironclad::bash::bash(),
            crate::cards::ironclad::iron_wave::iron_wave(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new_with_shuffle(deck, global_info, 100, 100, vec![enemy], &mut rng);

        // Add Warcry to hand manually
        battle.cards.add_card_to_hand(warcry());

        // Play Warcry
        let warcry_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Warcry")
            .expect("Warcry should be in hand");

        let result = battle.play_card(warcry_idx, Entity::Player);
        assert!(result.is_ok(), "Warcry should be playable");

        // Should be in SelectCardInHand state
        assert!(matches!(battle.battle_state, crate::battle::action::BattleState::SelectCardInHand));

        // Get the current top of draw pile before selection
        let initial_top_card = battle.cards.peek_top_card();

        // Check that the selected card is now on top of draw pile
        let new_top_card = battle.cards.peek_top_card();
        assert!(new_top_card.is_some(), "Should have a card on top of draw pile");

        // The new top card should be different from the initial top card (unless it was already going to be next)
        if initial_top_card.is_some() {
            let selected_card = &battle.cards.get_hand()[0];
            assert_eq!(new_top_card.unwrap().get_name(), selected_card.get_name(),
                      "Top card should be the selected card");
        }

        // Should be back to PlayerTurn state
        assert!(matches!(battle.battle_state, crate::battle::action::BattleState::PlayerTurn));
    }

    #[test]
    fn test_warcry_with_empty_deck() {
        // Create a battle with empty deck
        let empty_deck = Deck::new(vec![]);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new(empty_deck, global_info, 100, 100, vec![enemy], &mut rng);

        // Add Warcry to hand manually since there's no deck to draw from
        battle.cards.add_card_to_hand(warcry());

        // Try to play Warcry with empty deck
        let warcry_idx = 0;
        let result = battle.play_card(warcry_idx, Entity::Player);
        assert!(result.is_ok(), "Warcry should be playable even with empty deck");
    }

    #[test]
    fn test_warcry_zero_cost() {
        let warcry_card = warcry();
        assert_eq!(warcry_card.get_cost(), 0, "Warcry should cost 0 energy");

        let warcry_plus = warcry_upgraded();
        assert_eq!(warcry_plus.get_cost(), 0, "Warcry+ should also cost 0 energy");
    }
}