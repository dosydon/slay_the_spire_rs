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
}