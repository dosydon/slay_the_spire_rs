use crate::game::{card::Card, effect::Effect, card_type::CardType, card_enum::CardEnum, card::Rarity};

/// Body Slam - Deal damage equal to your Block
pub fn body_slam() -> Card {
    Card::new(
        CardEnum::BodySlam,
        1,
        CardType::Attack,
        vec![Effect::AttackToTargetWithBlock],
        false, // not upgraded
        true,  // playable
        Rarity::Common,
    )
}

/// Body Slam+ (Upgraded version)
pub fn body_slam_upgraded() -> Card {
    Card::new(
        CardEnum::BodySlam,
        0, // Cost reduced from 1 to 0
        CardType::Attack,
        vec![Effect::AttackToTargetWithBlock],
        true,  // upgraded
        true,  // playable
        Rarity::Common,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, target::Entity, enemy_in_battle::EnemyInBattle};
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};
    
    #[test]
    fn test_body_slam_card_creation() {
        let body_slam_card = body_slam();
        assert_eq!(body_slam_card.get_name(), "Body Slam");
        assert_eq!(body_slam_card.get_cost(), 1);
        assert_eq!(body_slam_card.get_card_type(), &CardType::Attack);
        assert!(!body_slam_card.is_upgraded());

        let effects = body_slam_card.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::AttackToTargetWithBlock)));
    }

    #[test]
    fn test_body_slam_upgraded_card_creation() {
        let body_slam_plus = body_slam_upgraded();
        assert_eq!(body_slam_plus.get_name(), "Body Slam+");
        assert_eq!(body_slam_plus.get_cost(), 0);
        assert_eq!(body_slam_plus.get_card_type(), &CardType::Attack);
        assert!(body_slam_plus.is_upgraded());

        let effects = body_slam_plus.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::AttackToTargetWithBlock)));
    }

    #[test]
    fn test_body_slam_card_enum() {
        let body_slam_card = body_slam();
        let card_enum = body_slam_card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::BodySlam));
    }

    #[test]
    fn test_body_slam_zero_cost() {
        let body_slam_card = body_slam();
        assert_eq!(body_slam_card.get_cost(), 1, "Body Slam should cost 1 energy");

        let body_slam_plus = body_slam_upgraded();
        assert_eq!(body_slam_plus.get_cost(), 0, "Body Slam+ should cost 0 energy");
    }

    #[test]
    fn test_body_slam_damage_equals_block() {
        // Create a battle with a known enemy
        let deck_cards = vec![
            crate::cards::ironclad::strike::strike(),
            crate::cards::ironclad::defend::defend(),
            body_slam(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let player_state = crate::game::player_run_state::PlayerRunState::new(100, 100, 0);
let mut battle = Battle::new_with_shuffle(deck, global_info, player_state, vec![enemy], &mut rng);

        // Give the player some Block
        battle.get_player_mut().gain_block(15);
        let player_block = battle.get_player().get_block();
        assert_eq!(player_block, 15);

        // Find Body Slam in hand
        let body_slam_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Body Slam")
            .expect("Body Slam should be in hand");

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Play Body Slam
        let result = battle.play_card(body_slam_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Body Slam should be playable");

        // Check that enemy took damage equal to player's Block (15)
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let expected_hp = if initial_enemy_hp >= player_block {
            initial_enemy_hp - player_block
        } else {
            0
        };
        assert_eq!(final_enemy_hp, expected_hp,
                 "Enemy should have taken {} damage (equal to player's Block)", player_block);
    }

    #[test]
    fn test_body_slam_with_no_block() {
        // Create a battle with a known enemy
        let deck_cards = vec![
            crate::cards::ironclad::strike::strike(),
            body_slam(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let player_state = crate::game::player_run_state::PlayerRunState::new(100, 100, 0);
let mut battle = Battle::new_with_shuffle(deck, global_info, player_state, vec![enemy], &mut rng);

        // Ensure player has no Block
        let player_block = battle.get_player().get_block();
        assert_eq!(player_block, 0);

        // Find Body Slam in hand
        let body_slam_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Body Slam")
            .expect("Body Slam should be in hand");

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Play Body Slam
        let result = battle.play_card(body_slam_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Body Slam should be playable");

        // Check that enemy took 0 damage (player had 0 Block)
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp,
                 "Enemy should have taken 0 damage (player had 0 Block)");
    }

    #[test]
    fn test_body_slam_upgraded_zero_cost() {
        let deck_cards = vec![
            crate::cards::ironclad::strike::strike(),
            body_slam_upgraded(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let player_state = crate::game::player_run_state::PlayerRunState::new(100, 100, 0);
let mut battle = Battle::new_with_shuffle(deck, global_info, player_state, vec![enemy], &mut rng);

        // Give the player some Block
        battle.get_player_mut().gain_block(10);

        // Find Body Slam+ in hand
        let body_slam_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Body Slam+")
            .expect("Body Slam+ should be in hand");

        // Play Body Slam+
        let result = battle.play_card(body_slam_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Body Slam+ should be playable");

    }
}