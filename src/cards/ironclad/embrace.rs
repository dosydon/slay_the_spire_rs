use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};
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
                vec![Effect::DrawCard(1)]
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
    Card::new(CardEnum::Embrace, 2, CardType::Power, vec![
        Effect::ActivateEmbrace,
    ], false, true)
}

pub fn embrace_upgraded() -> Card {
    Card::new(CardEnum::Embrace, 1, CardType::Power, vec![
        Effect::ActivateEmbrace,
    ], true, true)
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
        assert_eq!(effects[0], Effect::DrawCard(1));
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
        assert_eq!(effects1[0], Effect::DrawCard(1));

        // Second exhaust should also trigger
        let effects2 = listener.on_event(&exhaust_event);
        assert_eq!(effects2.len(), 1);
        assert_eq!(effects2[0], Effect::DrawCard(1));

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
        assert_eq!(effects[0], Effect::DrawCard(1));
    }
}
