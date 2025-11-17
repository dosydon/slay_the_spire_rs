use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// Anger - Deal 6 damage. Add a copy of this card to your discard pile.
pub fn anger() -> Card {
    Card::new_with_condition(
        CardEnum::Anger,
        0,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 },
            Effect::AddCardToDiscard(CardEnum::Anger),
        ],
        false, // not upgraded
        Condition::True,
    )
}

/// Anger+ (Upgraded version) - Deal 8 damage. Add a copy of this card to your discard pile.
pub fn anger_upgraded() -> Card {
    Card::new_with_condition(
        CardEnum::Anger,
        0,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: 8, num_attacks: 1, strength_multiplier: 1 },
            Effect::AddCardToDiscard(CardEnum::Anger),
        ],
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
    fn test_anger_card_creation() {
        let anger_card = anger();
        assert_eq!(anger_card.get_name(), "Anger");
        assert_eq!(anger_card.get_cost(), 0);
        assert_eq!(anger_card.get_card_type(), &CardType::Attack);
        assert!(!anger_card.is_upgraded());

        let effects = anger_card.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::AttackToTarget { amount: 6, .. })));
        assert!(effects.iter().any(|e| matches!(e, Effect::AddCardToDiscard(CardEnum::Anger))));
    }

    #[test]
    fn test_anger_upgraded_card_creation() {
        let anger_plus = anger_upgraded();
        assert_eq!(anger_plus.get_name(), "Anger+");
        assert_eq!(anger_plus.get_cost(), 0);
        assert_eq!(anger_plus.get_card_type(), &CardType::Attack);
        assert!(anger_plus.is_upgraded());

        let effects = anger_plus.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::AttackToTarget { amount: 8, .. })));
        assert!(effects.iter().any(|e| matches!(e, Effect::AddCardToDiscard(CardEnum::Anger))));
    }

    #[test]
    fn test_anger_card_enum() {
        let anger_card = anger();
        let card_enum = anger_card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::Anger));
    }

    #[test]
    fn test_anger_deals_damage() {
        // Create a battle with Anger
        let deck_cards = vec![
            anger(),
            crate::cards::ironclad::strike::strike(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new_with_shuffle(deck, global_info, 100, 100, vec![enemy], &mut rng);

        // Find Anger in hand
        let anger_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Anger")
            .expect("Anger should be in hand");

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Play Anger
        let result = battle.play_card(anger_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Anger should be playable");

        // Check that enemy took 6 damage
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let expected_hp = if initial_enemy_hp >= 6 {
            initial_enemy_hp - 6
        } else {
            0
        };
        assert_eq!(final_enemy_hp, expected_hp,
                 "Enemy should have taken 6 damage");
    }

    #[test]
    fn test_anger_adds_to_discard() {
        // Create a battle with Anger
        let deck_cards = vec![
            anger(),
            crate::cards::ironclad::strike::strike(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new_with_shuffle(deck, global_info, 100, 100, vec![enemy], &mut rng);

        // Find Anger in hand
        let anger_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Anger")
            .expect("Anger should be in hand");

        let initial_discard_size = battle.cards.discard_pile_size();

        // Play Anger
        let result = battle.play_card(anger_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Anger should be playable");

        // Check that a copy was added to discard pile
        assert_eq!(battle.cards.discard_pile_size(), initial_discard_size + 1,
                  "Should have added a copy of Anger to discard pile");

        // Check that the added card is Anger
        let discard_cards = battle.cards.get_discard_pile();
        assert_eq!(discard_cards.last().unwrap().get_name(), "Anger");
    }

    #[test]
    fn test_anger_upgraded_damage() {
        // Create a battle with Anger+
        let deck_cards = vec![
            anger_upgraded(),
            crate::cards::ironclad::strike::strike(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new_with_shuffle(deck, global_info, 100, 100, vec![enemy], &mut rng);

        // Find Anger+ in hand
        let anger_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Anger+")
            .expect("Anger+ should be in hand");

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Play Anger+
        let result = battle.play_card(anger_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Anger+ should be playable");

        // Check that enemy took 8 damage
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let expected_hp = if initial_enemy_hp >= 8 {
            initial_enemy_hp - 8
        } else {
            0
        };
        assert_eq!(final_enemy_hp, expected_hp,
                 "Enemy should have taken 8 damage from Anger+");
    }

    #[test]
    fn test_anger_zero_cost() {
        let anger_card = anger();
        assert_eq!(anger_card.get_cost(), 0, "Anger should cost 0 energy");

        let anger_plus = anger_upgraded();
        assert_eq!(anger_plus.get_cost(), 0, "Anger+ should also cost 0 energy");
    }
}