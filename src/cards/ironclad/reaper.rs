use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

pub fn reaper() -> Card {
    Card::new(
        CardEnum::Reaper,
        2,
        CardType::Attack,
        vec![
            Effect::AttackAllEnemiesAndHeal { amount: 4, num_attacks: 1 },
        ],
        false, // not upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

pub fn reaper_upgraded() -> Card {
    Card::new(
        CardEnum::Reaper,
        2,
        CardType::Attack,
        vec![
            Effect::AttackAllEnemiesAndHeal { amount: 5, num_attacks: 1 },
        ],
        true,  // upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reaper_creation() {
        let card = reaper();
        assert_eq!(card.get_name(), "Reaper");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert!(!card.is_upgraded());
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_playable());
    }

    #[test]
    fn test_reaper_upgraded_creation() {
        let card = reaper_upgraded();
        assert_eq!(card.get_name(), "Reaper+");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert!(card.is_upgraded());
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_playable());
    }

    #[test]
    fn test_reaper_effects() {
        let card = reaper();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackAllEnemiesAndHeal { amount, num_attacks } => {
                assert_eq!(*amount, 4);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackAllEnemiesAndHeal effect"),
        }
    }

    #[test]
    fn test_reaper_upgraded_effects() {
        let card = reaper_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackAllEnemiesAndHeal { amount, num_attacks } => {
                assert_eq!(*amount, 5);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackAllEnemiesAndHeal effect"),
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::battle::Battle;
    use crate::battle::target::Entity;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::game::deck::Deck;
    use crate::game::global_info::GlobalInfo;
    use crate::game::enemy::EnemyTrait;
    use crate::enemies::jaw_worm::JawWorm;
    use crate::enemies::red_louse::RedLouse;
    use crate::enemies::enemy_enum::EnemyEnum;

    #[test]
    fn test_reaper_attacks_all_enemies_and_heals() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create two enemies
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse)),
        ];

        let deck = Deck::new(vec![reaper()]);
        let mut battle = Battle::new(deck, global_info, 40, 80, enemies, &mut rng); // Start with 40 HP

        let initial_player_hp = battle.get_player().get_current_hp();
        let initial_enemy1_hp = battle.get_enemies()[0].get_current_hp();
        let initial_enemy2_hp = battle.get_enemies()[1].get_current_hp();

        // Play Reaper
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify both enemies took damage (4 damage each)
        let final_enemy1_hp = battle.get_enemies()[0].get_current_hp();
        let final_enemy2_hp = battle.get_enemies()[1].get_current_hp();
        assert_eq!(final_enemy1_hp, initial_enemy1_hp - 4);
        assert_eq!(final_enemy2_hp, initial_enemy2_hp - 4);

        // Verify player healed for damage dealt (4 damage to 2 enemies = 8 total healing)
        let final_player_hp = battle.get_player().get_current_hp();
        assert_eq!(final_player_hp, initial_player_hp + 8);
    }

    #[test]
    fn test_reaper_upgraded_higher_damage_and_healing() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse)),
        ];

        let deck = Deck::new(vec![reaper_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 40, 80, enemies, &mut rng);

        let initial_player_hp = battle.get_player().get_current_hp();
        let initial_enemy1_hp = battle.get_enemies()[0].get_current_hp();
        let initial_enemy2_hp = battle.get_enemies()[1].get_current_hp();

        // Play Reaper+
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify both enemies took 5 damage each
        let final_enemy1_hp = battle.get_enemies()[0].get_current_hp();
        let final_enemy2_hp = battle.get_enemies()[1].get_current_hp();
        assert_eq!(final_enemy1_hp, initial_enemy1_hp - 5);
        assert_eq!(final_enemy2_hp, initial_enemy2_hp - 5);

        // Verify player healed for 10 total (5 damage to 2 enemies)
        let final_player_hp = battle.get_player().get_current_hp();
        assert_eq!(final_player_hp, initial_player_hp + 10);
    }

    #[test]
    fn test_reaper_healing_capped_at_max_hp() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![reaper()]);
        let mut battle = Battle::new(deck, global_info, 79, 80, enemies, &mut rng); // 1 HP below max

        let initial_player_hp = battle.get_player().get_current_hp();
        assert_eq!(initial_player_hp, 79);

        // Play Reaper (would heal for 4, but capped at max HP)
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify healing is capped at max HP
        let final_player_hp = battle.get_player().get_current_hp();
        assert_eq!(final_player_hp, 80); // Capped at max HP
    }

    #[test]
    fn test_reaper_costs_two_energy() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![reaper()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        let initial_energy = battle.get_player().get_energy();

        // Play Reaper
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify energy was consumed (costs 2)
        assert_eq!(battle.get_player().get_energy(), initial_energy - 2);
    }

    #[test]
    fn test_reaper_is_attack_card() {
        let card = reaper();
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_cost(), 2);
        assert!(!card.is_upgraded());
    }

    #[test]
    fn test_reaper_upgraded_is_attack_card() {
        let card = reaper_upgraded();
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_cost(), 2);
        assert!(card.is_upgraded());
    }

    #[test]
    fn test_reaper_single_enemy() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![reaper()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        let initial_player_hp = battle.get_player().get_current_hp();
        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();

        // Play Reaper against single enemy
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify enemy took 4 damage
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 4);

        // Verify player healed for 4 HP
        let final_player_hp = battle.get_player().get_current_hp();
        assert_eq!(final_player_hp, initial_player_hp + 4);
    }

    #[test]
    fn test_reaper_heals_only_for_unblocked_damage() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let mut enemy = EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm));

        // Give the enemy 10 block to test damage blocking
        enemy.battle_info.gain_block(10);
        let enemies = vec![enemy];

        let deck = Deck::new(vec![reaper()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        let initial_player_hp = battle.get_player().get_current_hp();
        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();
        let initial_enemy_block = battle.get_enemies()[0].battle_info.get_block();

        // Play Reaper against enemy with block
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify enemy block was reduced but HP unchanged (all 4 damage was blocked)
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        let final_enemy_block = battle.get_enemies()[0].battle_info.get_block();
        assert_eq!(final_enemy_hp, initial_enemy_hp); // No HP damage
        assert_eq!(final_enemy_block, initial_enemy_block - 4); // Block reduced by 4

        // Verify player healed for 0 HP (no unblocked damage dealt)
        let final_player_hp = battle.get_player().get_current_hp();
        assert_eq!(final_player_hp, initial_player_hp); // No healing since all damage was blocked
    }

    #[test]
    fn test_reaper_heals_for_partial_unblocked_damage() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let mut enemy = EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm));

        // Give the enemy 2 block to test partial damage blocking
        enemy.battle_info.gain_block(2);
        let enemies = vec![enemy];

        let deck = Deck::new(vec![reaper()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        let initial_player_hp = battle.get_player().get_current_hp();
        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();
        let initial_enemy_block = battle.get_enemies()[0].battle_info.get_block();

        // Play Reaper against enemy with partial block
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify enemy took 2 HP damage (4 damage - 2 block)
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        let final_enemy_block = battle.get_enemies()[0].battle_info.get_block();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 2); // 2 HP damage
        assert_eq!(final_enemy_block, 0); // All block consumed

        // Verify player healed for 2 HP (only unblocked damage)
        let final_player_hp = battle.get_player().get_current_hp();
        assert_eq!(final_player_hp, initial_player_hp + 2); // Healed for unblocked damage only
    }
}