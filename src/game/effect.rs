use crate::game::target::Target;


#[derive(Copy, Debug, Clone, PartialEq)]
pub enum Effect {
    AttackToTarget {
        amount: u32,
        num_attacks: u32,
    },
    GainDefense (u32),
    Vulnerable (u32),
    GainStrength (u32),
}

impl Effect {
    pub fn with_target(self, target: Target) -> EffectWithTarget {
        match self {
            Effect::AttackToTarget { amount, num_attacks } => {
                EffectWithTarget::AttackToTarget { target, amount, num_attacks }
            }
            Effect::GainDefense(amount) => EffectWithTarget::GainDefense { amount },
            Effect::Vulnerable(duration) => EffectWithTarget::Vulnerable { duration },
            Effect::GainStrength(amount) => EffectWithTarget::GainStrength { amount },
        }
    }
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum EffectWithTarget {
    AttackToTarget {
        target: Target,
        amount: u32,
        num_attacks: u32,
    },
    GainDefense {
        amount: u32,
    },
    Vulnerable {
        duration: u32,
    },
    GainStrength {
        amount: u32,
    },
}

impl EffectWithTarget {
    pub fn from_effect(effect: Effect, target: Target) -> Self {
        match effect {
            Effect::AttackToTarget { amount, num_attacks } => {
                EffectWithTarget::AttackToTarget { target, amount, num_attacks }
            }
            Effect::GainDefense(amount) => EffectWithTarget::GainDefense { amount },
            Effect::Vulnerable(duration) => EffectWithTarget::Vulnerable { duration },
            Effect::GainStrength(amount) => EffectWithTarget::GainStrength { amount },
        }
    }
}
