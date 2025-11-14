use crate::battle::{events::{BattleEvent, EventListener}, target::Entity};
use crate::game::effect::Effect;

#[derive(Debug)]
pub struct LoseStrengthListener {
    amount_to_lose: u32,
    owner: Entity,
    is_active: bool,
}

impl LoseStrengthListener {
    pub(in crate::battle) fn new(owner: Entity, amount_to_lose: u32) -> Self {
        LoseStrengthListener {
            amount_to_lose,
            owner,
            is_active: true,
        }
    }
}

impl EventListener for LoseStrengthListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::EndOfTurn { entity } if *entity == self.owner && self.is_active => {
                self.is_active = false; // Only trigger once
                vec![Effect::LoseStrength(self.amount_to_lose)]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }
}

#[cfg(test)]
mod lose_strength_tests {
    use super::*;

    #[test]
    fn test_lose_strength_listener_creation() {
        let listener = LoseStrengthListener::new(Entity::Player, 3);
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Player);
    }

    #[test]
    fn test_lose_strength_triggers_on_end_of_turn() {
        let mut listener = LoseStrengthListener::new(Entity::Player, 2);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Player,
        };

        let effects = listener.on_event(&end_turn_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::LoseStrength(2));
        assert!(!listener.is_active()); // Used up
    }

    #[test]
    fn test_lose_strength_only_triggers_once() {
        let mut listener = LoseStrengthListener::new(Entity::Enemy(0), 1);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Enemy(0),
        };

        // First end turn triggers strength loss
        let effects1 = listener.on_event(&end_turn_event);
        assert_eq!(effects1.len(), 1);
        assert_eq!(effects1[0], Effect::LoseStrength(1));

        // Second end turn should not trigger
        let effects2 = listener.on_event(&end_turn_event);
        assert_eq!(effects2.len(), 0);
    }

    #[test]
    fn test_lose_strength_wrong_entity() {
        let mut listener = LoseStrengthListener::new(Entity::Enemy(0), 2);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Enemy(1), // Different enemy
        };

        let effects = listener.on_event(&end_turn_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active()); // Still active
    }

    #[test]
    fn test_lose_strength_different_amounts() {
        let mut listener_2 = LoseStrengthListener::new(Entity::Player, 2);
        let mut listener_5 = LoseStrengthListener::new(Entity::Player, 5);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Player,
        };

        let effects_2 = listener_2.on_event(&end_turn_event);
        assert_eq!(effects_2, vec![Effect::LoseStrength(2)]);

        let effects_5 = listener_5.on_event(&end_turn_event);
        assert_eq!(effects_5, vec![Effect::LoseStrength(5)]);
    }
}