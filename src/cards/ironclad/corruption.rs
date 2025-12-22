use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum, card::Rarity};

/// Corruption - Power Card
/// Cost: 3 (2 when upgraded)
/// Effect: Skills cost 0. Whenever you play a Skill, Exhaust it.
pub fn corruption() -> Card {
    Card::new(CardEnum::Corruption, 3, CardType::Power, vec![
        Effect::ActivateCorruption,
    ], Rarity::Rare)
        .set_play_condition(Condition::True)
}

pub fn corruption_upgraded() -> Card {
    Card::new(CardEnum::Corruption, 2, CardType::Power, vec![
        Effect::ActivateCorruption,
    ], Rarity::Rare)
        .set_upgraded(true)
        .set_play_condition(Condition::True)
}

#[cfg(test)]
mod tests {
    use crate::game::PlayerRunState;

    use super::*;

    #[test]
    fn test_corruption_creation() {
        let card = corruption();

        assert_eq!(card.get_name(), "Corruption");
        assert_eq!(card.get_cost(), 3);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::ActivateCorruption);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_corruption_upgraded_creation() {
        let card = corruption_upgraded();

        assert_eq!(card.get_name(), "Corruption+");
        assert_eq!(card.get_cost(), 2);  // Upgraded cost is 2
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::ActivateCorruption);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }


    #[test]
    fn test_corruption_power_activation() {
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

        // Create battle with Corruption in hand
        let deck = Deck::new(vec![corruption()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Play Corruption (power card targets player)
        let corruption_idx = 0;
        let result = battle.play_card(corruption_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Corruption was added to powers collection
        let powers = battle.get_powers();
        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].get_name(), "Corruption");

        // Verify Corruption did NOT go to discard pile (power cards stay in play)
        let discard = battle.cards.get_discard_pile();
        assert!(!discard.iter().any(|card| card.get_name() == "Corruption"));
    }

    #[test]
    fn test_corruption_cost_reduction() {
        let corruption_card = corruption();
        assert_eq!(corruption_card.get_cost(), 3, "Corruption should cost 3 energy");

        let corruption_plus = corruption_upgraded();
        assert_eq!(corruption_plus.get_cost(), 2, "Corruption+ should cost 2 energy");
    }

    #[test]
    fn test_corruption_makes_skills_cost_zero() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::defend::defend;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Corruption and Defend in hand
        let deck = Deck::new(vec![corruption(), defend()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0)  , enemies, &mut rng);

        // Give player enough energy to play both cards
        battle.get_player_mut().battle_info.energy = 5;

        // Verify Defend normally costs 1 energy before Corruption
        let hand = battle.get_hand();
        let defend_card = &hand[1]; // Defend is at index 1
        assert_eq!(defend_card.get_cost(), 1);
        assert_eq!(battle.get_modified_cost(defend_card), 1); // No Corruption yet

        // Play Corruption (power card targets player)
        let corruption_idx = 0;
        let initial_energy = battle.get_player().get_energy();
        let result = battle.play_card(corruption_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Corruption was activated and energy was spent
        assert_eq!(battle.get_player().get_energy(), initial_energy - 3);
        assert!(battle.has_corruption_active());

        // Check what's in hand after playing Corruption
        let hand = battle.get_hand();
        assert_eq!(hand.len(), 1, "Should have 1 card in hand after playing Corruption");
        
        // Check the card at index 0 before playing
        let card_name = hand[0].get_name();
        let card_cost = hand[0].get_cost();
        let card_modified_cost = battle.get_modified_cost(&hand[0]);
        assert_eq!(card_name, "Defend", "Card should be Defend");
        assert_eq!(card_cost, 1); // Base cost is still 1
        assert_eq!(card_modified_cost, 0); // Modified cost is 0 with Corruption

        // Store player's current block and energy before playing Defend
        let initial_block = battle.get_player().battle_info.get_block();
        let current_energy = battle.get_player().get_energy();
        
        // Debug: Check hand state right before playing Defend
        
        // Try to play Defend at index 0 - it should be exhausted immediately due to Corruption
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok(), "Should be able to play Defend with Corruption active");

        // The play should succeed (card gets exhausted) but energy should not be spent since cost is 0
        let energy_after_play = battle.get_player().get_energy();
        let hand_after = battle.get_hand();
                assert_eq!(energy_after_play, current_energy, "Energy should not be spent when skill costs 0");

        // Verify Defend's effects were applied - player should have gained 5 block
        let final_block = battle.get_player().battle_info.get_block();
        assert_eq!(final_block, initial_block + 5, "Player should have gained 5 block from Defend");

        // Verify Defend was exhausted due to Corruption
        let exhausted_cards = battle.cards.get_exhausted();
        assert!(exhausted_cards.iter().any(|card| card.get_name() == "Defend"));

        // Verify hand is empty after playing the only Defend card
        assert_eq!(hand_after.len(), 0, "Should have 0 cards in hand after Defend is exhausted");
    }

    #[test]
    fn test_corruption_exhausts_multiple_skills() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::defend::defend;
        use crate::cards::ironclad::shrug_it_off::shrug_it_off;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Corruption and multiple Skill cards in hand
        let deck = Deck::new(vec![corruption(), defend(), shrug_it_off(), defend()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Give player enough energy to play all cards
        battle.get_player_mut().battle_info.energy = 10;

        // Play Corruption first
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok(), "Should be able to play Corruption");

        // Verify Corruption is active
        assert!(battle.has_corruption_active());

        // Now play first Defend - should be exhausted
        let initial_exhausted = battle.cards.exhausted_size();
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok(), "Should be able to play Defend");
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted + 1, "Defend should be exhausted");

        // Play Shrug It Off - should also be exhausted
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok(), "Should be able to play Shrug It Off");
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted + 2, "Shrug It Off should be exhausted");

        // Play second Defend - should also be exhausted
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok(), "Should be able to play second Defend");
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted + 3, "Second Defend should be exhausted");

        // Verify all three Skill cards are in the exhausted pile
        let exhausted_cards = battle.cards.get_exhausted();
        let skill_cards_exhausted = exhausted_cards.iter()
            .filter(|card| card.get_card_type() == &crate::game::card_type::CardType::Skill)
            .count();
        assert_eq!(skill_cards_exhausted, 3, "All three Skill cards should be exhausted");
    }

    #[test]
    fn test_corruption_does_not_exhaust_attacks() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Corruption and an Attack card
        let deck = Deck::new(vec![corruption(), strike()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Give player enough energy
        battle.get_player_mut().battle_info.energy = 10;

        // Play Corruption first
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok(), "Should be able to play Corruption");

        // Play Strike - should NOT be exhausted (it's an Attack, not a Skill)
        let initial_exhausted = battle.cards.exhausted_size();
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok(), "Should be able to play Strike");
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted, "Strike should NOT be exhausted");

        // Verify Strike went to discard pile instead
        let discard = battle.cards.get_discard_pile();
        assert!(discard.iter().any(|card| card.get_name() == "Strike"), "Strike should be in discard pile");
    }

    }