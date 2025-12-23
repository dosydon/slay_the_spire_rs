use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;

/// Centennial Puzzle - The first time you lose HP each combat, draw 3 cards
pub struct CentennialPuzzleRelic {
    triggered: bool,
    owner: Entity,
}

impl CentennialPuzzleRelic {
    pub fn new(owner: Entity) -> Self {
        Self {
            triggered: false,
            owner,
        }
    }
}

impl EventListener for CentennialPuzzleRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CombatStart { player } if *player == self.owner => {
                self.triggered = false;
                vec![]
            }
            BattleEvent::HpLostFromCard { target, .. } if *target == self.owner && !self.triggered => {
                self.triggered = true;
                vec![BattleEffect::DrawCard { count: 3 }]
            }
            BattleEvent::DamageTaken { target, amount, .. } if *target == self.owner && *amount > 0 && !self.triggered => {
                self.triggered = true;
                vec![BattleEffect::DrawCard { count: 3 }]
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
    fn test_centennial_puzzle_creation() {
        let puzzle = CentennialPuzzleRelic::new(Entity::Player);
        assert!(!puzzle.triggered);
    }

    #[test]
    fn test_centennial_puzzle_draws_on_first_damage() {
        let mut puzzle = CentennialPuzzleRelic::new(Entity::Player);

        // Reset on combat start
        let _ = puzzle.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Take damage
        let effects = puzzle.on_event(&BattleEvent::DamageTaken {
            target: Entity::Player,
            amount: 5,
            source: Entity::Enemy(0),
        });

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::DrawCard { count: 3 }));
        assert!(puzzle.triggered);
    }

    #[test]
    fn test_centennial_puzzle_only_triggers_once_per_combat() {
        let mut puzzle = CentennialPuzzleRelic::new(Entity::Player);

        let _ = puzzle.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // First damage - should trigger
        let effects1 = puzzle.on_event(&BattleEvent::DamageTaken {
            target: Entity::Player,
            amount: 5,
            source: Entity::Enemy(0),
        });
        assert_eq!(effects1.len(), 1);
        assert!(puzzle.triggered);

        // Second damage - should not trigger
        let effects2 = puzzle.on_event(&BattleEvent::DamageTaken {
            target: Entity::Player,
            amount: 3,
            source: Entity::Enemy(0),
        });
        assert_eq!(effects2.len(), 0);
    }

    #[test]
    fn test_centennial_puzzle_resets_on_new_combat() {
        let mut puzzle = CentennialPuzzleRelic::new(Entity::Player);

        // First combat
        let _ = puzzle.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        let _ = puzzle.on_event(&BattleEvent::DamageTaken {
            target: Entity::Player,
            amount: 5,
            source: Entity::Enemy(0),
        });
        assert!(puzzle.triggered);

        // New combat
        let _ = puzzle.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        assert!(!puzzle.triggered);

        // Should trigger again
        let effects = puzzle.on_event(&BattleEvent::DamageTaken {
            target: Entity::Player,
            amount: 3,
            source: Entity::Enemy(0),
        });
        assert_eq!(effects.len(), 1);
    }

    #[test]
    fn test_centennial_puzzle_no_trigger_for_zero_damage() {
        let mut puzzle = CentennialPuzzleRelic::new(Entity::Player);

        let _ = puzzle.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        let effects = puzzle.on_event(&BattleEvent::DamageTaken {
            target: Entity::Player,
            amount: 0,
            source: Entity::Enemy(0),
        });

        assert_eq!(effects.len(), 0);
        assert!(!puzzle.triggered);
    }
}
