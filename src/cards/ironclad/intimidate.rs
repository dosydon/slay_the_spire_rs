use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// Intimidate - Apply 1 Weak to all enemies. Exhaust.
pub fn intimidate() -> Card {
    Card::new_with_condition(
        CardEnum::Intimidate,
        0,
        CardType::Skill,
        vec![Effect::ApplyWeakAll { duration: 1 }, Effect::Exhaust],
        false, // not upgraded
        Condition::True,
    )
}

/// Intimidate+ (Upgraded version) - Apply 2 Weak to all enemies
pub fn intimidate_upgraded() -> Card {
    Card::new_with_condition(
        CardEnum::Intimidate,
        0,
        CardType::Skill,
        vec![Effect::ApplyWeakAll { duration: 2 }, Effect::Exhaust],
        true,  // upgraded
        Condition::True,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, target::Entity, enemy_in_battle::EnemyInBattle};
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};

    #[test]
    fn test_intimidate_card_creation() {
        let intimidate_card = intimidate();
        assert_eq!(intimidate_card.get_name(), "Intimidate");
        assert_eq!(intimidate_card.get_cost(), 0);
        assert_eq!(intimidate_card.get_card_type(), &CardType::Skill);
        assert!(!intimidate_card.is_upgraded());

        let effects = intimidate_card.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::ApplyWeakAll { duration: 1 })));
        assert!(effects.iter().any(|e| matches!(e, Effect::Exhaust)));
    }

    #[test]
    fn test_intimidate_upgraded_card_creation() {
        let intimidate_plus = intimidate_upgraded();
        assert_eq!(intimidate_plus.get_name(), "Intimidate+");
        assert_eq!(intimidate_plus.get_cost(), 0);
        assert_eq!(intimidate_plus.get_card_type(), &CardType::Skill);
        assert!(intimidate_plus.is_upgraded());

        let effects = intimidate_plus.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::ApplyWeakAll { duration: 2 })));
        assert!(effects.iter().any(|e| matches!(e, Effect::Exhaust)));
    }

    #[test]
    fn test_intimidate_card_enum() {
        let intimidate_card = intimidate();
        let card_enum = intimidate_card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::Intimidate));
    }

    #[test]
    fn test_intimidate_applies_weak_to_all_enemies() {
        // Create a battle with multiple enemies
        let deck_cards = vec![
            intimidate(),
            crate::cards::ironclad::strike::strike(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2)),
        ];

        let mut battle = Battle::new_with_shuffle(deck, global_info, 100, 100, enemies, &mut rng);

        // Verify no enemies are weak initially
        assert!(!battle.get_enemies()[0].battle_info.is_weak());
        assert!(!battle.get_enemies()[1].battle_info.is_weak());

        // Find Intimidate in hand
        let intimidate_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Intimidate")
            .expect("Intimidate should be in hand");

        // Play Intimidate
        let result = battle.play_card(intimidate_idx, Entity::Player);
        assert!(result.is_ok(), "Intimidate should be playable");

        // Verify all enemies are weak
        assert!(battle.get_enemies()[0].battle_info.is_weak());
        assert!(battle.get_enemies()[1].battle_info.is_weak());

        // Check that the card was exhausted
        let exhausted_cards = battle.cards.get_exhausted();
        assert_eq!(exhausted_cards.last().unwrap().get_name(), "Intimidate");
    }

    #[test]
    fn test_intimidate_upgraded_applies_more_weak() {
        // Create a battle with Intimidate+
        let deck_cards = vec![
            intimidate_upgraded(),
            crate::cards::ironclad::strike::strike(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new_with_shuffle(deck, global_info, 100, 100, vec![enemy], &mut rng);

        // Verify enemy is not weak initially
        assert!(!battle.get_enemies()[0].battle_info.is_weak());

        // Find Intimidate+ in hand
        let intimidate_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Intimidate+")
            .expect("Intimidate+ should be in hand");

        // Play Intimidate+
        let result = battle.play_card(intimidate_idx, Entity::Player);
        assert!(result.is_ok(), "Intimidate+ should be playable");

        // Verify enemy has weak (upgraded applies 2 stacks)
        assert!(battle.get_enemies()[0].battle_info.is_weak());

        // Check that the card was exhausted
        let exhausted_cards = battle.cards.get_exhausted();
        assert_eq!(exhausted_cards.last().unwrap().get_name(), "Intimidate+");
    }

    #[test]
    fn test_intimidate_zero_cost() {
        let intimidate_card = intimidate();
        assert_eq!(intimidate_card.get_cost(), 0, "Intimidate should cost 0 energy");

        let intimidate_plus = intimidate_upgraded();
        assert_eq!(intimidate_plus.get_cost(), 0, "Intimidate+ should also cost 0 energy");
    }
}

