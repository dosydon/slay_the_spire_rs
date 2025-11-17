use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::{Effect, Condition}};
use crate::battle::{events::{BattleEvent, EventListener}, target::Entity};

/// Brutality Power Listener
/// At the start of your turn, lose 1 HP and draw 1 card
#[derive(Debug)]
pub struct BrutalityListener {
    owner: Entity,
}

impl BrutalityListener {
    pub fn new(owner: Entity) -> Self {
        BrutalityListener {
            owner,
        }
    }
}

impl EventListener for BrutalityListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::TurnStart { entity } if *entity == self.owner => {
                // At the start of owner's turn, lose 1 HP and draw 1 card
                vec![
                    Effect::LoseHp(1),
                    Effect::DrawCard { count: 1 },
                ]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }
}

/// Brutality - Rare Power Card
/// Cost: 0
/// Effect: At the start of your turn, lose 1 HP and draw 1 card.
pub fn brutality() -> Card {
    Card::new(CardEnum::Brutality, 0, CardType::Power, vec![
        Effect::ActivateBrutality,
    ], false, true)
}

/// Brutality+ (Upgraded)
/// Cost: 0
/// Effect: At the start of your turn, lose 1 HP and draw 1 card.
pub fn brutality_upgraded() -> Card {
    Card::new(CardEnum::Brutality, 0, CardType::Power, vec![
        Effect::ActivateBrutality,
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brutality_creation() {
        let card = brutality();

        assert_eq!(card.get_name(), "Brutality");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::ActivateBrutality);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_brutality_upgraded_creation() {
        let card = brutality_upgraded();

        assert_eq!(card.get_name(), "Brutality+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::ActivateBrutality);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_brutality_listener_functionality() {
        let mut listener = BrutalityListener::new(Entity::Player);

        // Test turn start event for player
        let turn_start_event = BattleEvent::TurnStart { entity: Entity::Player };
        let effects = listener.on_event(&turn_start_event);

        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0], Effect::LoseHp(1));
        assert_eq!(effects[1], Effect::DrawCard { count: 1 });

        // Test turn start event for enemy (should not trigger)
        let enemy_turn_start = BattleEvent::TurnStart { entity: Entity::Enemy(0) };
        let effects = listener.on_event(&enemy_turn_start);
        assert_eq!(effects.len(), 0);

        // Test other events (should not trigger)
        let other_event = BattleEvent::EndOfTurn { entity: Entity::Player };
        let effects = listener.on_event(&other_event);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_brutality_listener_properties() {
        let listener = BrutalityListener::new(Entity::Player);

        // Test basic properties
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Player);
    }
}