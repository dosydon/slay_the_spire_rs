use crate::game::{card::Card, effect::Effect, card_type::CardType, card_enum::CardEnum, card::Rarity};

/// Sever Soul - Uncommon Attack Card
/// Cost: 2 (2 when upgraded)
/// Effect: Deal 16 damage. Exhaust all non-Attack cards in hand
pub fn sever_soul() -> Card {
    Card::new(
        CardEnum::SeverSoul,
        2,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: 16, num_attacks: 1, strength_multiplier: 1 },
            Effect::ExhaustNonAttacksInHand,
        ],
        Rarity::Uncommon
    )
        .set_playable(true)
}

/// Sever Soul+ (Upgraded version)
/// Cost: 2
/// Effect: Deal 22 damage. Exhaust all non-Attack cards in hand
pub fn sever_soul_upgraded() -> Card {
    Card::new(
        CardEnum::SeverSoul,
        2,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: 22, num_attacks: 1, strength_multiplier: 1 },
            Effect::ExhaustNonAttacksInHand,
        ],
        Rarity::Uncommon
    )
        .set_upgraded(true)
        .set_playable(true)
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
    use crate::cards::ironclad::strike::strike;
    use crate::cards::ironclad::defend::defend;

    #[test]
    fn test_sever_soul_creation() {
        let card = sever_soul();
        assert_eq!(card.get_name(), "Sever Soul");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_sever_soul_upgraded_creation() {
        let card = sever_soul_upgraded();
        assert_eq!(card.get_name(), "Sever Soul+");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_sever_soul_effects() {
        let card = sever_soul();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 16);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }

        match &effects[1] {
            Effect::ExhaustNonAttacksInHand => {
                // This is the correct effect type
            }
            _ => panic!("Expected ExhaustNonAttacksInHand effect"),
        }
    }

    #[test]
    fn test_sever_soul_upgraded_effects() {
        let card = sever_soul_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 22);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_sever_soul_battle_integration_mixed_hand() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        // Create hand with mix of Attack and non-Attack cards
        let deck = Deck::new(vec![
            sever_soul(),
            strike(),
            defend(),
            crate::cards::ironclad::flex::flex(),
        ]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), vec![enemy], &mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();
        let initial_exhausted_size = battle.cards.exhausted_size();

        // Play Sever Soul (should be index 0)
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify damage dealt
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp.saturating_sub(16));

        // Verify non-Attack cards exhausted (defend and flex = 2 cards)
        // Attack card (strike) should remain in hand
        assert_eq!(battle.cards.hand_size(), 1); // Only Strike should remain
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted_size + 2); // defend + flex

        // Verify the remaining card is an Attack
        let hand = battle.get_hand();
        assert_eq!(hand.len(), 1);
        assert_eq!(hand[0].get_card_type(), &CardType::Attack);
    }

    #[test]
    fn test_sever_soul_battle_integration_only_attacks() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        // Create hand with only Attack cards
        let deck = Deck::new(vec![
            sever_soul(),
            strike(),
            strike(),
        ]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), vec![enemy], &mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();
        let initial_hand_size = battle.cards.hand_size();
        let initial_exhausted_size = battle.cards.exhausted_size();

        // Play Sever Soul
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify damage dealt
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp.saturating_sub(16));

        // Verify only Sever Soul exhausted (no non-Attack cards to exhaust)
        assert_eq!(battle.cards.hand_size(), initial_hand_size - 1); // Only Sever Soul removed
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted_size); 
    }

    #[test]
    fn test_sever_soul_upgraded_battle_integration() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let deck = Deck::new(vec![
            sever_soul_upgraded(),
            strike(),
            defend(),
        ]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), vec![enemy], &mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();

        // Play Sever Soul+
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify increased damage dealt
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp.saturating_sub(22));

        // Verify non-Attack card exhausted
        assert_eq!(battle.cards.hand_size(), 1); // Only Strike should remain
        assert_eq!(battle.cards.get_hand()[0].get_name(), "Strike");
    }

    #[test]
    fn test_sever_soul_costs_energy() {
        let normal_card = sever_soul();
        let upgraded_card = sever_soul_upgraded();

        assert_eq!(normal_card.get_cost(), 2, "Sever Soul should cost 2 energy");
        assert_eq!(upgraded_card.get_cost(), 2, "Sever Soul+ should also cost 2 energy");
    }

    #[test]
    fn test_sever_soul_card_enum() {
        let card = sever_soul();
        let card_enum = card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::SeverSoul));
    }
}