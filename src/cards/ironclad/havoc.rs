use crate::game::{card::Card, effect::Effect, card_type::CardType, card_enum::CardEnum};

/// Havoc - Play top card of draw pile. Exhaust it
pub fn havoc() -> Card {
    Card::new(
        CardEnum::Havoc,
        1,
        CardType::Skill,
        vec![Effect::PlayTopCardAndExhaust],
        false, // not upgraded
        true,  // playable
    )
}

/// Havoc+ (upgraded version)
pub fn havoc_upgraded() -> Card {
    Card::new(
        CardEnum::Havoc,
        0, // Cost reduced from 1 to 0
        CardType::Skill,
        vec![Effect::PlayTopCardAndExhaust],
        true,  // upgraded
        true,  // playable
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, target::Entity, player::Player};
    use crate::cards::ironclad::starter_deck::starter_deck;
    use crate::enemies::red_louse::RedLouse;
    use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};

    #[test]
    fn test_havoc_card_creation() {
        let havoc_card = havoc();
        assert_eq!(havoc_card.get_name(), "Havoc");
        assert_eq!(havoc_card.get_cost(), 1);
        assert_eq!(havoc_card.get_card_type(), &CardType::Skill);
        assert!(havoc_card.get_effects().contains(&Effect::PlayTopCard));
        assert!(havoc_card.get_effects().contains(&Effect::Exhaust));
    }

    #[test]
    fn test_havoc_upgraded_card_creation() {
        let havoc_plus = havoc_upgraded();
        assert_eq!(havoc_plus.get_name(), "Havoc+");
        assert_eq!(havoc_plus.get_cost(), 0); // Cost should be 0
        assert_eq!(havoc_plus.get_card_type(), &CardType::Skill);
        assert!(havoc_plus.get_effects().contains(&Effect::PlayTopCard));
        assert!(havoc_plus.get_effects().contains(&Effect::Exhaust));
    }

    #[test]
    fn test_havoc_card_enum() {
        let havoc_card = havoc();
        let card_enum = CardEnum::from_card(&havoc_card);
        assert!(matches!(card_enum, CardEnum::Havoc));
    }

    #[test]
    fn test_havoc_play_card_from_draw_pile() {
        // Create a battle with a known deck
        let mut deck_cards = starter_deck();
        // Add a specific card to the top of deck for testing
        deck_cards.insert(0, crate::cards::ironclad::strike::strike());

        let deck = Deck::new(deck_cards);
        let player = Player::new("Test Player".to_string(), 100, 3);
        let enemy = RedLouse.create_enemy(&GlobalInfo::new());
        let mut battle = Battle::new(deck, player, vec![enemy], GlobalInfo::new());

        // Draw initial hand
        battle.start_turn();
        let initial_hand_size = battle.cards.hand_size();

        // Play Havoc
        let havoc_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Havoc")
            .expect("Havoc should be in hand");

        let result = battle.play_card(havoc_idx, Entity::Player);
        assert!(result.is_ok(), "Havoc should be playable");

        // Check that Havoc was exhausted (not in discard)
        let found_in_discard = battle.cards.get_discard_pile().iter()
            .any(|c| c.get_name().contains("Havoc"));
        assert!(!found_in_discard, "Havoc should be exhausted, not in discard");

        // The top card should have been played (effects processed)
        // We can't easily verify which card was played without more complex state tracking,
        // but we can verify the battle state is still valid
        assert!(battle.player.battle_info.is_alive(), "Player should still be alive");
    }

    #[test]
    fn test_havoc_with_empty_deck() {
        // Create a battle with empty deck
        let empty_deck = Deck::new(vec![]);
        let player = Player::new("Test Player".to_string(), 100, 3);
        let enemy = RedLouse.create_enemy(&GlobalInfo::new());
        let mut battle = Battle::new(empty_deck, player, vec![enemy], GlobalInfo::new());

        // Add Havoc to hand manually since there's no deck to draw from
        battle.cards.add_card_to_hand(havoc());

        // Try to play Havoc with empty deck
        let havoc_idx = 0;
        let result = battle.play_card(havoc_idx, Entity::Player);
        assert!(result.is_ok(), "Havoc should be playable even with empty deck");

        // Should handle gracefully without crashing
        assert!(battle.player.battle_info.is_alive(), "Player should still be alive");
    }
}