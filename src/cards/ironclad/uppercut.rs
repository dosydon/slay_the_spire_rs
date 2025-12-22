use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::{Rarity, CardClass}};

/// Uppercut - Attack Card
/// Cost: 2
/// Effect: Deal 13 damage. Apply 1 Weak and 1 Vulnerable.
pub fn uppercut() -> Card {
    Card::new(CardEnum::Uppercut, 2, CardClass::IronClad(Rarity::Uncommon, CardType::Attack), vec![
        Effect::AttackToTarget {
            amount: 13,
            num_attacks: 1,
            strength_multiplier: 1,
        },
        Effect::ApplyWeak { duration: 1 },
        Effect::ApplyVulnerable { duration: 1 },
    ])
        .set_playable(true)
}

/// Uppercut+ (Upgraded)
/// Cost: 2
/// Effect: Deal 13 damage. Apply 2 Weak and 2 Vulnerable.
pub fn uppercut_upgraded() -> Card {
    Card::new(CardEnum::Uppercut, 2, CardClass::IronClad(Rarity::Uncommon, CardType::Attack), vec![
        Effect::AttackToTarget {
            amount: 13,
            num_attacks: 1,
            strength_multiplier: 1,
        },
        Effect::ApplyWeak { duration: 2 },
        Effect::ApplyVulnerable { duration: 2 },
    ])
        .set_upgraded(true)
        .set_playable(true)
}

#[cfg(test)]
mod tests {
    use crate::{enemies::gremlin_nob::GremlinNob, game::PlayerRunState};

    use super::*;

    #[test]
    fn test_uppercut_creation() {
        let card = uppercut();

        assert_eq!(card.get_name(), "Uppercut");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), CardType::Attack);
        assert_eq!(card.get_effects().len(), 3);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_uppercut_upgraded_creation() {
        let card = uppercut_upgraded();

        assert_eq!(card.get_name(), "Uppercut+");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), CardType::Attack);
        assert_eq!(card.get_effects().len(), 3);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_uppercut_effects() {
        let normal_card = uppercut();
        let upgraded_card = uppercut_upgraded();

        let normal_effects = normal_card.get_effects();
        let upgraded_effects = upgraded_card.get_effects();

        // Both should have 3 effects
        assert_eq!(normal_effects.len(), 3);
        assert_eq!(upgraded_effects.len(), 3);

        // Both should deal 13 damage
        assert_eq!(normal_effects[0], Effect::AttackToTarget {
            amount: 13,
            num_attacks: 1,
            strength_multiplier: 1,
        });
        assert_eq!(upgraded_effects[0], Effect::AttackToTarget {
            amount: 13,
            num_attacks: 1,
            strength_multiplier: 1,
        });

        // Normal should apply 1 Weak, upgraded should apply 2 Weak
        assert_eq!(normal_effects[1], Effect::ApplyWeak { duration: 1 });
        assert_eq!(upgraded_effects[1], Effect::ApplyWeak { duration: 2 });

        // Normal should apply 1 Vulnerable, upgraded should apply 2 Vulnerable
        assert_eq!(normal_effects[2], Effect::ApplyVulnerable { duration: 1 });
        assert_eq!(upgraded_effects[2], Effect::ApplyVulnerable { duration: 2 });
    }

    #[test]
    fn test_uppercut_upgraded_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(GremlinNob::new(60)))];

        // Create battle with Uppercut+ in hand
        let deck = Deck::new(vec![uppercut_upgraded()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let enemy_initial_hp = battle.get_enemies()[0].get_current_hp();
        let enemy_initial_weak = battle.get_enemies()[0].get_weak();
        let enemy_initial_vulnerable = battle.get_enemies()[0].get_vulnerable();

        // Play Uppercut+ targeting the first enemy
        let uppercut_idx = 0;
        let result = battle.play_card(uppercut_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify enemy took 13 damage
        let enemy_final_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(enemy_final_hp, enemy_initial_hp.saturating_sub(13));

        // Verify enemy has 2 Weak and 2 Vulnerable
        let enemy_final_weak = battle.get_enemies()[0].get_weak();
        let enemy_final_vulnerable = battle.get_enemies()[0].get_vulnerable();
        assert_eq!(enemy_final_weak, enemy_initial_weak + 2);
        assert_eq!(enemy_final_vulnerable, enemy_initial_vulnerable + 2);

        // Verify Uppercut+ is in discard pile
        let hand = battle.get_hand();
        assert_eq!(hand.len(), 0);
    }

    #[test]
    fn test_uppercut_multiple_targets() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::jaw_worm::JawWorm;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse)),
            EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm)),
        ];

        // Create battle with Uppercut in hand
        let deck = Deck::new(vec![uppercut()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let enemy1_initial_hp = battle.get_enemies()[0].get_current_hp();
        let enemy2_initial_hp = battle.get_enemies()[1].get_current_hp();
        let enemy1_initial_weak = battle.get_enemies()[0].get_weak();
        let enemy2_initial_weak = battle.get_enemies()[1].get_weak();

        // Play Uppercut targeting the second enemy
        let uppercut_idx = 0;
        let result = battle.play_card(uppercut_idx, Entity::Enemy(1));
        assert!(result.is_ok());

        // Verify only second enemy took damage and got status effects
        let enemy1_final_hp = battle.get_enemies()[0].get_current_hp();
        let enemy2_final_hp = battle.get_enemies()[1].get_current_hp();
        let enemy1_final_weak = battle.get_enemies()[0].get_weak();
        let enemy2_final_weak = battle.get_enemies()[1].get_weak();

        assert_eq!(enemy1_final_hp, enemy1_initial_hp); // No damage
        assert_eq!(enemy2_final_hp, enemy2_initial_hp - 13); // 13 damage
        assert_eq!(enemy1_final_weak, enemy1_initial_weak); // No status effect
        assert_eq!(enemy2_final_weak, enemy2_initial_weak + 1); // 1 Weak applied
    }

    #[test]
    fn test_uppercut_with_artifact() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(GremlinNob::new(60)))];

        // Create battle with Uppercut in hand
        let deck = Deck::new(vec![uppercut()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Give enemy 2 Artifact charges
        battle.get_enemies_mut()[0].battle_info.gain_artifact(2);

        let enemy_initial_hp = battle.get_enemies()[0].get_current_hp();
        let enemy_initial_artifact = battle.get_enemies()[0].battle_info.get_artifact();
        let enemy_initial_weak = battle.get_enemies()[0].get_weak();
        let enemy_initial_vulnerable = battle.get_enemies()[0].get_vulnerable();

        // Verify enemy has 2 Artifact charges
        assert_eq!(enemy_initial_artifact, 2);

        // Play Uppercut targeting the enemy
        let uppercut_idx = 0;
        let result = battle.play_card(uppercut_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify enemy took 13 damage (Artifact doesn't block damage)
        let enemy_final_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(enemy_final_hp, enemy_initial_hp.saturating_sub(13));

        // Verify enemy consumed 2 Artifact charges (one for Weak, one for Vulnerable)
        let enemy_final_artifact = battle.get_enemies()[0].battle_info.get_artifact();
        assert_eq!(enemy_final_artifact, 0);

        // Verify enemy has NO Weak or Vulnerable (blocked by Artifact)
        let enemy_final_weak = battle.get_enemies()[0].get_weak();
        let enemy_final_vulnerable = battle.get_enemies()[0].get_vulnerable();
        assert_eq!(enemy_final_weak, enemy_initial_weak);
        assert_eq!(enemy_final_vulnerable, enemy_initial_vulnerable);
    }

    #[test]
    fn test_uppercut_with_partial_artifact() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(GremlinNob::new(60)))];

        // Create battle with Uppercut in hand
        let deck = Deck::new(vec![uppercut()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Give enemy only 1 Artifact charge (not enough to block both debuffs)
        battle.get_enemies_mut()[0].battle_info.gain_artifact(1);

        let enemy_initial_hp = battle.get_enemies()[0].get_current_hp();
        let enemy_initial_artifact = battle.get_enemies()[0].battle_info.get_artifact();
        let enemy_initial_weak = battle.get_enemies()[0].get_weak();
        let enemy_initial_vulnerable = battle.get_enemies()[0].get_vulnerable();

        // Verify enemy has 1 Artifact charge
        assert_eq!(enemy_initial_artifact, 1);

        // Play Uppercut targeting the enemy
        let uppercut_idx = 0;
        let result = battle.play_card(uppercut_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify enemy took 13 damage
        let enemy_final_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(enemy_final_hp, enemy_initial_hp.saturating_sub(13));

        // Verify enemy consumed the 1 Artifact charge
        let enemy_final_artifact = battle.get_enemies()[0].battle_info.get_artifact();
        assert_eq!(enemy_final_artifact, 0);

        // Verify enemy has one debuff blocked and one applied
        // The first debuff (Weak) is blocked by Artifact, the second (Vulnerable) is applied
        let enemy_final_weak = battle.get_enemies()[0].get_weak();
        let enemy_final_vulnerable = battle.get_enemies()[0].get_vulnerable();
        assert_eq!(enemy_final_weak, enemy_initial_weak); // Blocked by Artifact
        assert_eq!(enemy_final_vulnerable, enemy_initial_vulnerable + 1); // Applied
    }
}