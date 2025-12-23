use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::Effect;
use crate::battle::target::Entity;

/// Happy Flower - Every 3 turns, gain 1 Energy
pub struct HappyFlowerRelic {
    turn_count: u32,
    owner: Entity,
}

impl HappyFlowerRelic {
    pub fn new(owner: Entity) -> Self {
        Self {
            turn_count: 0,
            owner,
        }
    }
}

impl EventListener for HappyFlowerRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::CombatStart { player } if *player == self.owner => {
                self.turn_count = 0;
                vec![]
            }
            BattleEvent::StartOfPlayerTurn if self.owner == Entity::Player => {
                self.turn_count += 1;
                if self.turn_count % 3 == 0 {
                    vec![Effect::GainEnergy { amount: 1 }]
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
    fn test_happy_flower_creation() {
        let flower = HappyFlowerRelic::new(Entity::Player);
        assert_eq!(flower.turn_count, 0);
    }

    #[test]
    fn test_happy_flower_gives_energy_every_3_turns() {
        let mut flower = HappyFlowerRelic::new(Entity::Player);

        // Reset on combat start
        let _ = flower.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Turn 1 - no energy
        let effects1 = flower.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects1.len(), 0);

        // Turn 2 - no energy
        let effects2 = flower.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects2.len(), 0);

        // Turn 3 - gain energy
        let effects3 = flower.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects3.len(), 1);
        assert!(matches!(effects3[0], Effect::GainEnergy { amount: 1 }));
    }

    #[test]
    fn test_happy_flower_resets_on_combat_start() {
        let mut flower = HappyFlowerRelic::new(Entity::Player);

        // First combat
        let _ = flower.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        assert_eq!(flower.turn_count, 0);

        // Take 3 turns
        let _ = flower.on_event(&BattleEvent::StartOfPlayerTurn);
        let _ = flower.on_event(&BattleEvent::StartOfPlayerTurn);
        let _ = flower.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(flower.turn_count, 3);

        // New combat - should reset
        let _ = flower.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        assert_eq!(flower.turn_count, 0);
    }

    #[test]
    fn test_happy_flower_continues_after_3() {
        let mut flower = HappyFlowerRelic::new(Entity::Player);

        let _ = flower.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Turns 1-3 (gain energy on turn 3)
        let e1 = flower.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(e1.len(), 0);
        let e2 = flower.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(e2.len(), 0);
        let e3 = flower.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(e3.len(), 1);

        // Turns 4-6 (gain energy on turn 6)
        let e4 = flower.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(e4.len(), 0);
        let e5 = flower.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(e5.len(), 0);
        let e6 = flower.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(e6.len(), 1);
    }
}
