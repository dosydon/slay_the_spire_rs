use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::Effect;
use crate::battle::target::Entity;

/// Orichalcum - If you end your turn without Block, gain 6 Block
pub struct OrichalcumRelic {
    owner: Entity,
}

impl OrichalcumRelic {
    pub fn new(owner: Entity) -> Self {
        Self { owner }
    }
}

impl EventListener for OrichalcumRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::EndOfTurn { entity } if *entity == self.owner => {
                // The relic itself doesn't know current block, so it always tries to gain block
                // The battle system should check if player has block and prevent this if they do
                vec![Effect::GainDefense { amount: 6 }]
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
    fn test_orichalcum_creation() {
        let orichalcum = OrichalcumRelic::new(Entity::Player);
        assert_eq!(orichalcum.owner, Entity::Player);
    }

    #[test]
    fn test_orichalcum_gains_block_at_end_of_turn() {
        let mut orichalcum = OrichalcumRelic::new(Entity::Player);

        let effects = orichalcum.on_event(&BattleEvent::EndOfTurn {
            entity: Entity::Player,
        });

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], Effect::GainDefense { amount: 6 }));
    }

    #[test]
    fn test_orichalcum_no_trigger_for_enemy_turn_end() {
        let mut orichalcum = OrichalcumRelic::new(Entity::Player);

        let effects = orichalcum.on_event(&BattleEvent::EndOfTurn {
            entity: Entity::Enemy(0),
        });

        assert_eq!(effects.len(), 0);
    }
}
