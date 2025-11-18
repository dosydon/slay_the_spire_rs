use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// Whirlwind - Uncommon Attack Card
/// Cost: 1 (represents X-cost - actual cost is energy spent)
/// Effect: Deal 5 damage to ALL enemies, X times (where X is the energy spent)
pub fn whirlwind() -> Card {
    Card::new_with_condition(
        CardEnum::Whirlwind,
        1, // Base cost 1 (represents X-cost mechanics)
        CardType::Attack,
        vec![
            Effect::AttackAllEnemies { amount: 5, num_attacks: 1 },
        ],
        false, // not upgraded
        Condition::True,
    )
}

/// Whirlwind+ (Upgraded version)
/// Cost: 1 (represents X-cost - actual cost is energy spent)
/// Effect: Deal 8 damage to ALL enemies, X times (where X is the energy spent)
pub fn whirlwind_upgraded() -> Card {
    Card::new_with_condition(
        CardEnum::Whirlwind,
        1, // Base cost 1 (represents X-cost mechanics)
        CardType::Attack,
        vec![
            Effect::AttackAllEnemies { amount: 8, num_attacks: 1 },
        ],
        true,  // upgraded
        Condition::True,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whirlwind_creation() {
        let card = whirlwind();

        assert_eq!(card.get_name(), "Whirlwind");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_whirlwind_upgraded_creation() {
        let card = whirlwind_upgraded();

        assert_eq!(card.get_name(), "Whirlwind+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_whirlwind_effects() {
        let card = whirlwind();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackAllEnemies { amount, num_attacks } => {
                assert_eq!(*amount, 5);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackAllEnemies effect"),
        }
    }

    #[test]
    fn test_whirlwind_upgraded_effects() {
        let card = whirlwind_upgraded();
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
    fn test_whirlwind_cost_stays_same() {
        let base_card = whirlwind();
        let upgraded_card = whirlwind_upgraded();

        assert_eq!(base_card.get_cost(), 1, "Whirlwind should cost 1 energy (X-cost)");
        assert_eq!(upgraded_card.get_cost(), 1, "Whirlwind+ should also cost 1 energy (X-cost)");
    }

    #[test]
    fn test_whirlwind_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2)),
        ];

        // Create battle with Whirlwind in hand
        let deck = Deck::new(vec![whirlwind()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Check initial enemy HP
        let initial_hp1 = battle.get_enemies()[0].battle_info.get_hp();
        let initial_hp2 = battle.get_enemies()[1].battle_info.get_hp();

        // Give player enough energy to play Whirlwind
        battle.get_player_mut().battle_info.energy = 1;

        // Play Whirlwind
        let whirlwind_idx = 0;
        let result = battle.play_card(whirlwind_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify damage was dealt to ALL enemies
        let final_hp1 = battle.get_enemies()[0].battle_info.get_hp();
        let final_hp2 = battle.get_enemies()[1].battle_info.get_hp();
        assert_eq!(final_hp1, initial_hp1 - 5);
        assert_eq!(final_hp2, initial_hp2 - 5);
    }

    #[test]
    fn test_whirlwind_upgraded_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2)),
        ];

        // Create battle with Whirlwind+ in hand
        let deck = Deck::new(vec![whirlwind_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Check initial enemy HP
        let initial_hp1 = battle.get_enemies()[0].battle_info.get_hp();
        let initial_hp2 = battle.get_enemies()[1].battle_info.get_hp();

        // Give player enough energy to play Whirlwind+
        battle.get_player_mut().battle_info.energy = 1;

        // Play Whirlwind+
        let whirlwind_idx = 0;
        let result = battle.play_card(whirlwind_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify increased damage was dealt to ALL enemies
        let final_hp1 = battle.get_enemies()[0].battle_info.get_hp();
        let final_hp2 = battle.get_enemies()[1].battle_info.get_hp();
        assert_eq!(final_hp1, initial_hp1 - 8);
        assert_eq!(final_hp2, initial_hp2 - 8);
    }

    #[test]
    fn test_whirlwind_single_enemy() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Whirlwind in hand (single enemy)
        let deck = Deck::new(vec![whirlwind()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Check initial enemy HP
        let initial_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Play Whirlwind
        let whirlwind_idx = 0;
        let result = battle.play_card(whirlwind_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify damage was dealt to the single enemy
        let final_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(final_hp, initial_hp - 5);
    }

    #[test]
    fn test_whirlwind_energy_cost() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Whirlwind in hand
        let deck = Deck::new(vec![whirlwind()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player exactly 1 energy
        battle.get_player_mut().battle_info.energy = 1;
        let initial_energy = battle.get_player().get_energy();

        // Play Whirlwind
        let whirlwind_idx = 0;
        let result = battle.play_card(whirlwind_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify energy was spent
        let final_energy = battle.get_player().get_energy();
        assert_eq!(final_energy, initial_energy - 1);
    }

    #[test]
    fn test_whirlwind_insufficient_energy() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Whirlwind in hand
        let deck = Deck::new(vec![whirlwind()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player 0 energy
        battle.get_player_mut().battle_info.energy = 0;

        // Try to play Whirlwind - should fail due to insufficient energy
        let whirlwind_idx = 0;
        let result = battle.play_card(whirlwind_idx, Entity::Enemy(0));
        assert!(result.is_err()); // Should return an error
    }
}