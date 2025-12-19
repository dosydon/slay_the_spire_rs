use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum, card::Rarity};

/// Infernal Blade - Uncommon Skill Card
/// Cost: 1 (0 when upgraded)
/// Effect: Add a random Attack card to your hand. Exhaust.
pub fn infernal_blade() -> Card {
    Card::new_with_condition(
        CardEnum::InfernalBlade,
        1,
        CardType::Skill,
        vec![
            Effect::AddRandomAttackToHand,
            Effect::Exhaust,
        ],
        false, // not upgraded
        Condition::True,
        Rarity::Common,
    )
}

/// Infernal Blade+ (Upgraded version)
/// Cost: 0
/// Effect: Add a random Attack card to your hand. Exhaust.
pub fn infernal_blade_upgraded() -> Card {
    Card::new_with_condition(
        CardEnum::InfernalBlade,
        0, // Costs 0 when upgraded
        CardType::Skill,
        vec![
            Effect::AddRandomAttackToHand,
            Effect::Exhaust,
        ],
        true,  // upgraded
        Condition::True,
        Rarity::Common,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infernal_blade_creation() {
        let card = infernal_blade();

        assert_eq!(card.get_name(), "Infernal Blade");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_infernal_blade_upgraded_creation() {
        let card = infernal_blade_upgraded();

        assert_eq!(card.get_name(), "Infernal Blade+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_infernal_blade_effects() {
        let card = infernal_blade();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&Effect::AddRandomAttackToHand));
        assert!(effects.contains(&Effect::Exhaust));
    }

    #[test]
    fn test_infernal_blade_upgraded_effects() {
        let card = infernal_blade_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&Effect::AddRandomAttackToHand));
        assert!(effects.contains(&Effect::Exhaust));
    }

    #[test]
    fn test_infernal_blade_cost_upgrade() {
        let base_card = infernal_blade();
        let upgraded_card = infernal_blade_upgraded();

        assert_eq!(base_card.get_cost(), 1, "Infernal Blade should cost 1 energy");
        assert_eq!(upgraded_card.get_cost(), 0, "Infernal Blade+ should cost 0 energy");
    }

    #[test]
    fn test_infernal_blade_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::game::card_type::CardType;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Infernal Blade in hand
        let deck = Deck::new(vec![infernal_blade()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Check initial hand size
        let initial_hand_size = battle.cards.hand_size();
        let initial_exhausted_size = battle.cards.get_exhausted().len();

        // Give player enough energy to play Infernal Blade
        battle.get_player_mut().battle_info.energy = 1;

        // Play Infernal Blade
        let infernal_blade_idx = 0;
        let result = battle.play_card(infernal_blade_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify hand size increased by 1 (lost Infernal Blade, gained random Attack)
        let final_hand_size = battle.cards.hand_size();
        assert_eq!(final_hand_size, initial_hand_size); // -1 (played) +1 (new card) = 0 net change

        // Verify Infernal Blade was exhausted
        let final_exhausted_size = battle.cards.get_exhausted().len();
        assert_eq!(final_exhausted_size, initial_exhausted_size + 1);

        // Verify the new card is an Attack card
        if final_hand_size > 0 {
            let hand = battle.cards.get_hand();
            let new_card_is_attack = hand.iter().any(|card| card.get_card_type() == &CardType::Attack);
            assert!(new_card_is_attack, "Should have added an Attack card to hand");
        }
    }

    #[test]
    fn test_infernal_blade_upgraded_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::game::card_type::CardType;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Infernal Blade+ in hand
        let deck = Deck::new(vec![infernal_blade_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Check initial hand size and energy
        let initial_hand_size = battle.cards.hand_size();
        let initial_energy = battle.get_player().get_energy();

        // Play Infernal Blade+ (costs 0 energy)
        let infernal_blade_idx = 0;
        let result = battle.play_card(infernal_blade_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify energy was not spent (costs 0)
        let final_energy = battle.get_player().get_energy();
        assert_eq!(final_energy, initial_energy);

        // Verify hand size increased by 1 (lost Infernal Blade+, gained random Attack)
        let final_hand_size = battle.cards.hand_size();
        assert_eq!(final_hand_size, initial_hand_size); // -1 (played) +1 (new card) = 0 net change

        // Verify the new card is an Attack card
        if final_hand_size > 0 {
            let hand = battle.cards.get_hand();
            let new_card_is_attack = hand.iter().any(|card| card.get_card_type() == &CardType::Attack);
            assert!(new_card_is_attack, "Should have added an Attack card to hand");
        }
    }

    #[test]
    fn test_infernal_blade_insufficient_energy() {
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

        // Create battle with Infernal Blade in hand
        let deck = Deck::new(vec![infernal_blade()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player 0 energy
        battle.get_player_mut().battle_info.energy = 0;

        // Try to play Infernal Blade - should fail due to insufficient energy
        let infernal_blade_idx = 0;
        let result = battle.play_card(infernal_blade_idx, Entity::Player);
        assert!(result.is_err()); // Should return an error
    }

    #[test]
    fn test_infernal_blade_upgraded_no_energy_required() {
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

        // Create battle with Infernal Blade+ in hand
        let deck = Deck::new(vec![infernal_blade_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player 0 energy (Infernal Blade+ costs 0)
        battle.get_player_mut().battle_info.energy = 0;

        // Should be able to play Infernal Blade+ even with 0 energy
        let infernal_blade_idx = 0;
        let result = battle.play_card(infernal_blade_idx, Entity::Player);
        assert!(result.is_ok()); // Should succeed
    }
}