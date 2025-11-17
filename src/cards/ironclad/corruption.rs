use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};
use crate::battle::{events::{BattleEvent, EventListener}, target::Entity};

/// Corruption Listener
/// Makes all Skills cost 0 and causes them to be Exhausted when played
#[derive(Debug)]
pub struct CorruptionListener {
    owner: Entity,
}

impl CorruptionListener {
    pub fn new(owner: Entity) -> Self {
        CorruptionListener {
            owner,
        }
    }
}

impl EventListener for CorruptionListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::CardPlayed { source, card_type }
                if *source == self.owner && *card_type == crate::game::card_type::CardType::Skill => {
                // When player plays a Skill, make it get exhausted
                vec![Effect::Exhaust]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true // Corruption is always active once played
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }
}

/// Corruption - Power Card
/// Cost: 3 (2 when upgraded)
/// Effect: Skills cost 0. Whenever you play a Skill, Exhaust it.
pub fn corruption() -> Card {
    Card::new_with_condition(CardEnum::Corruption, 3, CardType::Power, vec![
        Effect::ActivateCorruption,
    ], false, Condition::True)
}

pub fn corruption_upgraded() -> Card {
    Card::new_with_condition(CardEnum::Corruption, 2, CardType::Power, vec![
        Effect::ActivateCorruption,
    ], true, Condition::True)
}

#[cfg(test)]
mod tests {
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
    fn test_corruption_listener_creation() {
        let listener = CorruptionListener::new(Entity::Player);
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Player);
    }

    #[test]
    fn test_corruption_triggers_on_skill_played() {
        let mut listener = CorruptionListener::new(Entity::Player);

        let skill_event = BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: crate::game::card_type::CardType::Skill,
        };

        let effects = listener.on_event(&skill_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::Exhaust);
        assert!(listener.is_active()); // Still active after triggering
    }

    #[test]
    fn test_corruption_does_not_trigger_on_attack_played() {
        let mut listener = CorruptionListener::new(Entity::Player);

        let attack_event = BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: crate::game::card_type::CardType::Attack,
        };

        let effects = listener.on_event(&attack_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active());
    }

    #[test]
    fn test_corruption_does_not_trigger_on_power_played() {
        let mut listener = CorruptionListener::new(Entity::Player);

        let power_event = BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: crate::game::card_type::CardType::Power,
        };

        let effects = listener.on_event(&power_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active());
    }

    #[test]
    fn test_corruption_only_triggers_for_owner() {
        let mut listener = CorruptionListener::new(Entity::Player);

        // Enemy playing a skill should not trigger
        let enemy_skill_event = BattleEvent::CardPlayed {
            source: Entity::Enemy(0),
            card_type: crate::game::card_type::CardType::Skill,
        };

        let effects = listener.on_event(&enemy_skill_event);
        assert_eq!(effects.len(), 0);

        // Player playing a skill should trigger
        let player_skill_event = BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: crate::game::card_type::CardType::Skill,
        };

        let effects = listener.on_event(&player_skill_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::Exhaust);
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
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

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
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

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
        assert!(hand.len() >= 1, "Should have at least 1 card in hand");

        let defend_card = &hand[0]; // Defend should be at index 0
        assert_eq!(defend_card.get_name(), "Defend", "First card should be Defend");
        assert_eq!(defend_card.get_cost(), 1); // Base cost is still 1
        assert_eq!(battle.get_modified_cost(defend_card), 0); // Modified cost is 0 with Corruption

        // The main test: verify that skills cost 0 energy when Corruption is active
        // We can't actually play the card because Corruption exhaustes it immediately,
        // but we can verify the cost calculation works correctly
        let current_energy = battle.get_player().get_energy();

        // Try to play Defend - it should be exhausted immediately due to Corruption
        let result = battle.play_card(0, Entity::Player);

        // The play should succeed (card gets exhausted) but energy should not be spent since cost is 0
        let energy_after_play = battle.get_player().get_energy();
        assert_eq!(energy_after_play, current_energy, "Energy should not be spent when skill costs 0");

        // Verify Defend was exhausted due to Corruption
        let exhausted_cards = battle.cards.get_exhausted();
        assert!(exhausted_cards.iter().any(|card| card.get_name() == "Defend"));
    }

    }