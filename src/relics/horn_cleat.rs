use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::Effect;
use crate::battle::target::Entity;

/// Horn Cleat - At the start of your 2nd turn, gain 14 Block
pub struct HornCleatRelic {
    turn_count: u8,
    owner: Entity,
}

impl HornCleatRelic {
    pub fn new(owner: Entity) -> Self {
        Self {
            turn_count: 0,
            owner,
        }
    }
}

impl EventListener for HornCleatRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::StartOfPlayerTurn => {
                self.turn_count += 1;
                // Only trigger on turn 2
                if self.turn_count == 2 {
                    vec![Effect::GainDefense { amount: 14 }]
                } else {
                    vec![]
                }
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
    fn test_horn_cleat_creation() {
        let horn_cleat = HornCleatRelic::new(Entity::Player);
        assert_eq!(horn_cleat.turn_count, 0);
    }

    #[test]
    fn test_horn_cleat_triggers_on_turn_2() {
        let mut horn_cleat = HornCleatRelic::new(Entity::Player);

        // Turn 1 - should not trigger
        let effects1 = horn_cleat.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects1.len(), 0);
        assert_eq!(horn_cleat.turn_count, 1);

        // Turn 2 - should trigger
        let effects2 = horn_cleat.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects2.len(), 1);
        assert!(matches!(effects2[0], Effect::GainDefense { amount: 14 }));
        assert_eq!(horn_cleat.turn_count, 2);
    }

    #[test]
    fn test_horn_cleat_only_triggers_once() {
        let mut horn_cleat = HornCleatRelic::new(Entity::Player);

        // Turn 1
        let _ = horn_cleat.on_event(&BattleEvent::StartOfPlayerTurn);
        // Turn 2
        let effects1 = horn_cleat.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects1.len(), 1);

        // Turn 3
        let effects2 = horn_cleat.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects2.len(), 0);
    }
}
