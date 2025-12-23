//! Error types for game-level operations

use crate::battle::BattleError;
use crate::map::error::MapError;

/// Errors that can occur during game actions
#[derive(Debug, Clone, PartialEq)]
pub enum GameError {
    /// Battle-specific error
    Battle(BattleError),
    /// Map-specific error
    Map(MapError),
    /// Action not valid in current game state
    InvalidState,
    /// Invalid card index
    InvalidCardIndex,
    /// Invalid choice index
    InvalidChoice,
    /// Not enough gold to purchase
    NotEnoughGold,
    /// No active battle
    NoBattle,
}
