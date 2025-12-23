use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::Effect;
use crate::battle::target::Entity;

/// Bag of Preparation - At the start of each combat, draw 2 additional cards
pub struct BagOfPreparationRelic {
    owner: Entity,
}

impl BagOfPreparationRelic {
    pub fn new(owner: Entity) -> Self {
        Self { owner }
    }
}

impl EventListener for BagOfPreparationRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::CombatStart { player } if *player == self.owner => {
                vec![Effect::DrawCard { count: 2 }]
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
    fn test_bag_of_preparation_creation() {
        let bag = BagOfPreparationRelic::new(Entity::Player);
        assert_eq!(bag.owner, Entity::Player);
    }

    #[test]
    fn test_bag_of_preparation_draws_on_combat_start() {
        let mut bag = BagOfPreparationRelic::new(Entity::Player);

        let effects = bag.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], Effect::DrawCard { count: 2 }));
    }

    #[test]
    fn test_bag_of_preparation_no_trigger_for_enemy_combat_start() {
        let mut bag = BagOfPreparationRelic::new(Entity::Player);

        let effects = bag.on_event(&BattleEvent::CombatStart {
            player: Entity::Enemy(0),
        });

        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_bag_of_preparation_no_trigger_on_other_events() {
        let mut bag = BagOfPreparationRelic::new(Entity::Player);

        let effects = bag.on_event(&BattleEvent::StartOfPlayerTurn);

        assert_eq!(effects.len(), 0);
    }
}
