use crate::battle::target::Entity;


#[derive(Copy, Debug, Clone, PartialEq)]
pub enum Effect {
    AttackToTarget {
        amount: u32,
        num_attacks: u32,
    },
    GainDefense (u32),
    ApplyVulnerable (u32),
    ApplyWeak (u32),
    GainStrength (u32),
    GainRitual (u32),
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum BaseEffect {
    AttackToTarget {
        source: Entity,
        target: Entity,
        amount: u32,
        num_attacks: u32,
    },
    GainDefense {
        source: Entity,
        amount: u32,
    },
    ApplyVulnerable {
        target: Entity,
        duration: u32,
    },
    ApplyWeak {
        target: Entity,
        duration: u32,
    },
    GainStrength {
        source: Entity,
        amount: u32,
    },
    GainRitual {
        source: Entity,
        amount: u32,
    },
}

impl BaseEffect {
    pub fn from_effect(effect: Effect, source: Entity, target: Entity) -> Self {
        match effect {
            Effect::AttackToTarget { amount, num_attacks } => {
                BaseEffect::AttackToTarget { source, target, amount, num_attacks }
            }
            Effect::GainDefense(amount) => BaseEffect::GainDefense { source, amount },
            Effect::ApplyVulnerable(duration) => BaseEffect::ApplyVulnerable { target, duration },
            Effect::ApplyWeak(duration) => BaseEffect::ApplyWeak { target, duration },
            Effect::GainStrength(amount) => BaseEffect::GainStrength { source, amount },
            Effect::GainRitual(amount) => BaseEffect::GainRitual { source, amount },
        }
    }
}
