use crate::game::game_event::GameEventListener;
use crate::relics::{BurningBloodRelic, AnchorRelic, BloodVialRelic};
use crate::battle::events::EventListener;
use crate::battle::target::Entity;

pub fn create_burning_blood_relic() -> Box<dyn GameEventListener> {
    Box::new(BurningBloodRelic::new())
}

pub fn create_anchor_relic() -> Box<dyn EventListener> {
    Box::new(AnchorRelic::new(Entity::Player))
}

pub fn create_blood_vial_relic() -> Box<dyn EventListener> {
    Box::new(BloodVialRelic::new(Entity::Player))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::game_event::GameEvent;

    #[test]
    fn test_relic_factory_creates_burning_blood() {
        let mut relic = create_burning_blood_relic();

        // Should be active initially
        assert!(relic.is_active());

        // Should respond to combat victory
        let effects = relic.on_game_event(&GameEvent::CombatVictory);
        assert_eq!(effects.len(), 1);

        // Should be inactive after use
        assert!(!relic.is_active());
    }

    #[test]
    fn test_relic_factory_creates_anchor() {
        let mut relic = create_anchor_relic();

        // Should be active initially
        assert!(relic.is_active());
        assert_eq!(relic.get_owner(), Entity::Player);

        // Should be EventListener type
        // (Cannot test further without BattleEvent context)
    }

    #[test]
    fn test_relic_factory_creates_blood_vial() {
        let mut relic = create_blood_vial_relic();

        // Should be active initially
        assert!(relic.is_active());
        assert_eq!(relic.get_owner(), Entity::Player);

        // Should be EventListener type
        // (Cannot test further without BattleEvent context)
    }
}