/// Trait for types that can be simulated forward with MCTS
///
/// This trait abstracts the core simulation capabilities needed for Monte Carlo Tree Search,
/// allowing MCTS to work with different types of game states (Battle, Game, etc.)
use std::hash::Hash;
use std::fmt::Debug;
use crate::game::game_error::GameError;

pub trait ForwardSimulation: Clone + PartialEq + Eq + Hash {
    /// The action type for this simulation
    type Action: Clone + PartialEq + Eq + Hash + Debug;

    /// Get all legal actions available from the current state
    fn list_available_actions(&self) -> Vec<Self::Action>;

    /// Execute an action and update the state
    ///
    /// Returns Ok(()) if the action was executed successfully
    /// Returns Err if the action is invalid or cannot be executed
    fn eval_action(&mut self, action: Self::Action, rng: &mut impl rand::Rng) -> Result<(), GameError>;

    /// Check if the current state is terminal (game/battle over)
    fn is_terminal(&self) -> bool;

    /// Evaluate the current state
    /// Returns the reward/value for this state (higher is better)
    /// - Terminal states: 0.0 for defeat, high value for victory (50.0+ for boss)
    /// - Non-terminal: heuristic evaluation
    fn evaluate(&self) -> f32;
}
