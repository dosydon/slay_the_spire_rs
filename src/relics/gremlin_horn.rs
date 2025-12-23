use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;

/// Gremlin Horn - Whenever an enemy dies, gain 1 Energy and draw 1 card
pub struct GremlinHornRelic {
    owner: Entity,
}

impl GremlinHornRelic {
    pub fn new(owner: Entity) -> Self {
        Self { owner }
    }
}

impl EventListener for GremlinHornRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::EnemyDeath { enemy } => {
                // Trigger on any enemy death (regardless of who killed it)
                vec![
                    BattleEffect::GainEnergy { amount: 1 },
                    BattleEffect::DrawCard { count: 1 },
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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gremlin_horn_creation() {
        let horn = GremlinHornRelic::new(Entity::Player);
        assert_eq!(horn.owner, Entity::Player);
    }

    #[test]
    fn test_gremlin_horn_triggers_on_enemy_death() {
        let mut horn = GremlinHornRelic::new(Entity::Player);

        let effects = horn.on_event(&BattleEvent::EnemyDeath {
            enemy: Entity::Enemy(0),
        });

        assert_eq!(effects.len(), 2);
        assert!(matches!(effects[0], BattleEffect::GainEnergy { amount: 1 }));
        assert!(matches!(effects[1], BattleEffect::DrawCard { count: 1 }));
    }

    #[test]
    fn test_gremlin_horn_gives_energy_and_draw() {
        let mut horn = GremlinHornRelic::new(Entity::Player);

        let effects = horn.on_event(&BattleEvent::EnemyDeath {
            enemy: Entity::Enemy(0),
        });

        assert_eq!(effects.len(), 2);
        // Check both effects are present
        let has_energy = effects.iter().any(|e| matches!(e, BattleEffect::GainEnergy { amount: 1 }));
        let has_draw = effects.iter().any(|e| matches!(e, BattleEffect::DrawCard { count: 1 }));
        assert!(has_energy);
        assert!(has_draw);
    }

    #[test]
    fn test_gremlin_horn_triggers_for_each_enemy() {
        let mut horn = GremlinHornRelic::new(Entity::Player);

        // First enemy dies
        let effects1 = horn.on_event(&BattleEvent::EnemyDeath {
            enemy: Entity::Enemy(0),
        });
        assert_eq!(effects1.len(), 2);

        // Second enemy dies
        let effects2 = horn.on_event(&BattleEvent::EnemyDeath {
            enemy: Entity::Enemy(1),
        });
        assert_eq!(effects2.len(), 2);
    }
}
