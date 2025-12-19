use crate::game::{card::{Card, Rarity}, effect::Effect, card_type::CardType, card_enum::CardEnum};

/// Headbutt - Deal 9 damage. Put a card from discard pile on top of draw pile
pub fn headbutt() -> Card {
    Card::new(
        CardEnum::Headbutt,
        1,
        CardType::Attack,
        vec![Effect::AttackToTarget { amount: 9, num_attacks: 1, strength_multiplier: 0 }, Effect::EnterSelectCardInDiscard],
        false, // not upgraded
        true,  // playable
        Rarity::Uncommon)
}

/// Headbutt+ (upgraded version)
pub fn headbutt_upgraded() -> Card {
    Card::new(
        CardEnum::Headbutt,
        1,
        CardType::Attack,
        vec![Effect::AttackToTarget { amount: 12, num_attacks: 1, strength_multiplier: 0 }, Effect::EnterSelectCardInDiscard],
        true,  // upgraded
        true,  // playable
        Rarity::Uncommon)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, target::Entity, enemy_in_battle::EnemyInBattle};
    use crate::cards::ironclad::starter_deck::starter_deck;
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, enemy::EnemyTrait};

    #[test]
    fn test_headbutt_card_creation() {
        let headbutt_card = headbutt();
        assert_eq!(headbutt_card.get_name(), "Headbutt");
        assert_eq!(headbutt_card.get_cost(), 1);
        assert_eq!(headbutt_card.get_card_type(), &CardType::Attack);

        let effects = headbutt_card.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::AttackToTarget { amount: 9, .. })));
        assert!(effects.contains(&Effect::EnterSelectCardInDiscard));
    }

    #[test]
    fn test_headbutt_upgraded_card_creation() {
        let headbutt_plus = headbutt_upgraded();
        assert_eq!(headbutt_plus.get_name(), "Headbutt+");
        assert_eq!(headbutt_plus.get_cost(), 1);
        assert_eq!(headbutt_plus.get_card_type(), &CardType::Attack);

        let effects = headbutt_plus.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::AttackToTarget { amount: 12, .. })));
        assert!(effects.contains(&Effect::EnterSelectCardInDiscard));
        assert!(headbutt_plus.is_upgraded());
    }

    #[test]
    fn test_headbutt_card_enum() {
        let headbutt_card = headbutt();
        let card_enum = headbutt_card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::Headbutt));
    }

    #[test]
    fn test_headbutt_deals_damage_and_enters_selection_state() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new_with_shuffle(deck, global_info, 100, 100, vec![enemy], &mut rng);

        // Draw initial hand and then play some cards to create discard pile
        battle.at_start_of_player_turn(&mut rng);

        // Play a card to create discard pile
        let initial_hand_size = battle.cards.hand_size();
        if initial_hand_size > 0 {
            let _ = battle.play_card(0, Entity::Enemy(0));
        }

        // Add Headbutt to hand manually
        battle.add_card_to_hand_for_testing(headbutt());

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let initial_discard_size = battle.cards.discard_pile_size();

        // Play Headbutt
        let headbutt_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Headbutt")
            .expect("Headbutt should be in hand");

        let result = battle.play_card(headbutt_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Headbutt should be playable");

        // Check that damage was dealt
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert!(final_enemy_hp < initial_enemy_hp, "Enemy should have taken damage");

        // Check that battle entered SelectCardInDiscard state
        assert!(matches!(battle.battle_state, crate::battle::action::BattleState::SelectCardInDiscard));

        // Should have cards in discard to select from
        if initial_discard_size > 0 {
            assert!(battle.cards.discard_pile_size() > 0, "Should have cards in discard to select");
        }
    }
}