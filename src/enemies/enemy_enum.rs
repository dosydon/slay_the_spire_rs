use crate::{enemies::{red_louse::{RedLouse, RedLouseMove}, green_louse::{GreenLouse, GreenLouseMove}, jaw_worm::{JawWorm, JawWormMove}, cultist::{Cultist, CultistMove}}, game::{effect::Effect, global_info::GlobalInfo, enemy::EnemyTrait}, utils::CategoricalDistribution};

pub enum EnemyEnum {
    RedLouse(RedLouse),
    GreenLouse(GreenLouse),
    JawWorm(JawWorm),
    Cultist(Cultist),
}

/// Represents an enemy's intended action with actual values
#[derive(Debug, Clone, PartialEq)]
pub struct EnemyAction {
    /// The raw move type
    pub move_type: EnemyMoveType,
    /// Damage amount (if applicable)
    pub damage: Option<u32>,
    /// Block amount (if applicable)  
    pub block: Option<u32>,
    /// Strength gain (if applicable)
    pub strength: Option<u32>,
    /// Buff applications (ritual, etc.)
    pub buffs: Vec<String>,
    /// Debuff applications (weak, vulnerable, etc.)
    pub debuffs: Vec<String>,
}

impl EnemyAction {
    /// Get a display string for this action
    pub fn get_display_string(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(damage) = self.damage {
            parts.push(format!("🗡️ {}", damage));
        }
        
        if let Some(block) = self.block {
            parts.push(format!("🛡️ {}", block));
        }
        
        if let Some(strength) = self.strength {
            parts.push(format!("💪 +{}", strength));
        }
        
        for buff in &self.buffs {
            parts.push(format!("✨ {}", buff));
        }
        
        for debuff in &self.debuffs {
            parts.push(format!("🔻 {}", debuff));
        }
        
        if parts.is_empty() {
            "Unknown Action".to_string()
        } else {
            parts.join(" ")
        }
    }
}

/// Enum to hold any enemy move type  
#[derive(Debug, Clone, PartialEq)]
pub enum EnemyMoveType {
    RedLouse(RedLouseMove),
    GreenLouse(GreenLouseMove),
    JawWorm(JawWormMove),
    Cultist(CultistMove),
}

impl EnemyEnum {
    /// Sample a move and return both the action info and its effects
    /// This allows displaying the intended move while also getting the effects for execution
    pub fn sample_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (EnemyAction, Vec<Effect>) {
        match self {
            EnemyEnum::RedLouse(red_louse) => {
                let (selected_move, effects) = red_louse.choose_move_and_effects(global_info, rng);
                let action = Self::create_enemy_action(EnemyMoveType::RedLouse(selected_move), &effects);
                (action, effects)
            }
            EnemyEnum::GreenLouse(green_louse) => {
                let (selected_move, effects) = green_louse.choose_move_and_effects(global_info, rng);
                let action = Self::create_enemy_action(EnemyMoveType::GreenLouse(selected_move), &effects);
                (action, effects)
            }
            EnemyEnum::JawWorm(jaw_worm) => {
                let (selected_move, effects) = jaw_worm.choose_move_and_effects(global_info, rng);
                let action = Self::create_enemy_action(EnemyMoveType::JawWorm(selected_move), &effects);
                (action, effects)
            }
            EnemyEnum::Cultist(cultist) => {
                let (selected_move, effects) = cultist.choose_move_and_effects(global_info, rng);
                let action = Self::create_enemy_action(EnemyMoveType::Cultist(selected_move), &effects);
                (action, effects)
            }
        }
    }
    
    /// Create an EnemyAction by extracting values from effects
    fn create_enemy_action(move_type: EnemyMoveType, effects: &[Effect]) -> EnemyAction {
        let mut damage = None;
        let mut block = None;
        let mut strength = None;
        let mut buffs = Vec::new();
        let mut debuffs = Vec::new();
        
        for effect in effects {
            match effect {
                Effect::AttackToTarget { amount, .. } => {
                    damage = Some(*amount);
                }
                Effect::GainDefense(amount) => {
                    block = Some(*amount);
                }
                Effect::GainStrength(amount) => {
                    strength = Some(*amount);
                }
                Effect::GainRitual(amount) => {
                    buffs.push(format!("Ritual {}", amount));
                }
                Effect::ApplyWeak(duration) => {
                    debuffs.push(format!("Weak {}", duration));
                }
                Effect::ApplyVulnerable(duration) => {
                    debuffs.push(format!("Vulnerable {}", duration));
                }
            }
        }
        
        EnemyAction {
            move_type,
            damage,
            block,
            strength,
            buffs,
            debuffs,
        }
    }

    /// Choose and sample effects directly (kept for backward compatibility)
    /// This combines move selection, effect generation, and move tracking into one step
    pub fn choose_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> Vec<Effect> {
        let (_move, effects) = self.sample_move_and_effects(global_info, rng);
        effects
    }

    /// Get the HP of the enemy
    pub fn get_hp(&self) -> u32 {
        match self {
            EnemyEnum::RedLouse(red_louse) => red_louse.get_hp(),
            EnemyEnum::GreenLouse(green_louse) => green_louse.get_hp(),
            EnemyEnum::JawWorm(jaw_worm) => jaw_worm.get_hp(),
            EnemyEnum::Cultist(cultist) => cultist.get_hp(),
        }
    }
}