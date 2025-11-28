use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

/// Shockwave - Skill Card
/// Cost: 2
/// Effect: Apply 3 Weak and 3 Vulnerable to ALL enemies.
pub fn shockwave() -> Card {
    Card::new(CardEnum::Shockwave, 2, CardType::Skill, vec![
        Effect::ApplyWeakAll { duration: 3 },
        Effect::ApplyVulnerableAll { duration: 3 },
    ], false, true)
}

/// Shockwave+ (Upgraded)
/// Cost: 2
/// Effect: Apply 3 Weak and 3 Vulnerable to ALL enemies.
pub fn shockwave_upgraded() -> Card {
    Card::new(CardEnum::Shockwave, 2, CardType::Skill, vec![
        Effect::ApplyWeakAll { duration: 3 },
        Effect::ApplyVulnerableAll { duration: 3 },
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shockwave_creation() {
        let card = shockwave();

        assert_eq!(card.get_name(), "Shockwave");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_shockwave_upgraded_creation() {
        let card = shockwave_upgraded();

        assert_eq!(card.get_name(), "Shockwave+");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_shockwave_effects() {
        let normal_card = shockwave();
        let upgraded_card = shockwave_upgraded();

        let normal_effects = normal_card.get_effects();
        let upgraded_effects = upgraded_card.get_effects();

        // Both should have same effects
        assert_eq!(normal_effects.len(), 2);
        assert_eq!(upgraded_effects.len(), 2);

        // Both should apply Weak to all enemies
        assert_eq!(normal_effects[0], Effect::ApplyWeakAll { duration: 3 });
        assert_eq!(upgraded_effects[0], Effect::ApplyWeakAll { duration: 3 });

        // Both should apply Vulnerable to all enemies
        assert_eq!(normal_effects[1], Effect::ApplyVulnerableAll { duration: 3 });
        assert_eq!(upgraded_effects[1], Effect::ApplyVulnerableAll { duration: 3 });
    }

    #[test]
    fn test_shockwave_multiple_enemies() {
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

        // Create battle with Shockwave in hand
        let deck = Deck::new(vec![shockwave()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Play Shockwave
        let shockwave_idx = 0;
        let result = battle.play_card(shockwave_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify all enemies have status effects regardless of enemy type
        for enemy in battle.get_enemies() {
            assert_eq!(enemy.get_weak(), 3);
            assert_eq!(enemy.get_vulnerable(), 3);
        }
    }
}