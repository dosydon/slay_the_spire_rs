use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

/// Bludgeon - Rare Attack Card
/// Cost: 3
/// Effect: Deal 32 damage
pub fn bludgeon() -> Card {
    Card::new(CardEnum::Bludgeon, 3, CardType::Attack, vec![
        Effect::AttackToTarget {
            amount: 32,
            num_attacks: 1,
            strength_multiplier: 1,
        },
    ], false, true, Rarity::Rare)
}

/// Bludgeon+ (Upgraded)
/// Cost: 2
/// Effect: Deal 42 damage
pub fn bludgeon_upgraded() -> Card {
    Card::new(CardEnum::Bludgeon, 2, CardType::Attack, vec![
        Effect::AttackToTarget {
            amount: 42,
            num_attacks: 1,
            strength_multiplier: 1,
        },
    ], true, true, Rarity::Rare)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bludgeon_creation() {
        let card = bludgeon();

        assert_eq!(card.get_name(), "Bludgeon");
        assert_eq!(card.get_cost(), 3);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::AttackToTarget {
            amount: 32,
            num_attacks: 1,
            strength_multiplier: 1,
        });
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_bludgeon_upgraded_creation() {
        let card = bludgeon_upgraded();

        assert_eq!(card.get_name(), "Bludgeon+");
        assert_eq!(card.get_cost(), 3);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::AttackToTarget {
            amount: 42,
            num_attacks: 1,
            strength_multiplier: 1,
        });
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_bludgeon_integration_energy_spending() {
        use crate::battle::{Battle, target::Entity};
        use crate::game::{global_info::GlobalInfo, deck::Deck};
        use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
use crate::cards::ironclad::strike;
use crate::game::enemy::EnemyTrait;
        

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 20, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = crate::battle::enemy_in_battle::EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        // Create deck with bludgeon and strike cards
        let deck = Deck::new(vec![bludgeon(), strike()]);
        let mut battle = Battle::new(deck, global_info, 80, 80, vec![enemy], &mut rng);

        let initial_energy = battle.get_player().get_energy();
        assert_eq!(initial_energy, 3, "Player should start with 3 energy");

        // Find bludgeon in hand (it should be there since we put it in the deck)
        let bludgeon_idx = battle.get_hand().iter()
            .position(|card| card.get_name() == "Bludgeon")
            .expect("Bludgeon should be in hand");

        // Play bludgeon targeting the enemy
        let result = battle.play_card(bludgeon_idx, Entity::Enemy(0));
        assert!(result.is_ok(), "Playing bludgeon should succeed");

        // Check that 3 energy was spent
        let final_energy = battle.get_player().get_energy();
        assert_eq!(final_energy, 0, "Player should have 0 energy after playing bludgeon (cost 3)");

        // Verify enemy took damage (Red Louse with 11-14 HP should be defeated by 32 damage)
        assert!(!battle.get_enemies()[0].battle_info.is_alive(), "Enemy should be defeated by bludgeon damage");
    }
}