use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;

/// Mercury Hourglass - At the start of your turn, deal 3 damage to ALL enemies
pub struct MercuryHourglassRelic {
    owner: Entity,
}

impl MercuryHourglassRelic {
    pub fn new(owner: Entity) -> Self {
        Self { owner }
    }
}

impl EventListener for MercuryHourglassRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::StartOfPlayerTurn => {
                // Deal 3 damage to ALL enemies
                vec![BattleEffect::AttackAllEnemies { amount: 3, num_attacks: 1 }]
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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mercury_hourglass_creation() {
        let hourglass = MercuryHourglassRelic::new(Entity::Player);
        assert_eq!(hourglass.owner, Entity::Player);
    }

    #[test]
    fn test_mercury_hourglass_deals_damage_on_turn_start() {
        let mut hourglass = MercuryHourglassRelic::new(Entity::Player);

        let effects = hourglass.on_event(&BattleEvent::StartOfPlayerTurn);

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::AttackAllEnemies { amount: 3, num_attacks: 1 }));
    }

    #[test]
    fn test_mercury_hourglass_triggers_every_turn() {
        let mut hourglass = MercuryHourglassRelic::new(Entity::Player);

        // Turn 1
        let effects1 = hourglass.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects1.len(), 1);

        // Turn 2
        let effects2 = hourglass.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects2.len(), 1);

        // Turn 3
        let effects3 = hourglass.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects3.len(), 1);
    }
}
