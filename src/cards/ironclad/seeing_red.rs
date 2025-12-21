use crate::game::{card::{Card, Rarity}, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// Seeing Red - Gain 2 Energy. Exhaust.
pub fn seeing_red() -> Card {
    Card::new_with_condition(
        CardEnum::SeeingRed,
        1,
        CardType::Skill,
        vec![Effect::GainEnergy { amount: 2 }, Effect::Exhaust],
        false, // not upgraded
        Condition::True,
        Rarity::Uncommon)
}

/// Seeing Red+ (Upgraded version) - Costs 0
pub fn seeing_red_upgraded() -> Card {
    Card::new_with_condition(
        CardEnum::SeeingRed,
        0, // Cost reduced from 1 to 0
        CardType::Skill,
        vec![Effect::GainEnergy { amount: 2 }, Effect::Exhaust],
        true,  // upgraded
        Condition::True,
        Rarity::Uncommon)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, target::Entity, enemy_in_battle::EnemyInBattle};
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};

    #[test]
    fn test_seeing_red_card_creation() {
        let seeing_red_card = seeing_red();
        assert_eq!(seeing_red_card.get_name(), "Seeing Red");
        assert_eq!(seeing_red_card.get_cost(), 1);
        assert_eq!(seeing_red_card.get_card_type(), &CardType::Skill);
        assert!(!seeing_red_card.is_upgraded());

        let effects = seeing_red_card.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::GainEnergy { amount: 2 })));
        assert!(effects.iter().any(|e| matches!(e, Effect::Exhaust)));
    }

    #[test]
    fn test_seeing_red_upgraded_card_creation() {
        let seeing_red_plus = seeing_red_upgraded();
        assert_eq!(seeing_red_plus.get_name(), "Seeing Red+");
        assert_eq!(seeing_red_plus.get_cost(), 0);
        assert_eq!(seeing_red_plus.get_card_type(), &CardType::Skill);
        assert!(seeing_red_plus.is_upgraded());

        let effects = seeing_red_plus.get_effects();
        assert!(effects.iter().any(|e| matches!(e, Effect::GainEnergy { amount: 2 })));
        assert!(effects.iter().any(|e| matches!(e, Effect::Exhaust)));
    }

    #[test]
    fn test_seeing_red_card_enum() {
        let seeing_red_card = seeing_red();
        let card_enum = seeing_red_card.get_card_enum();
        assert!(matches!(card_enum, CardEnum::SeeingRed));
    }

    #[test]
    fn test_seeing_red_gain_energy() {
        // Create a battle with Seeing Red
        let deck_cards = vec![
            seeing_red(),
            crate::cards::ironclad::strike::strike(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let player_state = crate::game::player_run_state::PlayerRunState::new(100, 100, 0);
let mut battle = Battle::new_with_shuffle(deck, global_info, player_state, vec![enemy], &mut rng);

        // Find Seeing Red in hand
        let seeing_red_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Seeing Red")
            .expect("Seeing Red should be in hand");

        let initial_energy = battle.get_player().get_energy();
        let initial_hand_size = battle.cards.hand_size();
        let initial_exhausted_size = battle.cards.exhausted_size();

        // Play Seeing Red
        let result = battle.play_card(seeing_red_idx, Entity::Player);
        assert!(result.is_ok(), "Seeing Red should be playable");

        // Check that player gained 2 energy but spent 1 to play the card (net gain: +1)
        let final_energy = battle.get_player().get_energy();
        assert_eq!(final_energy, initial_energy + 2 - 1, "Player should have net gained 1 energy (gained 2, spent 1)");

        // Check that card was exhausted (removed from hand, not in discard)
        assert_eq!(battle.cards.hand_size(), initial_hand_size - 1, "Card should be removed from hand");
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted_size + 1, "Card should be in exhausted pile");

        // Check that the exhausted card is Seeing Red
        let exhausted_cards = battle.cards.get_exhausted();
        assert_eq!(exhausted_cards.last().unwrap().get_name(), "Seeing Red");
    }

    #[test]
    fn test_seeing_red_upgraded_zero_cost() {
        // Create a battle with Seeing Red+
        let deck_cards = vec![
            seeing_red_upgraded(),
            crate::cards::ironclad::strike::strike(),
        ];
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let player_state = crate::game::player_run_state::PlayerRunState::new(100, 100, 0);
let mut battle = Battle::new_with_shuffle(deck, global_info, player_state, vec![enemy], &mut rng);

        // Find Seeing Red+ in hand
        let seeing_red_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Seeing Red+")
            .expect("Seeing Red+ should be in hand");

        let initial_energy = battle.get_player().get_energy();

        // Play Seeing Red+
        let result = battle.play_card(seeing_red_idx, Entity::Player);
        assert!(result.is_ok(), "Seeing Red+ should be playable");

        // Check that player still gained 2 energy but spent 0 energy (net gain)
        let final_energy = battle.get_player().get_energy();
        assert_eq!(final_energy, initial_energy + 2, "Player should have gained 2 energy with 0 cost card");

        // Verify the card is exhausted
        let exhausted_cards = battle.cards.get_exhausted();
        assert_eq!(exhausted_cards.last().unwrap().get_name(), "Seeing Red+");
    }

    #[test]
    fn test_seeing_red_energy_cost() {
        let seeing_red_card = seeing_red();
        assert_eq!(seeing_red_card.get_cost(), 1, "Seeing Red should cost 1 energy");

        let seeing_red_plus = seeing_red_upgraded();
        assert_eq!(seeing_red_plus.get_cost(), 0, "Seeing Red+ should cost 0 energy");
    }
}
