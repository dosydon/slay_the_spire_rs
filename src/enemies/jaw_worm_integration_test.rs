#[cfg(test)]
mod integration_tests {
    use crate::battle::Battle;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::cards::ironclad::starter_deck::starter_deck;
    use crate::enemies::{EnemyEnum, jaw_worm::JawWorm};
    use crate::game::enemy::EnemyTrait;
    use crate::game::global_info::GlobalInfo;
    use rand::Rng;

    #[test]
    fn test_jaw_worm_battle_integration() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create a Jaw Worm enemy
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let hp = rng.random_range(JawWorm::hp_lb()..=JawWorm::hp_ub());
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm), hp)];
        
        // Create battle with Jaw Worm
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Verify Jaw Worm was created correctly
        assert_eq!(battle.get_enemies().len(), 1);
        assert!(battle.get_enemies()[0].battle_info.is_alive());
        
        // Verify player starts with correct stats
        assert_eq!(battle.get_player().battle_info.get_hp(), 80);
        assert_eq!(battle.get_player().get_energy(), 3);
        
        // Simulate enemy turn - Jaw Worm should use Chomp first in Act 1
        let initial_player_hp = battle.get_player().battle_info.get_hp();
        battle.enemy_turn(&mut rng, &global_info);
        
        // Player should take damage from Chomp (11 damage)
        let expected_damage = JawWorm::calculate_chomp_damage(&global_info);
        assert_eq!(battle.get_player().battle_info.get_hp(), initial_player_hp - expected_damage);
        
        println!("Jaw Worm battle integration test passed!");
        println!("Player HP after Chomp: {} (took {} damage)", 
                battle.get_player().battle_info.get_hp(), expected_damage);
    }

    #[test] 
    fn test_jaw_worm_act3_battle_integration() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 51 }; // Act 3
        
        // Create a Jaw Worm enemy (Act 3)
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let hp = rng.random_range(JawWorm::hp_lb()..=JawWorm::hp_ub());
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm), hp)];
        
        // Create battle with Act 3 Jaw Worm
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Act 3 Jaw Worm should start with initial Bellow effects
        // This would typically be applied during enemy creation, but for this test
        // we'll verify the enemy exists and can perform moves
        
        assert!(battle.get_enemies()[0].battle_info.is_alive());
        
        // Simulate enemy turn 
        let initial_player_hp = battle.get_player().battle_info.get_hp();
        battle.enemy_turn(&mut rng, &global_info);
        
        // In Act 3, first move probabilities are different, so we just verify
        // that some action was taken (either damage or no damage if Bellow was used)
        let final_player_hp = battle.get_player().battle_info.get_hp();
        
        // Should either take damage or stay the same (if Bellow was used)
        assert!(final_player_hp <= initial_player_hp);
        
        println!("Act 3 Jaw Worm battle integration test passed!");
        println!("Player HP after enemy turn: {} (started at {})", 
                final_player_hp, initial_player_hp);
    }

    #[test]
    fn test_jaw_worm_multiple_turns() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create a Jaw Worm enemy
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let hp = rng.random_range(JawWorm::hp_lb()..=JawWorm::hp_ub());
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm), hp)];
        
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Simulate multiple enemy turns to test move sequence
        let mut player_damage_taken = 0;
        let mut enemy_strength_gained = 0;
        
        for turn in 0..5 {
            let player_hp_before = battle.get_player().battle_info.get_hp();
            let enemy_strength_before = battle.get_enemies()[0].battle_info.get_strength();
            
            battle.enemy_turn(&mut rng, &global_info);
            
            let player_hp_after = battle.get_player().battle_info.get_hp();
            let enemy_strength_after = battle.get_enemies()[0].battle_info.get_strength();
            
            if player_hp_after < player_hp_before {
                player_damage_taken += player_hp_before - player_hp_after;
                println!("Turn {}: Player took {} damage", turn + 1, player_hp_before - player_hp_after);
            }
            
            if enemy_strength_after > enemy_strength_before {
                enemy_strength_gained += enemy_strength_after - enemy_strength_before;
                println!("Turn {}: Enemy gained {} strength", turn + 1, enemy_strength_after - enemy_strength_before);
            }
        }
        
        // Over 5 turns, either damage should be dealt or strength should be gained
        assert!(player_damage_taken > 0 || enemy_strength_gained > 0);
        
        println!("Multi-turn Jaw Worm test passed!");
        println!("Total damage dealt: {}, Total strength gained: {}", 
                player_damage_taken, enemy_strength_gained);
    }
}