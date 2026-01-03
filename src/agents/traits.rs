/// Agent trait for selecting actions in Slay the Spire battles
///
/// This trait provides a common interface for different agent implementations
/// (e.g., Random, MCTS, Deep RL, etc.)
use crate::battle::{Battle, battle_action::BattleAction};

pub trait Agent {
    /// Select an action for the given battle state
    ///
    /// # Arguments
    /// * `battle` - The current battle state
    /// * `rng` - Random number generator for stochastic decisions
    ///
    /// # Returns
    /// The selected battle action to execute
    fn select_action(&mut self, battle: &Battle, rng: &mut impl rand::Rng) -> BattleAction;

    /// Get the name of this agent (for logging/identification)
    fn name(&self) -> &str;

    /// Optional: Reset agent state between battles
    fn reset(&mut self) {
        // Default implementation does nothing
    }
}
