use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};
use crate::battle::{events::{BattleEvent, EventListener}, target::Entity};

/// Embrace Power Listener
/// Whenever a card is exhausted, draw 1 card
#[derive(Debug)]
pub struct EmbraceListener {
    owner: Entity,
}

impl EmbraceListener {
    pub fn new(owner: Entity) -> Self {
        EmbraceListener {
            owner,
        }
    }
}

impl EventListener for EmbraceListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::CardExhausted { source } if *source == self.owner => {
                // When the owner exhausts a card, draw 1 card
                vec![Effect::DrawCard { count: 1 }]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true // Embrace is always active once played
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }
}

/// Embrace - Power Card
/// Cost: 2 (1 when upgraded)
/// Effect: Whenever you Exhaust a card, draw 1 card.
pub fn embrace() -> Card {
    Card::new_with_condition(CardEnum::Embrace, 2, CardType::Power, vec![
        Effect::ActivateEmbrace,
    ], false, Condition::True)
}

pub fn embrace_upgraded() -> Card {
    Card::new_with_condition(CardEnum::Embrace, 1, CardType::Power, vec![
        Effect::ActivateEmbrace,
    ], true, Condition::True)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embrace_creation() {
        let card = embrace();

        assert_eq!(card.get_name(), "Embrace");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::ActivateEmbrace);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_embrace_upgraded_creation() {
        let card = embrace_upgraded();

        assert_eq!(card.get_name(), "Embrace+");
        assert_eq!(card.get_cost(), 1);  // Upgraded cost is 1
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::ActivateEmbrace);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_embrace_power_activation() {
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

        // Create battle with Embrace in hand
        let deck = Deck::new(vec![embrace(), strike()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Play Embrace (power card targets player)
        let embrace_idx = 0;
        let result = battle.play_card(embrace_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Embrace was added to powers collection
        let powers = battle.get_powers();
        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].get_name(), "Embrace");

        // Verify Embrace did NOT go to discard pile (power cards stay in play)
        let discard = battle.cards.get_discard_pile();
        assert!(!discard.iter().any(|card| card.get_name() == "Embrace"));
    }

    #[test]
    fn test_embrace_listener_creation() {
        let listener = EmbraceListener::new(Entity::Player);
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Player);
    }

    #[test]
    fn test_embrace_triggers_on_card_exhausted() {
        let mut listener = EmbraceListener::new(Entity::Player);

        let exhaust_event = BattleEvent::CardExhausted {
            source: Entity::Player,
        };

        let effects = listener.on_event(&exhaust_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::DrawCard { count: 1 });
        assert!(listener.is_active()); // Still active after triggering
    }

    #[test]
    fn test_embrace_does_not_trigger_on_other_events() {
        let mut listener = EmbraceListener::new(Entity::Player);

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Player,
            amount: 6,
            source: Entity::Enemy(0),
        };

        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active());
    }

    #[test]
    fn test_embrace_triggers_multiple_times() {
        let mut listener = EmbraceListener::new(Entity::Player);

        let exhaust_event = BattleEvent::CardExhausted {
            source: Entity::Player,
        };

        // First exhaust
        let effects1 = listener.on_event(&exhaust_event);
        assert_eq!(effects1.len(), 1);
        assert_eq!(effects1[0], Effect::DrawCard { count: 1 });

        // Second exhaust should also trigger
        let effects2 = listener.on_event(&exhaust_event);
        assert_eq!(effects2.len(), 1);
        assert_eq!(effects2[0], Effect::DrawCard { count: 1 });

        assert!(listener.is_active()); // Always active
    }

    #[test]
    fn test_embrace_only_triggers_for_owner() {
        let mut listener = EmbraceListener::new(Entity::Player);

        // Enemy exhausting a card should not trigger
        let enemy_exhaust_event = BattleEvent::CardExhausted {
            source: Entity::Enemy(0),
        };

        let effects = listener.on_event(&enemy_exhaust_event);
        assert_eq!(effects.len(), 0);

        // Player exhausting should trigger
        let player_exhaust_event = BattleEvent::CardExhausted {
            source: Entity::Player,
        };

        let effects = listener.on_event(&player_exhaust_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::DrawCard { count: 1 });
    }

    #[test]
    fn test_embrace_with_ethereal_card_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::battle::action::Action;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike;
        use crate::cards::ironclad::carnage::carnage;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Embrace, Carnage (ethereal), and Strike in hand
        let deck = Deck::new(vec![embrace(), carnage(), strike()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Play Embrace to activate the power
        let embrace_idx = 0;
        let result = battle.play_card(embrace_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Embrace is active
        let powers = battle.get_powers();
        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].get_name(), "Embrace");

        // End turn without playing Carnage to trigger ethereal exhaustion
        let initial_hand_size = battle.cards.hand_size();
        let initial_exhausted_size = battle.cards.exhausted_size();

        // Should have Carnage and Strike in hand (Embrace was played)
        assert_eq!(initial_hand_size, 2);

        // Process end of turn using public eval_action method
        let result = battle.eval_action(Action::EndTurn, &mut rng);
        assert!(result.is_ok());

        // Verify Carnage was exhausted (should be in exhausted pile)
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted_size + 1);

        // Verify the exhausted card is Carnage
        let exhausted_pile = battle.cards.get_exhausted();
        assert_eq!(exhausted_pile.len(), 1);
        assert_eq!(exhausted_pile[0].get_name(), "Carnage");

        // The CardExhausted event should have been triggered and Embrace should have drawn a card
        // This card would be in the hand for the next turn, but since we're at the end of turn,
        // we need to check if the draw effect was processed
        // The exact behavior depends on when draw effects are processed in the turn flow
    }

    #[test]
    fn test_embrace_with_ethereal_card_played_before_exhaustion() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike;
        use crate::cards::ironclad::carnage::carnage;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Embrace+, Carnage (ethereal), and 2 Strikes in hand
        let deck = Deck::new(vec![embrace_upgraded(), carnage(), strike(), strike()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Play Embrace+ to activate the power (costs 1 energy)
        let embrace_idx = 0;
        let result = battle.play_card(embrace_idx, Entity::Player);
        assert!(result.is_ok());

        // Play Carnage (it goes to discard pile, not exhausted)
        let carnage_idx = 0; // Now at index 0 after Embrace was played
        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let result = battle.play_card(carnage_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify Carnage dealt damage and went to discard
        let enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let expected_hp = if initial_enemy_hp > 20 { initial_enemy_hp - 20 } else { 0 };
        assert_eq!(enemy_hp, expected_hp);

        let discard = battle.cards.get_discard_pile();
        assert_eq!(discard.len(), 1);
        assert_eq!(discard[0].get_name(), "Carnage");

        // Embrace should not trigger since Carnage was played, not exhausted
        let exhausted_pile = battle.cards.get_exhausted();
        assert_eq!(exhausted_pile.len(), 0);
    }
}
