use crate::{enemies::red_louse::RedLouse, game::{effect::Effect, global_info::GlobalInfo}, utils::CategoricalDistribution};

pub enum EnemyEnum {
    RedLouse(RedLouse),
}

impl EnemyEnum {
    /// Choose and sample effects directly
    /// This combines move selection, effect generation, and move tracking into one step
    pub fn choose_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> Vec<Effect> {
        match self {
            EnemyEnum::RedLouse(red_louse) => {
                red_louse.choose_effects(global_info, rng)
            }
        }
    }
}