/// Monte Carlo Tree Search (MCTS) agent using Expectimax for stochastic games
///
/// This implementation properly handles stochastic actions in Slay the Spire by using:
/// - Decision nodes: Contain state, player chooses actions
/// - Chance nodes: Contain Action, environment determines outcome
///
/// Tree structure:
/// Decision Node (state) -> [Chance Node (action1), Chance Node (action2), ...]
/// Chance Node (action) -> [Decision Node (outcome1), Decision Node (outcome2), ...]
use crate::agents::ForwardSimulation;
use super::traits::Agent;
use super::decision_node::MCTSDecisionNode;
use super::chance_node::MCTSChanceNode;
use std::collections::HashMap;

/// MCTS agent with integrated tree structure
pub struct MCTS<S: ForwardSimulation> {
    /// Number of MCTS iterations to run per action selection
    pub iterations: usize,
    /// UCT exploration constant (typically sqrt(2) ≈ 1.41)
    pub exploration_constant: f32,
    /// Maximum depth for rollout simulations (None = until terminal)
    pub max_rollout_depth: Option<usize>,
    /// Number of samples per chance node to estimate outcomes
    pub samples_per_action: usize,

    /// Tree structure
    decision_nodes: Vec<MCTSDecisionNode<S>>,
    chance_nodes: Vec<MCTSChanceNode<S::Action>>,

    /// Global transposition table: maps any state to its decision node ID
    /// Allows reuse of nodes across different parts of the tree and different root states
    global_transposition_table: HashMap<S, usize>,
}

impl<S: ForwardSimulation> MCTS<S> {
    /// Create a new MCTS agent
    ///
    /// # Arguments
    /// * `iterations` - Number of MCTS iterations per action selection (e.g., 1000)
    /// * `exploration_constant` - UCT exploration parameter (typically 1.41)
    pub fn new(iterations: usize, exploration_constant: f32) -> Self {
        MCTS {
            iterations,
            exploration_constant,
            max_rollout_depth: Some(100),
            samples_per_action: 3, // Sample each action 3 times to estimate distribution
            decision_nodes: Vec::new(), // Start empty, roots created dynamically
            chance_nodes: Vec::new(),
            global_transposition_table: HashMap::new(),
        }
    }

    /// Run MCTS and return both the best action and statistics for all explored actions
    /// Returns (best_action, Vec<(action, visits, q_value)>)
    pub fn select_action(&mut self, root_state: &S, rng: &mut impl rand::Rng)
        -> (S::Action, Vec<(S::Action, usize, f32)>) {
        // Check if this state already exists in global transposition table
        let root_id = if let Some(&existing_id) = self.global_transposition_table.get(root_state) {
            // Reuse existing decision node as root
            existing_id
        } else {
            // Create new decision node for this root state
            let new_id = self.decision_nodes.len();
            self.decision_nodes.push(MCTSDecisionNode::new(None, root_state.clone()));
            self.global_transposition_table.insert(root_state.clone(), new_id);
            new_id
        };

        // Run MCTS iterations
        for _ in 0..self.iterations {
            // Clone state for this iteration
            let state = root_state.clone();

            // Run one iteration: select, expand, simulate, backpropagate
            let reward = self.run_iteration(root_id, state, rng);

            // Backpropagation is handled within run_iteration
            let _ = reward; // Suppress unused warning
        }

        // Select best action and get statistics
        let best_action = self.select_best_action(root_id, root_state);
        let stats = self.get_action_statistics(root_id);

        (best_action, stats)
    }

    /// Run a single MCTS iteration
    fn run_iteration(
        &mut self,
        root_id: usize,
        mut state: S,
        rng: &mut impl rand::Rng,
    ) -> f32 {
        // Track path: (is_decision_node, node_id)
        let mut path: Vec<(bool, usize)> = vec![(true, root_id)];
        let mut at_decision = true;
        let mut current_id = root_id;

        // Phase 1: Selection - traverse tree using UCT
        loop {
            if state.is_terminal() {
                // Terminal state - evaluate and backpropagate
                let reward = state.evaluate();
                self.backpropagate(&path, reward);
                return reward;
            }

            if at_decision {
                // At a decision node: select or expand an action
                let available_actions = state.list_available_actions();
                let decision_node = &self.decision_nodes[current_id];

                if !decision_node.is_fully_expanded(&available_actions) {
                    // Expand a new chance node (action)
                    if let Some(action) = decision_node.get_untried_action(&available_actions) {
                        let chance_id = self.expand_decision_node(current_id, action.clone());
                        path.push((false, chance_id));

                        // Execute action and continue from resulting decision node
                        let action_clone = action.clone();
                        if let Ok(_) = state.eval_action(action, rng) {
                            let (outcome_id, is_new_node) = self.add_decision_node(chance_id, &state);
                            path.push((true, outcome_id));

                            // Phase 2: Rollout - only if we created a new decision node
                            if is_new_node {
                                let reward = self.rollout(state, rng);
                                self.backpropagate(&path, reward);
                                return reward;
                            }

                            // Continue traversal if reusing existing node
                            at_decision = true;
                            current_id = outcome_id;
                            continue;
                        } else {
                            // Action failed - this should never happen for actions from list_available_actions
                            let action_for_error = action_clone.clone();
                            let error = state.eval_action(action_clone, rng).unwrap_err();
                            panic!(
                                "MCTS Bug: eval_action failed for an action from list_available_actions()\n\
                                 State Type: {}\n\
                                 Failed Action: {:?}\n\
                                 Error: {:?}\n\
                                 Available Actions: {:?}\n\
                                 This indicates list_available_actions() returned an invalid action.",
                                std::any::type_name::<S>(),
                                action_for_error,
                                error,
                                available_actions
                            );
                        }
                    }
                }

                // Fully expanded: select best chance node (action) by UCT
                let decision_node = &self.decision_nodes[current_id];
                if decision_node.children.is_empty() {
                    // No children - this should never happen if fully expanded
                    panic!("Decision node is fully expanded but has no children");
                }

                let parent_visits = decision_node.visits;
                let best_child_id = *decision_node.children.iter()
                    .max_by(|&&a, &&b| {
                        let a_uct = self.chance_nodes[a].uct_value(parent_visits, self.exploration_constant);
                        let b_uct = self.chance_nodes[b].uct_value(parent_visits, self.exploration_constant);
                        a_uct.partial_cmp(&b_uct).expect("UCT values should be comparable")
                    })
                    .expect("Decision node should have at least one child");

                path.push((false, best_child_id));
                at_decision = false;
                current_id = best_child_id;
            } else {
                // At a chance node: sample outcome
                let chance_node = &self.chance_nodes[current_id];
                let action = chance_node.action.clone();
                let is_fully_expanded = chance_node.is_fully_expanded(self.samples_per_action);

                if !is_fully_expanded {
                    // Sample a new outcome by executing the action
                    let action_clone = action.clone();
                    if let Ok(_) = state.eval_action(action, rng) {
                        let (outcome_id, is_new_node) = self.add_decision_node(current_id, &state);
                        path.push((true, outcome_id));

                        // Phase 2: Rollout - only if we created a new decision node
                        if is_new_node {
                            let reward = self.rollout(state, rng);
                            self.backpropagate(&path, reward);
                            return reward;
                        }

                        // Continue traversal if reusing existing node
                        at_decision = true;
                        current_id = outcome_id;
                        continue;
                    } else {
                        // Action failed - this should never happen for actions from list_available_actions
                        let error = state.eval_action(action_clone, rng).unwrap_err();
                        panic!(
                            "MCTS Bug: eval_action failed for an action from list_available_actions()\n\
                             State Type: {}\n\
                             Failed Action: {:?}\n\
                             Error: {:?}\n\
                             This indicates list_available_actions() returned an invalid action.",
                            std::any::type_name::<S>(),
                            chance_node.action,
                            error
                        );
                    }
                }

                // Fully expanded: use simulator to sample a fresh outcome
                // Get the parent decision node's state state
                let parent_decision_id = chance_node.parent;
                let parent_state = &self.decision_nodes[parent_decision_id].state;

                // Clone the parent state state and execute the action to get a fresh outcome
                let mut fresh_state = parent_state.clone();
                if let Ok(_) = fresh_state.eval_action(chance_node.action.clone(), rng) {
                    let (outcome_id, is_new_node) = self.add_decision_node(current_id, &fresh_state);
                    path.push((true, outcome_id));
                    state = fresh_state;

                    // Phase 2: Rollout - only if we created a new decision node
                    if is_new_node {
                        let reward = self.rollout(state, rng);
                        self.backpropagate(&path, reward);
                        return reward;
                    }

                    // Continue traversal if reusing existing node
                    at_decision = true;
                    current_id = outcome_id;
                } else {
                    // Action failed - this should never happen for actions from list_available_actions
                    let error = fresh_state.eval_action(chance_node.action.clone(), rng).unwrap_err();
                    panic!(
                        "MCTS Bug: eval_action failed for an action from list_available_actions()\n\
                         State Type: {}\n\
                         Failed Action: {:?}\n\
                         Error: {:?}\n\
                         This occurred during fresh outcome sampling from a fully expanded chance node.",
                        std::any::type_name::<S>(),
                        chance_node.action,
                        error
                    );
                }
            }
        }
    }

    /// Perform a random rollout from the current state state
    /// Returns the reward from the terminal state
    fn rollout(&self, mut state: S, rng: &mut impl rand::Rng) -> f32 {
        let mut depth = 0;
        let max_depth = self.max_rollout_depth.unwrap_or(100);

        loop {
            // Terminal state - evaluate
            if state.is_terminal() {
                return state.evaluate();
            }

            // Safety limit to prevent infinite rollouts
            if depth >= max_depth * 2 {
                return state.evaluate();
            }

            // Get available actions
            let available_actions = state.list_available_actions();

            if available_actions.is_empty() {
                return state.evaluate();
            }

            // Select a random action
            let random_action = available_actions[rng.gen_range(0..available_actions.len())].clone();

            // Execute the action
            match state.eval_action(random_action, rng) {
                Ok(_) => {
                    depth += 1;
                    continue;
                }
                Err(_) => {
                    // Action failed - evaluate current state
                    return state.evaluate();
                }
            }
        }
    }

    /// Expand a decision node by adding a new chance node (action)
    fn expand_decision_node(
        &mut self,
        decision_node_id: usize,
        action: S::Action,
    ) -> usize {
        // Create new chance node
        let chance_id = self.chance_nodes.len();
        self.chance_nodes.push(MCTSChanceNode::new(action.clone(), decision_node_id));

        // Add to decision node's children
        self.decision_nodes[decision_node_id].children.push(chance_id);
        self.decision_nodes[decision_node_id].tried_actions.push(action);

        chance_id
    }

    /// Add or reuse a decision node as child of a chance node
    /// Uses global transposition table to detect duplicate states across entire tree
    /// Returns (node_id, is_new) where is_new is true if a new node was created
    fn add_decision_node(&mut self, chance_node_id: usize, state: &S) -> (usize, bool) {
        // Check global transposition table for existing state
        if let Some(&existing_node_id) = self.global_transposition_table.get(state) {
            // Reuse existing decision node
            // Add to chance node's children if not already there
            let chance_node = &mut self.chance_nodes[chance_node_id];
            if !chance_node.children.contains(&existing_node_id) {
                chance_node.children.push(existing_node_id);
            }
            return (existing_node_id, false);
        }

        // Create new decision node
        let decision_id = self.decision_nodes.len();
        self.decision_nodes.push(MCTSDecisionNode::new(Some(chance_node_id), state.clone()));

        // Add to chance node's children and global transposition table
        self.chance_nodes[chance_node_id].children.push(decision_id);
        self.global_transposition_table.insert(state.clone(), decision_id);

        (decision_id, true)
    }

    /// Backpropagate reward up the tree
    fn backpropagate(&mut self, path: &[(bool, usize)], reward: f32) {
        for &(is_decision, node_id) in path.iter() {
            if is_decision {
                self.decision_nodes[node_id].visits += 1;
                self.decision_nodes[node_id].total_reward += reward;
            } else {
                self.chance_nodes[node_id].visits += 1;
                self.chance_nodes[node_id].total_reward += reward;
            }
        }
    }

    /// Select the best action based on visit counts of chance nodes
    fn select_best_action(&self, root_id: usize, state: &S) -> S::Action {
        let root = &self.decision_nodes[root_id];

        if root.children.is_empty() {
            // Fallback: return first available action
            let actions = state.list_available_actions();
            return actions[0].clone();
        }

        // Select chance node (action) with highest visit count
        let best_child_id = *root.children.iter()
            .max_by_key(|&&child_id| self.chance_nodes[child_id].visits)
            .expect("Root decision node should have at least one child chance node");

        // Extract action from chance node
        self.chance_nodes[best_child_id].action.clone()
    }

    /// Get statistics for all actions explored from the root
    /// Returns a vector of (action, visits, q_value)
    pub fn get_action_statistics(&self, root_id: usize) -> Vec<(S::Action, usize, f32)> {
        let root = &self.decision_nodes[root_id];

        root.children.iter()
            .map(|&child_id| {
                let chance_node = &self.chance_nodes[child_id];
                let q_value = if chance_node.visits > 0 {
                    chance_node.total_reward / chance_node.visits as f32
                } else {
                    0.0
                };
                (chance_node.action.clone(), chance_node.visits, q_value)
            })
            .collect()
    }

}

// Backward-compatible Agent implementation for Battle
impl Agent for MCTS<crate::battle::Battle> {
    fn select_action(&mut self, state: &crate::battle::Battle, rng: &mut impl rand::Rng) -> crate::battle::battle_action::BattleAction {
        // Call select_action and extract just the action component
        let (action, _stats) = <MCTS<crate::battle::Battle>>::select_action(self, state, rng);
        action
    }

    fn name(&self) -> &str {
        "MCTS-Expectimax"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle_builder::BattleBuilder;
    use crate::enemies::jaw_worm::JawWorm;
    use crate::enemies::enemy_enum::EnemyEnum;
    use crate::battle::Battle;

    #[test]
    fn test_mcts_agent_selects_valid_action() {
        let state = BattleBuilder::new()
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();

        let mut agent = MCTS::<Battle>::new(50, 1.41); // Fewer iterations for testing
        let mut rng = rand::rng();

        let (action, _stats) = agent.select_action(&state, &mut rng);

        // Action should be valid
        let available = state.list_available_actions();
        assert!(available.contains(&action));
    }

    #[test]
    fn test_decision_node_creation() {
        let state = BattleBuilder::new()
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();
        let node = MCTSDecisionNode::<Battle>::new(None, state);
        assert_eq!(node.visits, 0);
        assert_eq!(node.total_reward, 0.0);
        assert!(node.children.is_empty());
        assert!(node.tried_actions.is_empty());
    }

    #[test]
    fn test_chance_node_creation() {
        let state = BattleBuilder::new()
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();
        let actions = state.list_available_actions();
        let action = actions[0].clone();

        let node = MCTSChanceNode::<crate::battle::battle_action::BattleAction>::new(action.clone(), 0);
        assert_eq!(node.parent, 0);
        assert_eq!(node.visits, 0);
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_mcts_agent_name() {
        let agent = MCTS::<Battle>::new(1000, 1.41);
        assert_eq!(agent.name(), "MCTS-Expectimax");
    }

    #[test]
    fn test_node_uct_calculation() {
        let state = BattleBuilder::new()
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();
        let actions = state.list_available_actions();
        let action = actions[0].clone();

        let mut node = MCTSChanceNode::<crate::battle::battle_action::BattleAction>::new(action, 0);
        node.visits = 10;
        node.total_reward = 5.0;

        let uct = node.uct_value(100, 1.41);
        assert!(uct > 0.0);
        assert!(uct < f32::INFINITY);
    }
}
