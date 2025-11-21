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
    use crate::battle::{Battle, target::Entity, enemy_in_battle::EnemyInBattle};
    
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};

    #[test]
    fn test_havoc_card_creation() {
        let havoc_card = havoc();
        assert_eq!(havoc_card.get_name(), "Havoc");
        assert_eq!(havoc_card.get_cost(), 1);
        assert_eq!(havoc_card.get_card_type(), &CardType::Skill);
        assert!(havoc_card.get_effects().contains(&Effect::PlayTopCardAndExhaust));
    }

    #[test]
    fn test_havoc_upgraded_card_creation() {
        let havoc_plus = havoc_upgraded();
        assert_eq!(havoc_plus.get_name(), "Havoc+");
        assert_eq!(havoc_plus.get_cost(), 0); // Cost should be 0
        assert_eq!(havoc_plus.get_card_type(), &CardType::Skill);
        assert!(havoc_plus.get_effects().contains(&Effect::PlayTopCardAndExhaust));
    }

    #[test]
    fn test_havoc_card_enum() {
        let havoc_card = havoc();
        let card_enum = havoc_card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::Havoc));
    }


    #[test]
    fn test_havoc_with_empty_deck() {
        // Create a battle with empty deck
        let empty_deck = Deck::new(vec![]);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new(empty_deck, global_info, 100, 100, vec![enemy], &mut rng);

        // Add Havoc to hand manually since there's no deck to draw from
        battle.add_card_to_hand_for_testing(havoc());

        // Try to play Havoc with empty deck
        let havoc_idx = 0;
        let result = battle.play_card(havoc_idx, Entity::Player);
        assert!(result.is_ok(), "Havoc should be playable even with empty deck");

        // Should handle gracefully without crashing
        assert!(battle.get_player().battle_info.is_alive(), "Player should still be alive");
    }

    #[test]
    fn test_havoc_goes_to_discard_not_exhausted() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create empty deck to avoid interference with top card
        let deck = Deck::new(vec![]);

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        // Add Havoc to hand
        battle.add_card_to_hand_for_testing(havoc());

        let initial_hand_size = battle.cards.hand_size();
        let initial_exhausted_size = battle.cards.exhausted_size();

        // Play Havoc with empty deck
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify Havoc went to discard pile (not exhausted)
        assert_eq!(battle.cards.hand_size(), initial_hand_size - 1); // Havoc removed from hand
        assert_eq!(battle.cards.discard_pile_size(), 1); // Havoc in discard pile
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted_size); // No cards exhausted

        // Verify Havoc is in discard pile
        let discard_cards = battle.cards.get_discard_pile();
        assert_eq!(discard_cards.len(), 1);
        assert_eq!(discard_cards[0].get_name(), "Havoc");
    }

    #[test]
    fn test_havoc_plays_card_effects() {
        use crate::cards::ironclad::strike::strike;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create deck with Strike on top
        let deck_cards = vec![strike()];
        let deck = Deck::new(deck_cards);

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let mut enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();

        // Add Havoc to hand
        battle.add_card_to_hand_for_testing(havoc());

        // Play Havoc
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify Strike's effect was applied (6 damage to enemy)
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 6);
    }

    #[test]
    fn test_havoc_upgraded_free_to_play() {
        use crate::cards::ironclad::strike::strike;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create deck with Strike on top
        let deck_cards = vec![strike()];
        let deck = Deck::new(deck_cards);

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        let initial_energy = battle.get_player().get_energy();

        // Add Havoc+ to hand
        battle.add_card_to_hand_for_testing(havoc_upgraded());

        // Play Havoc+
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify no energy was spent (Havoc+ costs 0)
        assert_eq!(battle.get_player().get_energy(), initial_energy);
    }
}