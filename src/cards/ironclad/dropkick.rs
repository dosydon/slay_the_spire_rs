use crate::game::{card::{Card, Rarity}, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// Dropkick - Deal 5 damage. If enemy Vulnerable: gain 1 Energy, draw 1 card
pub fn dropkick() -> Card {
    Card::new(
        CardEnum::Dropkick,
        1,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: 5, num_attacks: 1, strength_multiplier: 0 },
            Effect::ConditionalEffect(
                Condition::TargetIsVulnerable,
                Box::new(Effect::GainEnergy { amount: 1 })
            ),
            Effect::ConditionalEffect(
                Condition::TargetIsVulnerable,
                Box::new(Effect::DrawCard { count: 1 })
            ),
        ],
        Rarity::Uncommon)
}

/// Dropkick+ (upgraded version)
pub fn dropkick_upgraded() -> Card {
    Card::new(
        CardEnum::Dropkick,
        1,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: 8, num_attacks: 1, strength_multiplier: 0 },
            Effect::ConditionalEffect(
                Condition::TargetIsVulnerable,
                Box::new(Effect::GainEnergy { amount: 1 })
            ),
            Effect::ConditionalEffect(
                Condition::TargetIsVulnerable,
                Box::new(Effect::DrawCard { count: 1 })
            ),
        ],
        Rarity::Uncommon)
        .set_upgraded(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, target::Entity, enemy_in_battle::EnemyInBattle};
    use crate::cards::ironclad::starter_deck::starter_deck;
    use crate::cards::ironclad::strike;
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::PlayerRunState;
    use crate::game::deck::Deck;
    use crate::game::{global_info::GlobalInfo, enemy::EnemyTrait};

    #[test]
    fn test_dropkick_card_creation() {
        let dropkick_card = dropkick();
        assert_eq!(dropkick_card.get_name(), "Dropkick");
        assert_eq!(dropkick_card.get_cost(), 1);
        assert_eq!(dropkick_card.get_card_type(), &CardType::Attack);

        let effects = dropkick_card.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::AttackToTarget { amount: 5, num_attacks: 1, strength_multiplier: 0 })));
    }

    #[test]
    fn test_dropkick_upgraded_card_creation() {
        let dropkick_plus = dropkick_upgraded();
        assert_eq!(dropkick_plus.get_name(), "Dropkick+");
        assert_eq!(dropkick_plus.get_cost(), 1);
        assert_eq!(dropkick_plus.get_card_type(), &CardType::Attack);

        let effects = dropkick_plus.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::AttackToTarget { amount: 8, num_attacks: 1, strength_multiplier: 0 })));
        assert!(dropkick_plus.is_upgraded());
    }

    #[test]
    fn test_dropkick_card_enum() {
        let dropkick_card = dropkick();
        let card_enum = dropkick_card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::Dropkick));
    }

    #[test]
    fn test_dropkick_deals_damage() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let player_state = crate::game::player_run_state::PlayerRunState::new(100, 100, 0);
let mut battle = Battle::new_with_shuffle(deck, global_info, player_state, vec![enemy], &mut rng);

        // Draw initial hand
        battle.at_start_of_player_turn(&mut rng);

        // Add Dropkick to hand manually
        battle.add_card_to_hand_for_testing(dropkick());

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let initial_player_energy = battle.get_player().get_energy();
        let initial_hand_size = battle.cards.hand_size();

        // Play Dropkick on non-vulnerable enemy
        let dropkick_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Dropkick")
            .expect("Dropkick should be in hand");

        let result = battle.play_card(dropkick_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Dropkick should be playable");

        // Check that damage was dealt
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 5, "Enemy should take exactly 5 damage");

        // Check that energy was spent but no bonus energy gained (enemy not vulnerable)
        assert_eq!(battle.get_player().get_energy(), initial_player_energy - 1, "Should spend 1 energy");

        // Check that no card was drawn (enemy not vulnerable)
        assert_eq!(battle.cards.hand_size(), initial_hand_size - 1, "Should not draw card when enemy not vulnerable");
    }

    #[test]
    fn test_dropkick_upgraded_deals_more_damage() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let player_state = crate::game::player_run_state::PlayerRunState::new(100, 100, 0);
let mut battle = Battle::new_with_shuffle(deck, global_info, player_state, vec![enemy], &mut rng);

        // Draw initial hand
        battle.at_start_of_player_turn(&mut rng);

        // Add Dropkick+ to hand manually
        battle.add_card_to_hand_for_testing(dropkick_upgraded());

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Play Dropkick+ on non-vulnerable enemy
        let dropkick_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Dropkick+")
            .expect("Dropkick+ should be in hand");

        let result = battle.play_card(dropkick_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Dropkick+ should be playable");

        // Check that increased damage was dealt
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 8, "Enemy should take exactly 8 damage for upgraded version");
    }

    #[test]
    fn test_dropkick_with_vulnerable_enemy() {
        let deck = Deck::new(
            vec![
                dropkick(),
                strike(),
                strike(),
                strike(),
                strike(),
                strike(),
                strike(),
            ]
        );
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(100, 100, 0), vec![enemy], &mut rng);

        // Make enemy vulnerable first (manually apply vulnerable status)
        // Note: This would normally be done through game effects, but for testing we'll simulate it
        battle.get_enemies_mut()[0].battle_info.apply_vulnerable(1);

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let initial_player_energy = battle.get_player().get_energy();
        let initial_hand_size = battle.cards.hand_size();

        // Play Dropkick on vulnerable enemy
        let dropkick_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Dropkick")
            .expect("Dropkick should be in hand");

        let result = battle.play_card(dropkick_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Dropkick should be playable");

        // Check that damage was dealt
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 7, "Enemy should take exactly 7 damage");

        // Check that energy was spent but bonus energy was also gained (net 0 change)
        assert_eq!(battle.get_player().get_energy(), initial_player_energy, "Should spend 1 energy but gain 1 bonus energy");

        // Check that a card was drawn (enemy was vulnerable)
        assert_eq!(battle.cards.hand_size(), initial_hand_size, "Should draw 1 card when enemy is vulnerable (net 0 change after playing Dropkick)");
    }

    #[test]
    fn test_dropkick_cost_one() {
        let dropkick_card = dropkick();
        assert_eq!(dropkick_card.get_cost(), 1, "Dropkick should cost 1 energy");

        let dropkick_plus = dropkick_upgraded();
        assert_eq!(dropkick_plus.get_cost(), 1, "Dropkick+ should also cost 1 energy");
    }
}