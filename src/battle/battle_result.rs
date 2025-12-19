//! Battle result types for battle outcomes

use crate::battle::battle_events::BattleEvent;

/// Result of a battle
#[derive(Debug, Clone, PartialEq)]
pub enum BattleResult {
    /// Battle is still ongoing
    Continued(Vec<BattleEvent>),
    /// Battle was won
    Won(Vec<BattleEvent>),
    /// Battle was lost
    Lost(Vec<BattleEvent>),
}