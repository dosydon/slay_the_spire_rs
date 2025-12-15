use crate::{enemies::{red_louse::{RedLouse, RedLouseMove}, green_louse::{GreenLouse, GreenLouseMove}, jaw_worm::{JawWorm, JawWormMove}, cultist::{Cultist, CultistMove}, spike_slime_s::{SpikeSlimeS, SpikeSlimeSMove}, spike_slime_m::{SpikeSlimeM, SpikeSlimeMMove}, acid_slime_s::{AcidSlimeS, AcidSlimeSMove}, acid_slime_m::{AcidSlimeM, AcidSlimeMMove}, gremlin_nob::{GremlinNob, GremlinNobMove}, lagavulin::{Lagavulin, LagavulinMove}, sentry::{Sentry, SentryMove}, fat_gremlin::{FatGremlin, FatGremlinMove}, sneaky_gremlin::{SneakyGremlin, SneakyGremlinMove}, mad_gremlin::{MadGremlin, MadGremlinMove}, shield_gremlin::{ShieldGremlin, ShieldGremlinMove}, gremlin_wizard::{GremlinWizard, GremlinWizardMove}, looter::{Looter, LooterMove}, fungi_beast::{FungiBeast, FungiBeastMove}, blue_slaver::{BlueSlaver, BlueSlaverMove}, red_slaver::{RedSlaver, RedSlaverMove}}, game::{effect::Effect, global_info::GlobalInfo, enemy::EnemyTrait}};

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
    Sentry(Sentry),
    FatGremlin(FatGremlin),
    SneakyGremlin(SneakyGremlin),
    MadGremlin(MadGremlin),
    ShieldGremlin(ShieldGremlin),
    GremlinWizard(GremlinWizard),
    Looter(Looter),
    FungiBeast(FungiBeast),
    BlueSlaver(BlueSlaver),
    RedSlaver(RedSlaver),
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
    Sentry(SentryMove),
    FatGremlin(FatGremlinMove),
    SneakyGremlin(SneakyGremlinMove),
    MadGremlin(MadGremlinMove),
    ShieldGremlin(ShieldGremlinMove),
    GremlinWizard(GremlinWizardMove),
    Looter(LooterMove),
    FungiBeast(FungiBeastMove),
    BlueSlaver(BlueSlaverMove),
    RedSlaver(RedSlaverMove),
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
            EnemyEnum::Sentry(sentry) => {
                let (selected_move, effects) = sentry.choose_move_and_effects(global_info, rng);
                (EnemyMove::Sentry(selected_move), effects)
            }
            EnemyEnum::FatGremlin(fat_gremlin) => {
                let (selected_move, effects) = fat_gremlin.choose_move_and_effects(global_info, rng);
                (EnemyMove::FatGremlin(selected_move), effects)
            }
            EnemyEnum::SneakyGremlin(sneaky_gremlin) => {
                let (selected_move, effects) = sneaky_gremlin.choose_move_and_effects(global_info, rng);
                (EnemyMove::SneakyGremlin(selected_move), effects)
            }
            EnemyEnum::MadGremlin(mad_gremlin) => {
                let (selected_move, effects) = mad_gremlin.choose_move_and_effects(global_info, rng);
                (EnemyMove::MadGremlin(selected_move), effects)
            }
            EnemyEnum::ShieldGremlin(shield_gremlin) => {
                let (selected_move, effects) = shield_gremlin.choose_move_and_effects(global_info, rng);
                (EnemyMove::ShieldGremlin(selected_move), effects)
            }
            EnemyEnum::GremlinWizard(gremlin_wizard) => {
                let (selected_move, effects) = gremlin_wizard.choose_move_and_effects(global_info, rng);
                (EnemyMove::GremlinWizard(selected_move), effects)
            }
            EnemyEnum::Looter(looter) => {
                let (selected_move, effects) = looter.choose_move_and_effects(global_info, rng);
                (EnemyMove::Looter(selected_move), effects)
            }
            EnemyEnum::FungiBeast(fungi_beast) => {
                let (selected_move, effects) = fungi_beast.choose_move_and_effects(global_info, rng);
                (EnemyMove::FungiBeast(selected_move), effects)
            }
            EnemyEnum::BlueSlaver(blue_slaver) => {
                let (selected_move, effects) = blue_slaver.choose_move_and_effects(global_info, rng);
                (EnemyMove::BlueSlaver(selected_move), effects)
            }
            EnemyEnum::RedSlaver(red_slaver) => {
                let (selected_move, effects) = red_slaver.choose_move_and_effects(global_info, rng);
                (EnemyMove::RedSlaver(selected_move), effects)
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
            EnemyEnum::Sentry(sentry) => sentry.get_hp(),
            EnemyEnum::FatGremlin(fat_gremlin) => fat_gremlin.get_hp(),
            EnemyEnum::SneakyGremlin(sneaky_gremlin) => sneaky_gremlin.get_hp(),
            EnemyEnum::MadGremlin(mad_gremlin) => mad_gremlin.get_hp(),
            EnemyEnum::ShieldGremlin(shield_gremlin) => shield_gremlin.get_hp(),
            EnemyEnum::GremlinWizard(gremlin_wizard) => gremlin_wizard.get_hp(),
            EnemyEnum::Looter(looter) => looter.get_hp(),
            EnemyEnum::FungiBeast(fungi_beast) => fungi_beast.get_hp(),
            EnemyEnum::BlueSlaver(blue_slaver) => blue_slaver.get_hp(),
            EnemyEnum::RedSlaver(red_slaver) => red_slaver.get_hp(),
        }
    }
}