use crate::{enemies::{red_louse::RedLouse, green_louse::GreenLouse, jaw_worm::JawWorm}, game::{effect::Effect, global_info::GlobalInfo}, utils::CategoricalDistribution};

pub enum EnemyEnum {
    RedLouse(RedLouse),
    GreenLouse(GreenLouse),
    JawWorm(JawWorm),
}

impl EnemyEnum {
    /// Choose and sample effects directly
    /// This combines move selection, effect generation, and move tracking into one step
    pub fn choose_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> Vec<Effect> {
        match self {
            EnemyEnum::RedLouse(red_louse) => {
                red_louse.choose_effects(global_info, rng)
            }
            EnemyEnum::GreenLouse(green_louse) => {
                green_louse.choose_effects(global_info, rng)
            }
            EnemyEnum::JawWorm(jaw_worm) => {
                jaw_worm.choose_effects(global_info, rng)
            }
        }
    }
}