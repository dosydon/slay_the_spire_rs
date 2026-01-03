/// Agents module for Slay the Spire
///
/// This module provides various agent implementations for playing the game,
/// including baselines (Random) and search-based methods (MCTS).
///
/// # Examples
///
/// ```rust
/// use slay_the_spire::agents::{Agent, RandomAgent, MCTS};
/// use slay_the_spire::battle_builder::BattleBuilder;
/// use slay_the_spire::enemies::{jaw_worm::JawWorm, enemy_enum::EnemyEnum};
///
/// // Create a battle
/// let mut battle = BattleBuilder::new()
///     .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
///     .build();
///
/// // Use random agent
/// let mut random_agent = RandomAgent::new();
/// let mut rng = rand::rng();
/// let action = random_agent.select_action(&battle, &mut rng);
///
/// // Or use MCTS agent
/// let mut mcts_agent = MCTS::new(1000, 1.41); // 1000 iterations, exploration constant 1.41
/// let action = mcts_agent.select_action(&battle, &mut rng);
/// ```

mod traits;
mod random;
mod decision_node;
mod chance_node;
mod mcts;

pub use traits::Agent;
pub use random::RandomAgent;
pub use decision_node::MCTSDecisionNode;
pub use chance_node::MCTSChanceNode;
pub use mcts::MCTS;
