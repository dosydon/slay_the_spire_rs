use crate::{game::global_info::GlobalInfo, utils::CategoricalDistribution};

pub trait EnemyTrait {
    type MoveType;
    fn instantiate(rng: &mut impl rand::Rng, _global_info: &GlobalInfo) -> Self;
    fn hp_lb() -> u32;
    fn hp_ub() -> u32;
    fn choose_next_move(&self, global_info: &GlobalInfo) -> CategoricalDistribution<Self::MoveType>;
    fn get_name() -> String;
    fn get_hp(&self) -> u32;
}