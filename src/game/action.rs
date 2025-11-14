//! Global game actions that can be performed throughout the entire game

use crate::battle::action::Action as BattleAction;

/// High-level game actions that encompass the entire game flow
#[derive(Debug, Clone, PartialEq)]
pub enum GameAction {
    /// Execute a battle action (PlayCard, EndTurn, etc.)
    /// Only valid when in battle
    Battle(BattleAction),

    /// Choose a path on the map (0-based index)
    /// Determines what type of encounter comes next
    ChoosePath(usize),

    /// Select a card reward (0, 1, or 2)
    /// Only valid when in CardRewardSelection state
    SelectCardReward(usize),
}

/// Result of ending a run
#[derive(Debug, Clone, PartialEq)]
pub enum RunResult {
    /// Player died in battle
    Death,
    /// Player completed all floors and won
    Victory,
    /// Player chose to abandon the run
    Abandon,
}