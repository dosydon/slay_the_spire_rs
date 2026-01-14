use std::hash::Hash;

/// Chance node: stochastic outcome of taking an action
#[derive(Clone)]
pub struct MCTSChanceNode<A: Clone + PartialEq + Eq + Hash> {
    /// The action this chance node represents
    pub action: A,
    /// Number of visits to this node
    pub visits: usize,
    /// Total reward accumulated through this node
    pub total_reward: f32,
    /// Children decision nodes (one per sampled outcome)
    pub children: Vec<usize>,
    /// Parent decision node ID
    pub parent: usize,
}

impl<A: Clone + PartialEq + Eq + Hash> MCTSChanceNode<A> {
    pub fn new(action: A, parent: usize) -> Self {
        MCTSChanceNode {
            action,
            visits: 0,
            total_reward: 0.0,
            children: Vec::new(),
            parent,
        }
    }

    /// Get average reward (Q-value)
    pub fn avg_reward(&self) -> f32 {
        if self.visits == 0 {
            0.0
        } else {
            self.total_reward / self.visits as f32
        }
    }

    /// Calculate UCT value for chance nodes
    pub fn uct_value(&self, parent_visits: usize, exploration_constant: f32) -> f32 {
        if self.visits == 0 {
            f32::INFINITY // Prioritize unvisited nodes
        } else {
            let exploitation = self.avg_reward();
            let exploration = exploration_constant * ((parent_visits as f32).ln() / self.visits as f32).sqrt();
            exploitation + exploration
        }
    }

    /// Check if this chance node has sampled enough outcomes
    pub fn is_fully_expanded(&self, samples_per_action: usize) -> bool {
        self.children.len() >= samples_per_action
    }
}
