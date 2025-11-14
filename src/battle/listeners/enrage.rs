use crate::battle::{events::{BattleEvent, EventListener}, target::Entity};
use crate::game::effect::Effect;

#[derive(Debug)]
pub struct EnrageListener {
    enrage_amount: u32,
    owner: Entity,
}

impl EnrageListener {
    pub(in crate::battle) fn new(owner: Entity, enrage_amount: u32) -> Self {
        EnrageListener {
            enrage_amount,
            owner,
        }
    }
}

impl EventListener for EnrageListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::SkillCardPlayed { source } if *source == Entity::Player => {
                // When the player plays a Skill card, the Gremlin Nob (this listener's owner) gains Strength
                vec![Effect::GainStrength(self.enrage_amount)]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true // Enrage is always active
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }
}

#[cfg(test)]
mod enrage_tests {
    use super::*;

    #[test]
    fn test_enrage_listener_creation() {
        let listener = EnrageListener::new(Entity::Enemy(0), 2);
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Enemy(0));
        assert_eq!(listener.enrage_amount, 2);
    }

    #[test]
    fn test_enrage_triggers_on_skill_card() {
        let mut listener = EnrageListener::new(Entity::Enemy(0), 2);

        let skill_event = BattleEvent::SkillCardPlayed {
            source: Entity::Player,
        };

        let effects = listener.on_event(&skill_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::GainStrength(2));
        assert!(listener.is_active()); // Still active after triggering
    }

    #[test]
    fn test_enrage_does_not_trigger_on_damage() {
        let mut listener = EnrageListener::new(Entity::Enemy(0), 2);

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 6,
            source: Entity::Player,
        };

        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active());
    }

    #[test]
    fn test_enrage_triggers_multiple_times() {
        let mut listener = EnrageListener::new(Entity::Enemy(0), 3);

        let skill_event = BattleEvent::SkillCardPlayed {
            source: Entity::Player,
        };

        // First skill card
        let effects1 = listener.on_event(&skill_event);
        assert_eq!(effects1.len(), 1);
        assert_eq!(effects1[0], Effect::GainStrength(3));

        // Second skill card should also trigger
        let effects2 = listener.on_event(&skill_event);
        assert_eq!(effects2.len(), 1);
        assert_eq!(effects2[0], Effect::GainStrength(3));

        assert!(listener.is_active()); // Always active
    }

    #[test]
    fn test_enrage_different_amounts() {
        let mut listener_2 = EnrageListener::new(Entity::Enemy(0), 2);
        let mut listener_3 = EnrageListener::new(Entity::Enemy(1), 3);

        let skill_event = BattleEvent::SkillCardPlayed {
            source: Entity::Player,
        };

        let effects_2 = listener_2.on_event(&skill_event);
        assert_eq!(effects_2, vec![Effect::GainStrength(2)]);

        let effects_3 = listener_3.on_event(&skill_event);
        assert_eq!(effects_3, vec![Effect::GainStrength(3)]);
    }
}