use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

pub fn cleave() -> Card {
    Card::new(CardEnum::Cleave, 1, CardType::Attack, vec![
        Effect::AttackAllEnemies { amount: 8, num_attacks: 1 }
    ], Rarity::Common)
}

pub fn cleave_upgraded() -> Card {
    Card::new(CardEnum::Cleave, 1, CardType::Attack, vec![
        Effect::AttackAllEnemies { amount: 11, num_attacks: 1 }
    ], Rarity::Common)
        .set_upgraded(true)
}

#[cfg(test)]
mod tests {
    use crate::game::PlayerRunState;

    use super::*;

    #[test]
    fn test_cleave_basic() {
        let card = cleave();
        assert_eq!(card.get_name(), "Cleave");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackAllEnemies { amount, num_attacks } => {
                assert_eq!(*amount, 8);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackAllEnemies effect"),
        }
    }

    #[test]
    fn test_cleave_upgraded() {
        let card = cleave_upgraded();
        assert_eq!(card.get_name(), "Cleave+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackAllEnemies { amount, num_attacks } => {
                assert_eq!(*amount, 11); // +3 damage over regular Cleave
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackAllEnemies effect"),
        }
    }

    #[test]
    fn test_cleave_battle_integration() {
        use crate::battle::{Battle, enemy_in_battle::EnemyInBattle, battle_action::BattleAction, target::Entity};
        use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
        use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};

        // Create a deck with Cleave and some other cards
        let mut deck_cards = vec![cleave()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        let deck = Deck::new(deck_cards);
        
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create battle with multiple enemies
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse3 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse3))
        ];
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), enemies, &mut rng);
        
        // Draw hand (manually add to hand for testing)
        battle.at_start_of_player_turn(&mut rng);
        
        // Record initial HP of all enemies
        let initial_enemy1_hp = battle.get_enemies()[0].battle_info.get_hp();
        let initial_enemy2_hp = battle.get_enemies()[1].battle_info.get_hp();
        let initial_enemy3_hp = battle.get_enemies()[2].battle_info.get_hp();
        let initial_player_energy = battle.get_player().get_energy();
        
        // Find Cleave card in hand
        let hand = battle.get_hand();
        let cleave_idx = hand.iter().position(|card| card.get_name() == "Cleave");
        assert!(cleave_idx.is_some(), "Cleave card should be in hand");
        
        // Play Cleave targeting Entity::None (hits all enemies)
        let action = BattleAction::PlayCard(cleave_idx.unwrap(), Entity::None);
        let result = battle.eval_action(action, &mut rng);
        assert!(matches!(result, Ok(_)), "Playing Cleave should succeed");
        
        // Verify effects:
        // 1. All three enemies should take 8 damage
        assert_eq!(battle.get_enemies()[0].battle_info.get_hp(), initial_enemy1_hp - 8);
        assert_eq!(battle.get_enemies()[1].battle_info.get_hp(), initial_enemy2_hp - 8);
        assert_eq!(battle.get_enemies()[2].battle_info.get_hp(), initial_enemy3_hp - 8);
        
        // 2. Player should have spent 1 energy
        assert_eq!(battle.get_player().get_energy(), initial_player_energy - 1);
        
        // 3. Cleave card should no longer be in hand
        let hand_after = battle.get_hand();
        let cleave_still_in_hand = hand_after.iter().any(|card| card.get_name() == "Cleave");
        assert!(!cleave_still_in_hand, "Cleave should be removed from hand after playing");
        
        // 4. Verify that enemies with 0 HP are considered dead
        // (Red Louse has 10-15 HP, so 8 damage won't kill them, but let's test the principle)
        assert!(battle.get_enemies()[0].battle_info.is_alive(), "Enemy 1 should still be alive");
        assert!(battle.get_enemies()[1].battle_info.is_alive(), "Enemy 2 should still be alive");
        assert!(battle.get_enemies()[2].battle_info.is_alive(), "Enemy 3 should still be alive");
    }

    #[test]
    fn test_cleave_with_strength() {
        use crate::battle::{Battle, enemy_in_battle::EnemyInBattle, battle_action::BattleAction, target::Entity};
        use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
        use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};

        let mut deck_cards = vec![cleave()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        let deck = Deck::new(deck_cards);
        
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create battle with two enemies
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2))
        ];
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), enemies, &mut rng);
        
        // Give player some strength using the effect system
        let strength_effect = crate::game::effect::BaseEffect::GainStrength { source: Entity::Player, amount: 3 };
        battle.eval_base_effect(&strength_effect);
        assert_eq!(battle.get_player().battle_info.get_strength(), 3);
        
        battle.at_start_of_player_turn(&mut rng);
        
        let initial_enemy1_hp = battle.get_enemies()[0].battle_info.get_hp();
        let initial_enemy2_hp = battle.get_enemies()[1].battle_info.get_hp();
        
        // Find and play Cleave
        let hand = battle.get_hand();
        let cleave_idx = hand.iter().position(|card| card.get_name() == "Cleave");
        assert!(cleave_idx.is_some(), "Cleave card should be in hand");
        
        let action = BattleAction::PlayCard(cleave_idx.unwrap(), Entity::None);
        let result = battle.eval_action(action, &mut rng);
        assert!(matches!(result, Ok(_)));
        
        // Both enemies should take 8 + 3 = 11 damage due to strength
        let expected_enemy1_hp = initial_enemy1_hp.saturating_sub(11);
        let expected_enemy2_hp = initial_enemy2_hp.saturating_sub(11);
        assert_eq!(battle.get_enemies()[0].battle_info.get_hp(), expected_enemy1_hp);
        assert_eq!(battle.get_enemies()[1].battle_info.get_hp(), expected_enemy2_hp);
    }

    #[test]
    fn test_cleave_with_vulnerable_enemies() {
        use crate::battle::{Battle, enemy_in_battle::EnemyInBattle, battle_action::BattleAction, target::Entity};
        use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
        use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait, effect::BaseEffect};

        let mut deck_cards = vec![cleave()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        let deck = Deck::new(deck_cards);
        
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create battle with two enemies
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2))
        ];
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), enemies, &mut rng);
        
        // Make both enemies vulnerable
        let vulnerable_effect1 = BaseEffect::ApplyVulnerable { target: Entity::Enemy(0), duration: 2 };
        let vulnerable_effect2 = BaseEffect::ApplyVulnerable { target: Entity::Enemy(1), duration: 2 };
        battle.eval_base_effect(&vulnerable_effect1);
        battle.eval_base_effect(&vulnerable_effect2);
        
        assert!(battle.get_enemies()[0].battle_info.is_vulnerable());
        assert!(battle.get_enemies()[1].battle_info.is_vulnerable());
        
        battle.at_start_of_player_turn(&mut rng);
        
        let initial_enemy1_hp = battle.get_enemies()[0].battle_info.get_hp();
        let initial_enemy2_hp = battle.get_enemies()[1].battle_info.get_hp();
        
        // Find and play Cleave
        let hand = battle.get_hand();
        let cleave_idx = hand.iter().position(|card| card.get_name() == "Cleave");
        assert!(cleave_idx.is_some(), "Cleave card should be in hand");
        
        let action = BattleAction::PlayCard(cleave_idx.unwrap(), Entity::None);
        let result = battle.eval_action(action, &mut rng);
        assert!(matches!(result, Ok(_)));
        
        // Both enemies should take 8 * 1.5 = 12 damage due to vulnerable
        let expected_enemy1_hp = initial_enemy1_hp.saturating_sub(12);
        let expected_enemy2_hp = initial_enemy2_hp.saturating_sub(12);
        assert_eq!(battle.get_enemies()[0].battle_info.get_hp(), expected_enemy1_hp);
        assert_eq!(battle.get_enemies()[1].battle_info.get_hp(), expected_enemy2_hp);
    }

    #[test]
    fn test_cleave_skips_dead_enemies() {
        use crate::battle::{Battle, enemy_in_battle::EnemyInBattle, battle_action::BattleAction, target::Entity};
        use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
        use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};

        let mut deck_cards = vec![cleave()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        let deck = Deck::new(deck_cards);
        
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create battle with three enemies
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse3 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse3))
        ];
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), enemies, &mut rng);
        
        // Kill the middle enemy
        let middle_enemy_hp = battle.get_enemies()[1].battle_info.get_hp();
        let kill_effect = crate::game::effect::BaseEffect::AttackToTarget {
            source: Entity::None,
            target: Entity::Enemy(1),
            amount: middle_enemy_hp + 10,
            num_attacks: 1,
            strength_multiplier: 1
        };
        battle.eval_base_effect(&kill_effect);
        assert!(!battle.get_enemies()[1].battle_info.is_alive());
        
        battle.at_start_of_player_turn(&mut rng);
        
        let initial_enemy1_hp = battle.get_enemies()[0].battle_info.get_hp();
        let initial_enemy3_hp = battle.get_enemies()[2].battle_info.get_hp();
        let dead_enemy_hp = battle.get_enemies()[1].battle_info.get_hp();
        
        // Find and play Cleave
        let hand = battle.get_hand();
        let cleave_idx = hand.iter().position(|card| card.get_name() == "Cleave");
        assert!(cleave_idx.is_some(), "Cleave card should be in hand");
        
        let action = BattleAction::PlayCard(cleave_idx.unwrap(), Entity::None);
        let result = battle.eval_action(action, &mut rng);
        assert!(matches!(result, Ok(_)));
        
        // Only living enemies should take damage
        assert_eq!(battle.get_enemies()[0].battle_info.get_hp(), initial_enemy1_hp - 8);
        assert_eq!(battle.get_enemies()[1].battle_info.get_hp(), dead_enemy_hp); // No change
        assert_eq!(battle.get_enemies()[2].battle_info.get_hp(), initial_enemy3_hp - 8);
        
        // Dead enemy should remain dead
        assert!(!battle.get_enemies()[1].battle_info.is_alive());
    }
}