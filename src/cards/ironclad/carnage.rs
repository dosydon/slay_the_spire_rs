use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

/// Carnage - Uncommon Attack Card
/// Cost: 2
/// Deal 20 damage
/// Ethereal: Exhausts at end of turn if not played
pub fn carnage() -> Card {
    Card::new_with_ethereal(CardEnum::Carnage, 2, CardType::Attack, vec![
        Effect::AttackToTarget {
            amount: 20,
            num_attacks: 1,
            strength_multiplier: 1,
        },
    ], false, true, true, Rarity::Uncommon)
}

/// Carnage+ (Upgraded)
/// Cost: 2
/// Deal 28 damage
/// Ethereal: Exhausts at end of turn if not played
pub fn carnage_upgraded() -> Card {
    Card::new_with_ethereal(CardEnum::Carnage, 2, CardType::Attack, vec![
        Effect::AttackToTarget {
            amount: 28,
            num_attacks: 1,
            strength_multiplier: 1,
        },
    ], true, true, true, Rarity::Uncommon)
}

#[cfg(test)]
mod tests {
    use crate::game::PlayerRunState;

    use super::*;

    #[test]
    fn test_carnage_creation() {
        let card = carnage();

        assert_eq!(card.get_name(), "Carnage");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
        assert!(card.is_ethereal());
    }

    #[test]
    fn test_carnage_upgraded_creation() {
        let card = carnage_upgraded();

        assert_eq!(card.get_name(), "Carnage+");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
        assert!(card.is_ethereal());
    }

    #[test]
    fn test_carnage_damage() {
        let card = carnage();

        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 20);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_carnage_upgraded_damage() {
        let card = carnage_upgraded();

        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 28);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_carnage_battle_integration() {
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

        // Create battle with Carnage in hand
        let deck = Deck::new(vec![carnage()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Play Carnage targeting the enemy
        let carnage_idx = 0;
        let result = battle.play_card(carnage_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify damage was dealt (20 damage or enemy defeated)
        let enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let expected_hp = if initial_hp > 20 { initial_hp - 20 } else { 0 };
        assert_eq!(enemy_hp, expected_hp);

        // Verify Carnage went to discard pile (not a power card)
        let discard = battle.cards.get_discard_pile();
        assert_eq!(discard.len(), 1);
        assert_eq!(discard[0].get_name(), "Carnage");
    }

    #[test]
    fn test_carnage_upgraded_battle_integration() {
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

        // Create battle with Carnage+ in hand
        let deck = Deck::new(vec![carnage_upgraded()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Play Carnage+ targeting the enemy
        let carnage_idx = 0;
        let result = battle.play_card(carnage_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify damage was dealt (28 damage or enemy defeated)
        let enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let expected_hp = if initial_hp > 28 { initial_hp - 28 } else { 0 };
        assert_eq!(enemy_hp, expected_hp);

        // Verify Carnage+ went to discard pile
        let discard = battle.cards.get_discard_pile();
        assert_eq!(discard.len(), 1);
        assert_eq!(discard[0].get_name(), "Carnage+");
    }
}
