use crate::agents::ForwardSimulation;

/// Decision node: player chooses action from this state
#[derive(Clone)]
pub struct MCTSDecisionNode<S: ForwardSimulation> {
    /// Number of visits to this node
    pub visits: usize,
    /// Total reward accumulated through this node
    pub total_reward: f32,
    /// Children chance nodes (one per action tried)
    pub children: Vec<usize>,
    /// Parent chance node ID (None for root)
    pub parent: Option<usize>,
    /// Actions that have been tried (corresponds to children indices)
    pub tried_actions: Vec<S::Action>,
    /// The state at this decision node
    pub state: S,
}

impl<S: ForwardSimulation> MCTSDecisionNode<S> {
    pub fn new(parent: Option<usize>, state: S) -> Self {
        MCTSDecisionNode {
            visits: 0,
            total_reward: 0.0,
            children: Vec::new(),
            parent,
            tried_actions: Vec::new(),
            state,
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

    /// Check if this node has tried the given action
    pub fn has_tried_action(&self, action: &S::Action) -> bool {
        self.tried_actions.iter().any(|a| a == action)
    }

    /// Get an untried action from the available actions
    pub fn get_untried_action(&self, available_actions: &[S::Action]) -> Option<S::Action> {
        available_actions.iter()
            .find(|action| !self.has_tried_action(action))
            .cloned()
    }

    /// Check if all available actions have been tried
    pub fn is_fully_expanded(&self, available_actions: &[S::Action]) -> bool {
        available_actions.iter().all(|action| self.has_tried_action(action))
    }
}
