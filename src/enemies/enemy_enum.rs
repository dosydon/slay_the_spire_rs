use crate::{enemies::{red_louse::{RedLouse, RedLouseMove}, green_louse::{GreenLouse, GreenLouseMove}, jaw_worm::{JawWorm, JawWormMove}, cultist::{Cultist, CultistMove}, spike_slime::{SpikeSlimeS, SpikeSlimeSMove}}, game::{effect::Effect, global_info::GlobalInfo, enemy::EnemyTrait}};

pub enum EnemyEnum {
    RedLouse(RedLouse),
    GreenLouse(GreenLouse),
    JawWorm(JawWorm),
    Cultist(Cultist),
    SpikeSlimeS(SpikeSlimeS),
}


/// Enum to hold any enemy move type  
#[derive(Debug, Clone, PartialEq)]
pub enum EnemyMove {
    RedLouse(RedLouseMove),
    GreenLouse(GreenLouseMove),
    JawWorm(JawWormMove),
    Cultist(CultistMove),
    SpikeSlimeS(SpikeSlimeSMove),
}

impl EnemyMove {
    /// Get a display string for this move based on its effects
    pub fn get_display_string(&self, effects: &[Effect]) -> String {
        let mut parts = Vec::new();
        
        for effect in effects {
            match effect {
                Effect::AttackToTarget { amount, .. } => {
                    parts.push(format!("🗡️ {}", amount));
                }
                Effect::GainDefense(amount) => {
                    parts.push(format!("🛡️ {}", amount));
                }
                Effect::GainStrength(amount) => {
                    parts.push(format!("💪 +{}", amount));
                }
                Effect::GainRitual(amount) => {
                    parts.push(format!("✨ Ritual {}", amount));
                }
                Effect::ApplyWeak(duration) => {
                    parts.push(format!("🔻 Weak {}", duration));
                }
                Effect::ApplyVulnerable(duration) => {
                    parts.push(format!("🔻 Vulnerable {}", duration));
                }
            }
        }
        
        if parts.is_empty() {
            "Unknown Action".to_string()
        } else {
            parts.join(" ")
        }
    }
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
        }
    }
}