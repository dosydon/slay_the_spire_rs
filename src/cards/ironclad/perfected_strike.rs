use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

/// Perfected Strike: Deal 6 damage. Deals an additional 2 damage for ALL Strike cards in your deck.
pub fn perfected_strike() -> Card {
    Card::new(CardEnum::PerfectedStrike, 2, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        BattleEffect::PerfectedStrike {
            base_damage: 6,
            damage_per_strike: 2,
        }
    ])
}

/// Perfected Strike+: Deal 10 damage. Deals an additional 3 damage for ALL Strike cards in your deck.
pub fn perfected_strike_upgraded() -> Card {
    Card::new(CardEnum::PerfectedStrike, 2, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        BattleEffect::PerfectedStrike {
            base_damage: 10,
            damage_per_strike: 3,
        }
    ])
        .set_upgraded(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::Battle;
    use crate::battle::target::Entity;
    use crate::enemies::red_louse::RedLouse;
    use crate::enemies::EnemyEnum;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::game::PlayerRunState;
    use crate::game::deck::Deck;
    use crate::game::global_info::GlobalInfo;
    use crate::game::enemy::EnemyTrait;
    use crate::cards::ironclad::strike::strike;
    use crate::cards::ironclad::pommel_strike::pommel_strike;

    #[test]
    fn test_perfected_strike_base_damage() {
        // Test with no other Strike cards in deck
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        // Deck with only Perfected Strike
        let deck = Deck::new(vec![perfected_strike()]);
        let mut battle = Battle::new(deck, global_info.clone(), PlayerRunState::new(50, 80, 0), vec![enemy], &mut rng);

        // Check strike count
        let strike_count = battle.count_strike_cards_in_deck();
        assert_eq!(strike_count, 1, "Should have 1 Strike card (Perfected Strike itself)");

        let initial_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Play Perfected Strike (hand index 0) targeting enemy 0
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok(), "Failed to play Perfected Strike");

        // With only 1 Perfected Strike in deck: base damage 6 + (2 × 1 strike) = 8 damage
        let final_hp = battle.get_enemies()[0].battle_info.get_hp();
        let damage_dealt = initial_hp.saturating_sub(final_hp);
        assert_eq!(damage_dealt, 8, "Should deal 8 damage (6 base + 2*1 strikes)");
    }

    #[test]
    fn test_perfected_strike_with_multiple_strikes() {
        // Test with multiple Strike cards - use a fixed enemy HP for deterministic testing
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let mut enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));
        // Set a fixed HP to ensure deterministic damage testing
        enemy.battle_info.set_max_hp(100);
        enemy.battle_info.heal(100);

        // Deck with: Perfected Strike + 3 Strikes + 1 Pommel Strike = 5 Strike cards total
        let deck = Deck::new(vec![
            perfected_strike(),
            strike(),
            strike(),
            strike(),
            pommel_strike(),
        ]);
        let mut battle = Battle::new(deck, global_info.clone(), PlayerRunState::new(50, 80, 0), vec![enemy], &mut rng);

        // Check strike count before playing
        let strike_count = battle.count_strike_cards_in_deck();
        assert_eq!(strike_count, 5, "Should have 5 Strike cards in deck");

        let initial_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(initial_hp, 100, "Enemy should have 100 HP");

        // Play Perfected Strike (find it in hand)
        let hand = battle.get_hand();
        let perfected_strike_index = hand.iter().position(|c| c.get_card_enum() == CardEnum::PerfectedStrike);
        assert!(perfected_strike_index.is_some(), "Perfected Strike should be in hand");

        let result = battle.play_card(perfected_strike_index.unwrap(), Entity::Enemy(0));
        assert!(result.is_ok(), "Failed to play Perfected Strike");

        // With 5 Strike cards: base damage 6 + (2 × 5 strikes) = 16 damage
        let final_hp = battle.get_enemies()[0].battle_info.get_hp();
        let damage_dealt = initial_hp.saturating_sub(final_hp);
        assert_eq!(damage_dealt, 16, "Should deal 16 damage (6 base + 2*5 strikes)");
    }

    #[test]
    fn test_perfected_strike_upgraded() {
        // Test upgraded version - use fixed enemy HP for deterministic testing
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let mut enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));
        // Set a fixed HP to ensure deterministic damage testing
        enemy.battle_info.set_max_hp(100);
        enemy.battle_info.heal(100);

        // Deck with: Perfected Strike+ + 2 Strikes = 3 Strike cards total
        let deck = Deck::new(vec![
            perfected_strike_upgraded(),
            strike(),
            strike(),
        ]);
        let mut battle = Battle::new(deck, global_info.clone(), PlayerRunState::new(50, 80, 0), vec![enemy], &mut rng);

        let initial_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(initial_hp, 100, "Enemy should have 100 HP");

        // Find and play Perfected Strike+
        let hand = battle.get_hand();
        let perfected_strike_index = hand.iter().position(|c| c.get_card_enum() == CardEnum::PerfectedStrike);
        assert!(perfected_strike_index.is_some());

        let result = battle.play_card(perfected_strike_index.unwrap(), Entity::Enemy(0));
        assert!(result.is_ok());

        // With 3 Strike cards: base damage 10 + (3 × 3 strikes) = 19 damage
        let final_hp = battle.get_enemies()[0].battle_info.get_hp();
        let damage_dealt = initial_hp.saturating_sub(final_hp);
        assert_eq!(damage_dealt, 19, "Should deal 19 damage (10 base + 3*3 strikes)");
    }
}