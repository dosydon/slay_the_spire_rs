use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// Spot Weakness - Uncommon Skill Card
/// Cost: 1 (1 when upgraded)
/// Effect: If enemy is attacking, gain 3 Strength. Exhaust
pub fn spot_weakness() -> Card {
    Card::new(
        CardEnum::SpotWeakness,
        1,
        CardType::Skill,
        vec![
            Effect::GainStrengthIfEnemyAttacking { amount: 3 },
        ],
        false, // not upgraded
        Condition::EnemyIsAttacking,
    )
}

/// Spot Weakness+ (Upgraded version)
/// Cost: 1
/// Effect: If enemy is attacking, gain 4 Strength. Exhaust
pub fn spot_weakness_upgraded() -> Card {
    Card::new(
        CardEnum::SpotWeakness,
        1,
        CardType::Skill,
        vec![
            Effect::GainStrengthIfEnemyAttacking { amount: 4 },
        ],
        true,  // upgraded
        Condition::EnemyIsAttacking,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, target::Entity};
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::game::deck::Deck;
    use crate::game::global_info::GlobalInfo;
    use crate::game::enemy::EnemyTrait;
    use crate::enemies::jaw_worm::JawWorm;
    use crate::enemies::enemy_enum::{EnemyEnum, EnemyMove};
    use crate::enemies::red_louse::RedLouse;

    #[test]
    fn test_spot_weakness_creation() {
        let card = spot_weakness();
        assert_eq!(card.get_name(), "Spot Weakness");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_spot_weakness_upgraded_creation() {
        let card = spot_weakness_upgraded();
        assert_eq!(card.get_name(), "Spot Weakness+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_spot_weakness_effects() {
        let card = spot_weakness();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::GainStrengthIfEnemyAttacking { amount } => {
                assert_eq!(*amount, 3);
            }
            _ => panic!("Expected GainStrengthIfEnemyAttacking effect"),
        }
    }

    #[test]
    fn test_spot_weakness_upgraded_effects() {
        let card = spot_weakness_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::GainStrengthIfEnemyAttacking { amount } => {
                assert_eq!(*amount, 4);
            }
            _ => panic!("Expected GainStrengthIfEnemyAttacking effect"),
        }
    }

    #[test]
    fn test_spot_weakness_condition() {
        let card = spot_weakness();
        let play_condition = card.get_play_condition();

        // Should use EnemyIsAttacking condition
        assert!(matches!(play_condition, Condition::EnemyIsAttacking));
    }

    #[test]
    fn test_spot_weakness_playable_when_enemy_attacking() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let mut enemy = EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm));

        // Force enemy to have an attacking move
        enemy.battle_info.set_next_move(&EnemyMove::Attack { amount: 12 });

        let deck = Deck::new(vec![spot_weakness()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        let initial_strength = battle.get_player().battle_info.get_strength();

        // Play Spot Weakness when enemy is attacking
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok(), "Spot Weakness should be playable when enemy is attacking");

        // Verify strength gained
        assert_eq!(battle.get_player().battle_info.get_strength(), initial_strength + 3);
    }

    #[test]
    fn test_spot_weakness_not_playable_when_enemy_not_attacking() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let mut enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        // Force enemy to have a non-attacking move (like gain strength)
        enemy.battle_info.set_next_move(&EnemyMove::GainStrength { amount: 3 });

        let deck = Deck::new(vec![spot_weakness()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        // Try to play Spot Weakness when enemy is not attacking
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_err(), "Spot Weakness should not be playable when enemy is not attacking");
    }

    #[test]
    fn test_spot_weakness_upgraded_battle_integration() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let mut enemy = EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm));

        // Force enemy to have an attacking move
        enemy.battle_info.set_next_move(&EnemyMove::Attack { amount: 12 });

        let deck = Deck::new(vec![spot_weakness_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        let initial_strength = battle.get_player().battle_info.get_strength();

        // Play Spot Weakness+ when enemy is attacking
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify increased strength gained
        assert_eq!(battle.get_player().battle_info.get_strength(), initial_strength + 4);
    }

    #[test]
    fn test_spot_weakness_costs_energy() {
        let normal_card = spot_weakness();
        let upgraded_card = spot_weakness_upgraded();

        assert_eq!(normal_card.get_cost(), 1, "Spot Weakness should cost 1 energy");
        assert_eq!(upgraded_card.get_cost(), 1, "Spot Weakness+ should cost 1 energy");
    }

    #[test]
    fn test_spot_weakness_card_enum() {
        let card = spot_weakness();
        let card_enum = card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::SpotWeakness));
    }

    #[test]
    fn test_spot_weakness_multiple_enemies_attacking() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let jaw_worm1 = JawWorm::instantiate(&mut rng, &global_info);
        let jaw_worm2 = JawWorm::instantiate(&mut rng, &global_info);
        let mut enemy1 = EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm1));
        let mut enemy2 = EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm2));

        // Make both enemies attack
        enemy1.battle_info.set_next_move(&EnemyMove::Attack { amount: 10 });
        enemy2.battle_info.set_next_move(&EnemyMove::Attack { amount: 8 });

        let deck = Deck::new(vec![spot_weakness()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy1, enemy2], &mut rng);

        let initial_strength = battle.get_player().battle_info.get_strength();

        // Play Spot Weakness when at least one enemy is attacking
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok(), "Spot Weakness should be playable when any enemy is attacking");

        // Verify strength gained
        assert_eq!(battle.get_player().battle_info.get_strength(), initial_strength + 3);
    }
}