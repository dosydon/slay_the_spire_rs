use crate::{enemies::{red_louse::{RedLouse, RedLouseMove}, green_louse::{GreenLouse, GreenLouseMove}, jaw_worm::{JawWorm, JawWormMove}, cultist::{Cultist, CultistMove}, spike_slime_s::{SpikeSlimeS, SpikeSlimeSMove}, spike_slime_m::{SpikeSlimeM, SpikeSlimeMMove}, acid_slime_s::{AcidSlimeS, AcidSlimeSMove}, acid_slime_m::{AcidSlimeM, AcidSlimeMMove}, gremlin_nob::{GremlinNob, GremlinNobMove}, lagavulin::{Lagavulin, LagavulinMove}}, game::{effect::Effect, global_info::GlobalInfo, enemy::EnemyTrait}};

#[derive(Debug)]
pub enum EnemyEnum {
    RedLouse(RedLouse),
    GreenLouse(GreenLouse),
    JawWorm(JawWorm),
    Cultist(Cultist),
    SpikeSlimeS(SpikeSlimeS),
    SpikeSlimeM(SpikeSlimeM),
    AcidSlimeS(AcidSlimeS),
    AcidSlimeM(AcidSlimeM),
    GremlinNob(GremlinNob),
    Lagavulin(Lagavulin),
}


/// Enum to hold any enemy move type
#[derive(Debug, Clone, PartialEq)]
pub enum EnemyMove {
    RedLouse(RedLouseMove),
    GreenLouse(GreenLouseMove),
    JawWorm(JawWormMove),
    Cultist(CultistMove),
    SpikeSlimeS(SpikeSlimeSMove),
    SpikeSlimeM(SpikeSlimeMMove),
    AcidSlimeS(AcidSlimeSMove),
    AcidSlimeM(AcidSlimeMMove),
    GremlinNob(GremlinNobMove),
    Lagavulin(LagavulinMove),
}


impl EnemyEnum {
    /// Sample a move and return both the move type and its effects
    pub fn sample_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (EnemyMove, Vec<Effect>) {
        match self {
            EnemyEnum::RedLouse(red_louse) => {
                let (selected_move, effects) = red_louse.choose_move_and_effects(global_info, rng);
                (EnemyMove::RedLouse(selected_move), effects)
            }
            EnemyEnum::GreenLouse(green_louse) => {
                let (selected_move, effects) = green_louse.choose_move_and_effects(global_info, rng);
                (EnemyMove::GreenLouse(selected_move), effects)
            }
            EnemyEnum::JawWorm(jaw_worm) => {
                let (selected_move, effects) = jaw_worm.choose_move_and_effects(global_info, rng);
                (EnemyMove::JawWorm(selected_move), effects)
            }
            EnemyEnum::Cultist(cultist) => {
                let (selected_move, effects) = cultist.choose_move_and_effects(global_info, rng);
                (EnemyMove::Cultist(selected_move), effects)
            }
            EnemyEnum::SpikeSlimeS(spike_slime) => {
                let (selected_move, effects) = spike_slime.choose_move_and_effects(global_info, rng);
                (EnemyMove::SpikeSlimeS(selected_move), effects)
            }
            EnemyEnum::SpikeSlimeM(spike_slime) => {
                let (selected_move, effects) = spike_slime.choose_move_and_effects(global_info, rng);
                (EnemyMove::SpikeSlimeM(selected_move), effects)
            }
            EnemyEnum::AcidSlimeS(acid_slime) => {
                let (selected_move, effects) = acid_slime.choose_move_and_effects(global_info, rng);
                (EnemyMove::AcidSlimeS(selected_move), effects)
            }
            EnemyEnum::AcidSlimeM(acid_slime) => {
                let (selected_move, effects) = acid_slime.choose_move_and_effects(global_info, rng);
                (EnemyMove::AcidSlimeM(selected_move), effects)
            }
            EnemyEnum::GremlinNob(gremlin_nob) => {
                let (selected_move, effects) = gremlin_nob.choose_move_and_effects(global_info, rng);
                (EnemyMove::GremlinNob(selected_move), effects)
            }
            EnemyEnum::Lagavulin(lagavulin) => {
                let (selected_move, effects) = lagavulin.choose_move_and_effects(global_info, rng);
                (EnemyMove::Lagavulin(selected_move), effects)
            }
        }
    }


    /// Get the HP of the enemy
    pub fn get_hp(&self) -> u32 {
        match self {
            EnemyEnum::RedLouse(red_louse) => red_louse.get_hp(),
            EnemyEnum::GreenLouse(green_louse) => green_louse.get_hp(),
            EnemyEnum::JawWorm(jaw_worm) => jaw_worm.get_hp(),
            EnemyEnum::Cultist(cultist) => cultist.get_hp(),
            EnemyEnum::SpikeSlimeS(spike_slime) => spike_slime.get_hp(),
            EnemyEnum::SpikeSlimeM(spike_slime) => spike_slime.get_hp(),
            EnemyEnum::AcidSlimeS(acid_slime) => acid_slime.get_hp(),
            EnemyEnum::AcidSlimeM(acid_slime) => acid_slime.get_hp(),
            EnemyEnum::GremlinNob(gremlin_nob) => gremlin_nob.get_hp(),
            EnemyEnum::Lagavulin(lagavulin) => lagavulin.get_hp(),
        }
    }
}