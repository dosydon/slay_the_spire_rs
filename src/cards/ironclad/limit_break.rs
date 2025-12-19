use crate::game::{card::{Card, Rarity}, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// Limit Break - Rare Skill Card
/// Effect: Double your Strength. Exhaust.
pub fn limit_break() -> Card {
    Card::new_with_condition(
        CardEnum::LimitBreak,
        1,
        CardType::Skill,
        vec![
            Effect::DoubleStrength, // Double current Strength
            Effect::Exhaust,         // Exhaust after use
        ],
        false, // not upgraded
        Condition::True,
        Rarity::Uncommon)
}

/// Limit Break+ (Upgraded version)
/// Cost: 1 (1 when upgraded)
/// Effect: Double your Strength. (No exhaust when upgraded)
pub fn limit_break_upgraded() -> Card {
    Card::new_with_condition(
        CardEnum::LimitBreak,
        1, // Costs 1 when upgraded
        CardType::Skill,
        vec![
            Effect::DoubleStrength, // Double current Strength
            // No Exhaust when upgraded
        ],
        true,  // upgraded
        Condition::True,
        Rarity::Uncommon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_break_creation() {
        let card = limit_break();

        assert_eq!(card.get_name(), "Limit Break");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_limit_break_upgraded_creation() {
        let card = limit_break_upgraded();

        assert_eq!(card.get_name(), "Limit Break+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 1); // Only DoubleStrength, no Exhaust
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_limit_break_effects() {
        let card = limit_break();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0], Effect::DoubleStrength);
        assert_eq!(effects[1], Effect::Exhaust);
    }

    #[test]
    fn test_limit_break_upgraded_effects() {
        let card = limit_break_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1); // Only DoubleStrength, no Exhaust when upgraded
        assert_eq!(effects[0], Effect::DoubleStrength);
    }

    #[test]
    fn test_limit_break_cost_upgrade() {
        let base_card = limit_break();
        let upgraded_card = limit_break_upgraded();

        assert_eq!(base_card.get_cost(), 1, "Limit Break should cost 1 energy");
        assert_eq!(upgraded_card.get_cost(), 1, "Limit Break+ should cost 1 energy");
    }

    #[test]
    fn test_limit_break_upgrade_maintains_effects() {
        let base_card = limit_break();
        let upgraded_card = limit_break_upgraded();

        let base_effects = base_card.get_effects();
        let upgraded_effects = upgraded_card.get_effects();

        // Base version has DoubleStrength + Exhaust
        assert_eq!(base_effects.len(), 2);
        // Upgraded version has only DoubleStrength (no Exhaust)
        assert_eq!(upgraded_effects.len(), 1);

        // Both should double Strength
        assert_eq!(base_effects[0], Effect::DoubleStrength);
        assert_eq!(upgraded_effects[0], Effect::DoubleStrength);

        // Only base version should exhaust
        assert_eq!(base_effects[1], Effect::Exhaust);
    }

    #[test]
    fn test_limit_break_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::gremlin_nob::GremlinNob;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::inflame::inflame;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];

        // Create battle with Limit Break and Inflame in hand
        let deck = Deck::new(vec![limit_break(), inflame()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player enough energy to play both cards
        battle.get_player_mut().battle_info.energy = 3;

        // Check initial strength
        let initial_strength = battle.get_player().battle_info.get_strength();
        assert_eq!(initial_strength, 0);

        // Play Inflame first to gain some Strength
        let inflame_idx = 1; // Inflame is at index 1 in the deck [limit_break(), inflame()]
        let result = battle.play_card(inflame_idx, Entity::Player);
        assert!(result.is_ok());

        let strength_after_inflame = battle.get_player().battle_info.get_strength();
        assert_eq!(strength_after_inflame, 2);

        // Play Limit Break
        let limit_break_idx = 0; // Now Limit Break is at index 0
        let result = battle.play_card(limit_break_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Strength was doubled (2 * 2 = 4)
        let final_strength = battle.get_player().battle_info.get_strength();
        assert_eq!(final_strength, 4);

        // Verify Limit Break was exhausted
        let exhausted_cards = battle.cards.get_exhausted();
        assert!(exhausted_cards.iter().any(|card| card.get_name() == "Limit Break"));
    }

    #[test]
    fn test_limit_break_multiple_uses() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::gremlin_nob::GremlinNob;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::inflame::inflame;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];

        // Create battle with multiple Limit Break cards
        let deck = Deck::new(vec![inflame(), limit_break(), limit_break()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player enough energy
        battle.get_player_mut().battle_info.energy = 6;

        // Play Inflame to get initial Strength
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());
        assert_eq!(battle.get_player().battle_info.get_strength(), 2);

        // Play first Limit Break
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());
        assert_eq!(battle.get_player().battle_info.get_strength(), 4);

        // Play second Limit Break
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());
        assert_eq!(battle.get_player().battle_info.get_strength(), 8);

        // Verify both Limit Break cards were exhausted
        let exhausted_cards = battle.cards.get_exhausted();
        let limit_break_count = exhausted_cards.iter()
            .filter(|card| card.get_name() == "Limit Break")
            .count();
        assert_eq!(limit_break_count, 2);
    }

    #[test]
    fn test_limit_break_upgraded_no_exhaust() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::gremlin_nob::GremlinNob;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::inflame::inflame;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];

        // Create battle with Limit Break+ and Inflame in hand
        let deck = Deck::new(vec![limit_break_upgraded(), inflame()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player enough energy for both cards
        battle.get_player_mut().battle_info.energy = 2;

        // Play Inflame first
        let result = battle.play_card(1, Entity::Player); // Inflame is at index 1
        assert!(result.is_ok());
        assert_eq!(battle.get_player().battle_info.get_strength(), 2);

        // Play Limit Break+ (costs 1 energy)
        let initial_energy = battle.get_player().get_energy();
        let result = battle.play_card(0, Entity::Player); // Limit Break+ is now at index 0
        assert!(result.is_ok());

        // Energy should be spent (Limit Break+ costs 1)
        assert_eq!(battle.get_player().get_energy(), initial_energy - 1);

        // Verify Strength was doubled
        assert_eq!(battle.get_player().battle_info.get_strength(), 4);

        // Verify Limit Break+ was NOT exhausted (goes to discard pile)
        let exhausted_cards = battle.cards.get_exhausted();
        let limit_break_exhausted = exhausted_cards.iter()
            .any(|card| card.get_name() == "Limit Break+");
        assert!(!limit_break_exhausted, "Limit Break+ should not exhaust");

        // Should be in discard pile instead
        let discard = battle.cards.get_discard_pile();
        let limit_break_in_discard = discard.iter()
            .any(|card| card.get_name() == "Limit Break+");
        assert!(limit_break_in_discard, "Limit Break+ should be in discard pile");
    }
}