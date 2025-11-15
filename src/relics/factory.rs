use crate::game::game_event::GameEventListener;
use crate::relics::BurningBloodRelic;

pub fn create_burning_blood_relic() -> Box<dyn GameEventListener> {
    Box::new(BurningBloodRelic::new())
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
}