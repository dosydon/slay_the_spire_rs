use crate::battle::{Battle, battle_action::BattleAction};

/// Decision node: player chooses action from this battle state
#[derive(Clone)]
pub struct MCTSDecisionNode {
    /// Number of visits to this node
    pub visits: usize,
    /// Total reward accumulated through this node
    pub total_reward: f32,
    /// Children chance nodes (one per action tried)
    pub children: Vec<usize>,
    /// Parent chance node ID (None for root)
    pub parent: Option<usize>,
    /// Actions that have been tried (corresponds to children indices)
    pub tried_actions: Vec<BattleAction>,
    /// The battle state at this decision node
    pub battle_state: Battle,
}

impl MCTSDecisionNode {
    pub fn new(parent: Option<usize>, battle_state: Battle) -> Self {
        MCTSDecisionNode {
            visits: 0,
            total_reward: 0.0,
            children: Vec::new(),
            parent,
            tried_actions: Vec::new(),
            battle_state,
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
    pub fn has_tried_action(&self, action: &BattleAction) -> bool {
        self.tried_actions.iter().any(|a| a == action)
    }

    /// Get an untried action from the available actions
    pub fn get_untried_action(&self, available_actions: &[BattleAction]) -> Option<BattleAction> {
        available_actions.iter()
            .find(|action| !self.has_tried_action(action))
            .cloned()
    }

    /// Check if all available actions have been tried
    pub fn is_fully_expanded(&self, available_actions: &[BattleAction]) -> bool {
        available_actions.iter().all(|action| self.has_tried_action(action))
    }
}
