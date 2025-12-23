use crate::game::{card::Card, effect::BattleEffect, card_type::CardType, card_enum::CardEnum, card::{Rarity, CardClass}};

/// Second Wind - Uncommon Skill Card
/// Cost: 1
/// Effect: Exhaust all non-Attack cards from your hand. Gain 5 Block per card exhausted.
pub fn second_wind() -> Card {
    Card::new(CardEnum::SecondWind, 1, CardClass::IronClad(Rarity::Uncommon, CardType::Skill), vec![BattleEffect::ExhaustNonAttackCardsFromHand { block_per_card: 5 }])
        .set_playable(true)
}

/// Second Wind+ (Upgraded version)
/// Cost: 1
/// Effect: Exhaust all non-Attack cards from your hand. Gain 8 Block per card exhausted.
pub fn second_wind_upgraded() -> Card {
    Card::new(CardEnum::SecondWind, 1, CardClass::IronClad(Rarity::Uncommon, CardType::Skill), vec![BattleEffect::ExhaustNonAttackCardsFromHand { block_per_card: 8 }])
        .set_upgraded(true)
        .set_playable(true)
}

#[cfg(test)]
mod tests {
    use crate::game::PlayerRunState;

    use super::*;

    #[test]
    fn test_second_wind_creation() {
        let card = second_wind();
        assert_eq!(card.get_name(), "Second Wind");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_second_wind_upgraded_creation() {
        let card = second_wind_upgraded();
        assert_eq!(card.get_name(), "Second Wind+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_second_wind_effect() {
        let card = second_wind();
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::ExhaustNonAttackCardsFromHand { block_per_card: 5 }));
    }

    #[test]
    fn test_second_wind_upgraded_effect() {
        let card = second_wind_upgraded();
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::ExhaustNonAttackCardsFromHand { block_per_card: 8 }));
    }

    #[test]
    fn test_second_wind_battle_integration_with_only_attacks() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::gremlin_nob::GremlinNob;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];

        // Create battle with Second Wind and only Attack cards in hand
        let deck = Deck::new(vec![
            second_wind(),
            strike(),  // Attack card
            strike(),  // Attack card
            strike(),  // Attack card
        ]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Draw all cards to hand
        for _ in 0..4 {
            battle.cards.draw_card();
        }

        let initial_block = battle.get_player().battle_info.get_block();
        let initial_hand_size = battle.cards.hand_size();

        // Play Second Wind
        let second_wind_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Second Wind").unwrap();
        let result = battle.play_card(second_wind_idx, Entity::Player);
        assert!(result.is_ok());

        // Should not gain any block since all cards are Attacks
        let final_block = battle.get_player().battle_info.get_block();
        assert_eq!(final_block, initial_block);

        // Hand should only contain Attack cards (no cards exhausted)
        let final_hand_size = battle.cards.hand_size();
        assert_eq!(final_hand_size, initial_hand_size - 1); // Only Second Wind removed

        // All remaining cards should be Attack cards
        let hand = battle.cards.get_hand();
        for card in hand {
            assert_eq!(card.get_card_type(), CardType::Attack);
        }
    }

    #[test]
    fn test_second_wind_battle_integration_with_non_attacks() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::gremlin_nob::GremlinNob;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike;
        use crate::cards::ironclad::defend::defend;
        use crate::cards::status::wound::wound;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];

        // Create battle with Second Wind and mixed cards in hand
        let deck = Deck::new(vec![
            second_wind(),
            strike(),  // Attack card
            defend(),  // Skill card (will be exhausted)
            wound(),   // Status card (will be exhausted)
        ]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Draw all cards to hand
        for _ in 0..4 {
            battle.cards.draw_card();
        }

        let initial_block = battle.get_player().battle_info.get_block();

        // Play Second Wind
        let second_wind_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Second Wind").unwrap();
        let result = battle.play_card(second_wind_idx, Entity::Player);
        assert!(result.is_ok());

        // Should gain block for 2 non-Attack cards (defend and wound)
        let final_block = battle.get_player().battle_info.get_block();
        assert_eq!(final_block, initial_block + (2 * 5)); // 2 cards * 5 block each

        // Hand should only contain the Attack card that wasn't exhausted
        let final_hand_size = battle.cards.hand_size();
        assert_eq!(final_hand_size, 1); // Only the strike remains

        // The remaining card should be an Attack
        let hand = battle.cards.get_hand();
        assert_eq!(hand.len(), 1);
        assert_eq!(hand[0].get_card_type(), CardType::Attack);
        assert_eq!(hand[0].get_name(), "Strike");
    }

    #[test]
    fn test_second_wind_upgraded_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::gremlin_nob::GremlinNob;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike;
        use crate::cards::ironclad::defend::defend;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];

        // Create battle with Second Wind+ and mixed cards in hand
        let deck = Deck::new(vec![
            second_wind_upgraded(),
            strike(),  // Attack card
            defend(),  // Skill card (will be exhausted)
        ]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Draw all cards to hand
        for _ in 0..3 {
            battle.cards.draw_card();
        }

        let initial_block = battle.get_player().battle_info.get_block();

        // Play Second Wind+
        let second_wind_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Second Wind+").unwrap();
        let result = battle.play_card(second_wind_idx, Entity::Player);
        assert!(result.is_ok());

        // Should gain upgraded block for 1 non-Attack card (defend)
        let final_block = battle.get_player().battle_info.get_block();
        assert_eq!(final_block, initial_block + (1 * 8)); // 1 card * 8 block each (upgraded)
    }

    #[test]
    fn test_second_wind_upgraded_name() {
        let card = second_wind();
        assert_eq!(card.get_card_enum().upgraded_name(), "Second Wind+");
    }
}