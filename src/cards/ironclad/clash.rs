use crate::game::{card::{Card, Rarity}, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// Clash - Deal 14 damage. Can only be played if hand is all Attacks
pub fn clash() -> Card {
    Card::new_with_condition(
        CardEnum::Clash,
        0,
        CardType::Attack,
        vec![Effect::AttackToTarget { amount: 14, num_attacks: 1, strength_multiplier: 1 }],
        false, // not upgraded
        Condition::HandAllAttacks,
        Rarity::Common)
}

/// Clash+ (upgraded version)
pub fn clash_upgraded() -> Card {
    Card::new_with_condition(
        CardEnum::Clash,
        0,
        CardType::Attack,
        vec![Effect::AttackToTarget { amount: 18, num_attacks: 1, strength_multiplier: 1 }],
        true,  // upgraded
        Condition::HandAllAttacks,
        Rarity::Common)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, target::Entity, enemy_in_battle::EnemyInBattle};
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};

    #[test]
    fn test_clash_card_creation() {
        let clash_card = clash();
        assert_eq!(clash_card.get_name(), "Clash");
        assert_eq!(clash_card.get_cost(), 0);
        assert_eq!(clash_card.get_card_type(), &CardType::Attack);
        assert!(!clash_card.is_upgraded());

        let effects = clash_card.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::AttackToTarget { amount: 14, .. })));
    }

    #[test]
    fn test_clash_upgraded_card_creation() {
        let clash_plus = clash_upgraded();
        assert_eq!(clash_plus.get_name(), "Clash+");
        assert_eq!(clash_plus.get_cost(), 0);
        assert_eq!(clash_plus.get_card_type(), &CardType::Attack);
        assert!(clash_plus.is_upgraded());

        let effects = clash_plus.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::AttackToTarget { amount: 18, .. })));
    }

    #[test]
    fn test_clash_card_enum() {
        let clash_card = clash();
        let card_enum = clash_card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::Clash));
    }

    #[test]
    fn test_clash_playable_with_all_attacks() {
        // Create a battle with only attack cards in hand
        let deck_cards = vec![
            clash(),
            crate::cards::ironclad::strike::strike(),
            crate::cards::ironclad::strike::strike(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let player_state = crate::game::player_run_state::PlayerRunState::new(100, 100, 0);
    let mut battle = Battle::new_with_shuffle(deck, global_info, player_state, vec![enemy], &mut rng);

        // Find Clash in hand
        let clash_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Clash")
            .expect("Clash should be in hand");

        // Should be able to play Clash when all cards in hand are attacks
        let result = battle.play_card(clash_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Clash should be playable when all cards in hand are attacks");
    }

    #[test]
    fn test_clash_not_playable_with_non_attacks() {
        // Create a battle with non-attack cards in hand
        let deck_cards = vec![
            clash(),
            crate::cards::ironclad::defend::defend(), // This is a Skill, not Attack
            crate::cards::ironclad::strike::strike(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let player_state = crate::game::player_run_state::PlayerRunState::new(100, 100, 0);
    let mut battle = Battle::new_with_shuffle(deck, global_info, player_state, vec![enemy], &mut rng);

        // Find Clash in hand
        let clash_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Clash")
            .expect("Clash should be in hand");

        // Should NOT be able to play Clash when hand contains non-attack cards
        let result = battle.play_card(clash_idx, Entity::Enemy(0));
        assert!(result.is_err(), "Clash should not be playable when hand contains non-attack cards");
    }

    #[test]
    fn test_clash_damage() {
        // Create a battle with only attack cards
        let deck_cards = vec![
            clash(),
            crate::cards::ironclad::strike::strike(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let player_state = crate::game::player_run_state::PlayerRunState::new(100, 100, 0);
    let mut battle = Battle::new_with_shuffle(deck, global_info, player_state, vec![enemy], &mut rng);

        // Find Clash in hand
        let clash_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Clash")
            .expect("Clash should be in hand");

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Play Clash
        let result = battle.play_card(clash_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Clash should be playable");

        // Check that enemy took 14 damage
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let expected_hp = if initial_enemy_hp >= 14 {
            initial_enemy_hp - 14
        } else {
            0
        };
        assert_eq!(final_enemy_hp, expected_hp,
                 "Enemy should have taken 14 damage");
    }

    #[test]
    fn test_clash_upgraded_damage() {
        // Create a battle with only attack cards
        let deck_cards = vec![
            clash_upgraded(),
            crate::cards::ironclad::strike::strike(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let player_state = crate::game::player_run_state::PlayerRunState::new(100, 100, 0);
    let mut battle = Battle::new_with_shuffle(deck, global_info, player_state, vec![enemy], &mut rng);

        // Find Clash+ in hand
        let clash_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Clash+")
            .expect("Clash+ should be in hand");

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Play Clash+
        let result = battle.play_card(clash_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Clash+ should be playable");

        // Check that enemy took 18 damage
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let expected_hp = if initial_enemy_hp >= 18 {
            initial_enemy_hp - 18
        } else {
            0
        };
        assert_eq!(final_enemy_hp, expected_hp,
                 "Enemy should have taken 18 damage from Clash+");
    }

    #[test]
    fn test_clash_zero_cost() {
        let clash_card = clash();
        assert_eq!(clash_card.get_cost(), 0, "Clash should cost 0 energy");

        let clash_plus = clash_upgraded();
        assert_eq!(clash_plus.get_cost(), 0, "Clash+ should also cost 0 energy");
    }
}