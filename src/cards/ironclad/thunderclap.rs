use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

pub fn thunderclap() -> Card {
    Card::new(CardEnum::Thunderclap, 1, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        BattleEffect::AttackAllEnemies { amount: 4, num_attacks: 1 },
        BattleEffect::ApplyVulnerableAll { duration: 1 }
    ])
}

pub fn thunderclap_upgraded() -> Card {
    Card::new(CardEnum::Thunderclap, 1, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        BattleEffect::AttackAllEnemies { amount: 7, num_attacks: 1 }, // +2 damage
        BattleEffect::ApplyVulnerableAll { duration: 1 } //
    ])
        .set_upgraded(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, battle_action::BattleAction, target::Entity};
    use crate::game::global_info::GlobalInfo;
    use crate::game::deck::Deck;
    use crate::game::enemy::EnemyTrait;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};

    #[test]
    fn test_thunderclap_applies_vulnerable_to_all_enemies() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create multiple enemies to test ApplyVulnerableAll
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse3 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse3)),
        ];

        // Create a deck with thunderclap
        let deck = Deck::new(vec![thunderclap()]);
        let player_state = crate::game::player_run_state::PlayerRunState::new(80, 80, 0);
let mut battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Draw the thunderclap card into hand
        battle.at_start_of_player_turn(&mut rng);

        // Get initial vulnerable status (should be 0 for all enemies)
        let initial_vulnerable: Vec<u32> = battle.get_enemies()
            .iter()
            .map(|enemy| enemy.battle_info.get_vulnerable_turns())
            .collect();

        assert!(initial_vulnerable.iter().all(|&v| v == 0),
                "All enemies should start with 0 vulnerable, got: {:?}", initial_vulnerable);

        // Play thunderclap (target doesn't matter for ApplyVulnerableAll)
        let action = BattleAction::PlayCard(0, Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok(), "Thunderclap should play successfully");

        // Check that ALL enemies now have Vulnerable(1)
        let final_vulnerable: Vec<u32> = battle.get_enemies()
            .iter()
            .map(|enemy| enemy.battle_info.get_vulnerable_turns())
            .collect();

        assert!(final_vulnerable.iter().all(|&v| v == 1),
                "All enemies should have Vulnerable(1) after thunderclap, got: {:?}", final_vulnerable);


        // Create a fresh battle to test damage
        let deck2 = Deck::new(vec![thunderclap()]);
        let red_louse1_new = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2_new = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse3_new = RedLouse::instantiate(&mut rng, &global_info);
        let enemies2 = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1_new)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2_new)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse3_new)),
        ];
        let player_state = crate::game::player_run_state::PlayerRunState::new(80, 80, 0);
let mut battle2 = Battle::new(deck2, global_info, player_state, enemies2, &mut rng);
        battle2.at_start_of_player_turn(&mut rng);

        let hp_before: Vec<u32> = battle2.get_enemies()
            .iter()
            .map(|enemy| enemy.battle_info.get_hp())
            .collect();

        let action2 = BattleAction::PlayCard(0, Entity::Enemy(0));
        battle2.eval_action(action2, &mut rng).unwrap();

        let hp_after: Vec<u32> = battle2.get_enemies()
            .iter()
            .map(|enemy| enemy.battle_info.get_hp())
            .collect();

        assert!(hp_after.iter().zip(hp_before.iter()).all(|(&after, &before)| after < before),
                "All enemies should have taken damage from thunderclap");
    }

    #[test]
    fn test_thunderclap_upgraded_applies_vulnerable_to_all_enemies() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create multiple enemies to test ApplyVulnerableAll
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2)),
        ];

        // Create a deck with upgraded thunderclap
        let deck = Deck::new(vec![thunderclap_upgraded()]);
        let player_state = crate::game::player_run_state::PlayerRunState::new(80, 80, 0);
let mut battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Draw the thunderclap+ card into hand
        battle.at_start_of_player_turn(&mut rng);

        // Get initial vulnerable status (should be 0 for all enemies)
        let initial_vulnerable: Vec<u32> = battle.get_enemies()
            .iter()
            .map(|enemy| enemy.battle_info.get_vulnerable_turns())
            .collect();

        assert!(initial_vulnerable.iter().all(|&v| v == 0),
                "All enemies should start with 0 vulnerable, got: {:?}", initial_vulnerable);

        // Play thunderclap+ (target doesn't matter for ApplyVulnerableAll)
        let action = BattleAction::PlayCard(0, Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok(), "Thunderclap+ should play successfully");

        // Check that ALL enemies now have Vulnerable(1)
        let final_vulnerable: Vec<u32> = battle.get_enemies()
            .iter()
            .map(|enemy| enemy.battle_info.get_vulnerable_turns())
            .collect();

        assert!(final_vulnerable.iter().all(|&v| v == 1),
                "All enemies should have Vulnerable(1) after thunderclap+, got: {:?}", final_vulnerable);
    }
}