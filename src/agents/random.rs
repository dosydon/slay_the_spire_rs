/// Random agent that selects actions uniformly at random
///
/// This serves as a baseline for comparison with more sophisticated agents
use crate::battle::{Battle, battle_action::BattleAction};
use super::traits::Agent;

/// Random agent implementation
pub struct RandomAgent;

impl RandomAgent {
    /// Create a new random agent
    pub fn new() -> Self {
        RandomAgent
    }
}

impl Default for RandomAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl Agent for RandomAgent {
    fn select_action(&mut self, battle: &Battle, rng: &mut impl rand::Rng) -> BattleAction {
        let actions = battle.list_available_actions();

        // This should never happen in practice, but handle gracefully
        if actions.is_empty() {
            panic!("No available actions - battle should be over");
        }

        // Select random action
        let idx = rng.gen_range(0..actions.len());
        actions[idx].clone()
    }

    fn name(&self) -> &str {
        "Random"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle_builder::BattleBuilder;
    use crate::enemies::jaw_worm::JawWorm;
    use crate::enemies::enemy_enum::EnemyEnum;

    #[test]
    fn test_random_agent_selects_valid_action() {
        let mut battle = BattleBuilder::new()
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();

        let mut agent = RandomAgent::new();
        let mut rng = rand::rng();

        // Agent should select a valid action
        let action = agent.select_action(&battle, &mut rng);

        // Action should be in the list of available actions
        let available = battle.list_available_actions();
        assert!(available.contains(&action));
    }

    #[test]
    fn test_random_agent_name() {
        let agent = RandomAgent::new();
        assert_eq!(agent.name(), "Random");
    }
}
