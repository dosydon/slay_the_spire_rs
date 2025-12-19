//! Game result types for game actions outcomes

use crate::game::game_event::GameEvent;

/// Result of a game action
#[derive(Debug, Clone)]
pub struct GameResult {
    /// Game outcome after the action
    pub outcome: GameOutcome,
    /// Game events that occurred during this action (if any)
    pub game_events: Vec<GameEvent>,
}

/// Game outcome after an action
#[derive(Debug, Clone, PartialEq)]
pub enum GameOutcome {
    /// Action completed, game continues
    Continue,
    /// Run completed successfully
    Victory,
    /// Run ended in defeat
    Defeat,
}