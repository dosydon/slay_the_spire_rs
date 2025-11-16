use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

/// Hemokinesis - Uncommon Attack Card
/// Cost: 1
/// Effect: Lose 2 HP. Deal 15 damage
pub fn hemokinesis() -> Card {
    Card::new(CardEnum::Hemokinesis, 1, CardType::Attack, vec![
        Effect::LoseHp(2),
        Effect::AttackToTarget {
            amount: 15,
            num_attacks: 1,
            strength_multiplier: 1,
        },
    ], false, true)
}

/// Hemokinesis+ (Upgraded)
/// Cost: 1
/// Effect: Lose 2 HP. Deal 22 damage
pub fn hemokinesis_upgraded() -> Card {
    Card::new(CardEnum::Hemokinesis, 1, CardType::Attack, vec![
        Effect::LoseHp(2),
        Effect::AttackToTarget {
            amount: 22,
            num_attacks: 1,
            strength_multiplier: 1,
        },
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hemokinesis_creation() {
        let card = hemokinesis();

        assert_eq!(card.get_name(), "Hemokinesis");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 2);
        assert_eq!(card.get_effects()[0], Effect::LoseHp(2));
        assert_eq!(card.get_effects()[1], Effect::AttackToTarget {
            amount: 15,
            num_attacks: 1,
            strength_multiplier: 1,
        });
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_hemokinesis_upgraded_creation() {
        let card = hemokinesis_upgraded();

        assert_eq!(card.get_name(), "Hemokinesis+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 2);
        assert_eq!(card.get_effects()[0], Effect::LoseHp(2));
        assert_eq!(card.get_effects()[1], Effect::AttackToTarget {
            amount: 22,
            num_attacks: 1,
            strength_multiplier: 1,
        });
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_hemokinesis_effect_order() {
        let card = hemokinesis();
        let effects = card.get_effects();

        // Verify that HP loss comes before damage
        assert_eq!(effects[0], Effect::LoseHp(2));
        assert_eq!(effects[1], Effect::AttackToTarget {
            amount: 15,
            num_attacks: 1,
            strength_multiplier: 1,
        });
    }

    #[test]
    fn test_hemokinesis_integration_player_loses_hp() {
        use crate::battle::{Battle, target::Entity};
        use crate::game::{global_info::GlobalInfo, deck::Deck};
        use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
        use crate::cards::ironclad::strike;
        use crate::game::enemy::EnemyTrait;
        use rand::RngCore;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 20, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = crate::battle::enemy_in_battle::EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        // Create deck with hemokinesis and strike cards
        let deck = Deck::new(vec![hemokinesis(), strike()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        let initial_player_hp = battle.get_player().battle_info.get_hp();
        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        assert_eq!(initial_player_hp, 50, "Player should start with 50 HP");
        assert!(initial_enemy_hp > 15, "Enemy should have more than 15 HP to survive the attack");

        // Find hemokinesis in hand
        let hemokinesis_idx = battle.get_hand().iter()
            .position(|card| card.get_name() == "Hemokinesis")
            .expect("Hemokinesis should be in hand");

        // Play hemokinesis targeting the enemy
        let result = battle.play_card(hemokinesis_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Playing hemokinesis should succeed");

        // Check that player lost 2 HP
        let final_player_hp = battle.get_player().battle_info.get_hp();
        assert_eq!(final_player_hp, 48, "Player should lose 2 HP (50 -> 48)");

        // Check that enemy took 15 damage
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 15, "Enemy should lose 15 HP from hemokinesis damage");
    }

    #[test]
    fn test_hemokinesis_upgraded_integration_player_loses_hp() {
        use crate::battle::{Battle, target::Entity};
        use crate::game::{global_info::GlobalInfo, deck::Deck};
        use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
        use crate::cards::ironclad::strike;
        use crate::game::enemy::EnemyTrait;
        use rand::RngCore;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 20, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = crate::battle::enemy_in_battle::EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        // Create deck with hemokinesis+ and strike cards
        let deck = Deck::new(vec![hemokinesis_upgraded(), strike()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        let initial_player_hp = battle.get_player().battle_info.get_hp();
        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        assert_eq!(initial_player_hp, 50, "Player should start with 50 HP");

        // Find hemokinesis+ in hand
        let hemokinesis_idx = battle.get_hand().iter()
            .position(|card| card.get_name() == "Hemokinesis+")
            .expect("Hemokinesis+ should be in hand");

        // Play hemokinesis+ targeting the enemy
        let result = battle.play_card(hemokinesis_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Playing hemokinesis+ should succeed");

        // Check that player lost 2 HP (same cost as normal version)
        let final_player_hp = battle.get_player().battle_info.get_hp();
        assert_eq!(final_player_hp, 48, "Player should lose 2 HP (50 -> 48)");

        // Check that enemy took 22 damage (upgraded version)
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 22, "Enemy should lose 22 HP from hemokinesis+ damage");
    }
}