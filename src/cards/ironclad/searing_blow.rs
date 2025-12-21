use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

/// Calculate Searing Blow damage using quadratic progression
/// Formula: damage = n(n+7)/2 + 12 where n is upgrade level
/// Level 0 (base): 12 damage
/// Level 1: 16 damage (+4)
/// Level 2: 21 damage (+5)
/// Level 3: 27 damage (+6)
/// Level 4: 34 damage (+7)
/// Level n: n(n+7)/2 + 12
fn calculate_searing_blow_damage(upgrade_level: u32) -> u32 {
    if upgrade_level == 0 {
        12
    } else {
        // Using the quadratic formula: n(n+7)/2 + 12
        (upgrade_level * (upgrade_level + 7)) / 2 + 12
    }
}

/// Create a Searing Blow card at a specific upgrade level
/// This allows for the infinite upgrade system that Searing Blow uses
fn searing_blow_with_level(upgrade_level: u32) -> Card {
    let damage = calculate_searing_blow_damage(upgrade_level);

    Card::new_with_upgrade_level(
        CardEnum::SearingBlow,
        2,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 },
        ],
        upgrade_level,
        true,  // playable
        Rarity::Uncommon,
    )
}

/// Searing Blow - Uncommon Attack Card (Level 0)
/// Cost: 2
/// Effect: Deal 12 damage. Can be upgraded infinitely outside of combat
pub fn searing_blow() -> Card {
    searing_blow_with_level(0)
}

/// Searing Blow+ - Standard upgraded version (Level 1)
/// Cost: 2
/// Effect: Deal 16 damage. Can be upgraded infinitely outside of combat
pub fn searing_blow_upgraded() -> Card {
    searing_blow_with_level(1)
}

/// Create Searing Blow at a specific upgrade level for upgrade systems
/// This function allows external upgrade systems to create properly scaled Searing Blow cards
pub fn searing_blow_at_level(upgrade_level: u32) -> Card {
    searing_blow_with_level(upgrade_level)
}

/// Upgrade a Searing Blow card to the next level
/// Returns a new card with one higher upgrade level
pub fn upgrade_searing_blow_to_next_level(current_card: Card) -> Card {
    let current_level = current_card.get_upgrade_level();
    searing_blow_at_level(current_level + 1)
}

/// Upgrade a Searing Blow card multiple levels
/// Returns a new card with the specified number of additional upgrades
pub fn upgrade_searing_blow_multiple_levels(current_card: Card, additional_upgrades: u32) -> Card {
    let current_level = current_card.get_upgrade_level();
    searing_blow_at_level(current_level + additional_upgrades)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, target::Entity};
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::game::PlayerRunState;
    use crate::game::deck::Deck;
    use crate::game::global_info::GlobalInfo;
    use crate::game::enemy::EnemyTrait;
    use crate::enemies::red_louse::RedLouse;
    use crate::enemies::enemy_enum::EnemyEnum;

    #[test]
    fn test_searing_blow_creation() {
        let card = searing_blow();
        assert_eq!(card.get_name(), "Searing Blow");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_searing_blow_upgraded_creation() {
        let card = searing_blow_upgraded();
        assert_eq!(card.get_name(), "Searing Blow+");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_searing_blow_effects() {
        let card = searing_blow();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 12);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_searing_blow_upgraded_effects() {
        let card = searing_blow_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 16);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_searing_blow_battle_integration() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let deck = Deck::new(vec![searing_blow()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), vec![enemy], &mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();

        // Play Searing Blow
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify damage dealt (damage should not go below 0)
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp.saturating_sub(12));
    }

    #[test]
    fn test_searing_blow_upgraded_battle_integration() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let deck = Deck::new(vec![searing_blow_upgraded()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), vec![enemy], &mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();

        // Play Searing Blow+
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify increased damage dealt
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp.saturating_sub(16));
    }

    #[test]
    fn test_searing_blow_costs_energy() {
        let normal_card = searing_blow();
        let upgraded_card = searing_blow_upgraded();

        assert_eq!(normal_card.get_cost(), 2, "Searing Blow should cost 2 energy");
        assert_eq!(upgraded_card.get_cost(), 2, "Searing Blow+ should also cost 2 energy");
    }

    #[test]
    fn test_searing_blow_card_enum() {
        let card = searing_blow();
        let card_enum = card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::SearingBlow));
    }

    #[test]
    fn test_searing_blow_upgrade_scaling() {
        // Test base damage values
        let base_card = searing_blow();
        let upgraded_card = searing_blow_upgraded();

        // Base version deals 12 damage
        assert_eq!(base_card.get_effects()[0],
                  Effect::AttackToTarget { amount: 12, num_attacks: 1, strength_multiplier: 1 });

        // Upgraded version deals 16 damage (4 more damage)
        assert_eq!(upgraded_card.get_effects()[0],
                  Effect::AttackToTarget { amount: 16, num_attacks: 1, strength_multiplier: 1 });

        // Note: Searing Blow can be upgraded multiple times outside of combat
        // This would be handled by the card upgrade system, not during gameplay
        // Future upgrades would continue to increase the damage by 4 each time
    }

    #[test]
    fn test_searing_blow_quadratic_damage() {
        // Test the quadratic damage progression matches the provided table
        assert_eq!(calculate_searing_blow_damage(0), 12);  // Base
        assert_eq!(calculate_searing_blow_damage(1), 16);  // +4
        assert_eq!(calculate_searing_blow_damage(2), 21);  // +5
        assert_eq!(calculate_searing_blow_damage(3), 27);  // +6
        assert_eq!(calculate_searing_blow_damage(4), 34);  // +7
        assert_eq!(calculate_searing_blow_damage(5), 42);  // +8
        assert_eq!(calculate_searing_blow_damage(10), 97); // +13
    }

    #[test]
    fn test_searing_blow_at_level() {
        // Test creating cards at different upgrade levels
        let level_0 = searing_blow_at_level(0);
        let level_2 = searing_blow_at_level(2);
        let level_5 = searing_blow_at_level(5);

        assert_eq!(level_0.get_effects()[0],
                  Effect::AttackToTarget { amount: 12, num_attacks: 1, strength_multiplier: 1 });
        assert_eq!(level_0.get_upgrade_level(), 0);
        assert!(!level_0.is_upgraded());

        assert_eq!(level_2.get_effects()[0],
                  Effect::AttackToTarget { amount: 21, num_attacks: 1, strength_multiplier: 1 });
        assert_eq!(level_2.get_upgrade_level(), 2);
        assert!(level_2.is_upgraded());

        assert_eq!(level_5.get_effects()[0],
                  Effect::AttackToTarget { amount: 42, num_attacks: 1, strength_multiplier: 1 });
        assert_eq!(level_5.get_upgrade_level(), 5);
        assert!(level_5.is_upgraded());
    }

    #[test]
    fn test_searing_blow_upgrade_to_next_level() {
        // Test upgrading a card to the next level
        let level_0 = searing_blow();
        let level_1 = upgrade_searing_blow_to_next_level(level_0);
        let level_2 = upgrade_searing_blow_to_next_level(level_1.clone()); 

        assert_eq!(level_1.get_upgrade_level(), 1);
        assert_eq!(level_2.get_upgrade_level(), 2);

        // Verify damage progression
        assert_eq!(level_1.get_effects()[0],
                  Effect::AttackToTarget { amount: 16, num_attacks: 1, strength_multiplier: 1 });
        assert_eq!(level_2.get_effects()[0],
                  Effect::AttackToTarget { amount: 21, num_attacks: 1, strength_multiplier: 1 });
    }

    #[test]
    fn test_searing_blow_upgrade_multiple_levels() {
        // Test upgrading a card multiple levels at once
        let level_0 = searing_blow();
        let level_3 = upgrade_searing_blow_multiple_levels(level_0, 3);

        assert_eq!(level_3.get_upgrade_level(), 3);
        assert_eq!(level_3.get_effects()[0],
                  Effect::AttackToTarget { amount: 27, num_attacks: 1, strength_multiplier: 1 });
    }

    #[test]
    fn test_searing_blow_formula_matches_table() {
        // Verify the formula n(n+7)/2 + 12 matches the provided upgrade table
        for n in 0..=10u32 {
            let expected = if n == 0 { 12 } else { (n * (n + 7)) / 2 + 12 };
            let actual = calculate_searing_blow_damage(n);
            assert_eq!(actual, expected, "Damage calculation failed for level {}", n);
        }
    }
}