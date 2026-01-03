use crate::{battle::target::Entity, game::effect::BattleEffect};
use std::any::Any;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BattleEvent {
    DamageTaken {
        target: Entity,
        amount: u32,
        source: Entity
    },
    HpLostFromCard {
        target: Entity,
        amount: u32,
    },
    SkillCardPlayed {
        source: Entity,
    },
    CardPlayed {
        source: Entity,
        card_type: crate::game::card_type::CardType,
    },
    CardExhausted {
        source: Entity,
    },
    BlockGained {
        source: Entity,
        amount: u32,
    },
    StartOfPlayerTurn,
    StartOfEnemyTurn {
        enemy_index: usize,
    },
    EndOfTurn {
        entity: Entity,
    },
    CombatVictory {
        player: Entity,
    },
    CombatStart {
        player: Entity,
    },
    CardDrawn {
        card_type: crate::game::card_type::CardType,
        is_status_or_curse: bool,
    },
    EnemyDeath {
        enemy: Entity,
    },
    EnemySpawned {
        new_enemy_count: usize, // Total enemy count after spawning
    },
}

pub trait EventListener: Any + Send + Sync {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect>;
    fn is_active(&self) -> bool;
    fn get_owner(&self) -> Entity;

    /// Downcast to Any for dynamic type checking
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Hash this listener's state for MCTS transposition table
    /// Object-safe alternative to implementing Hash directly
    fn hash_to(&self, state: &mut std::collections::hash_map::DefaultHasher);
}