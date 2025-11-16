use crate::game::{effect::Effect, global_info::GlobalInfo};

pub trait EnemyTrait {
    type MoveType;
    fn instantiate(rng: &mut impl rand::Rng, _global_info: &GlobalInfo) -> Self;
    fn get_name() -> String;
    fn get_hp(&self) -> u32;
    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (Self::MoveType, Vec<Effect>);
}