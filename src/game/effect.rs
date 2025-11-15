use crate::battle::target::Entity;
use crate::game::card_enum::CardEnum;


#[derive(Copy, Debug, Clone, PartialEq)]
pub enum Effect {
    AttackToTarget {
        amount: u32,
        num_attacks: u32,
        strength_multiplier: u32,
    },
    AttackAllEnemies {
        amount: u32,
        num_attacks: u32,
    },
    GainDefense (u32),
    ApplyVulnerable (u32),
    ApplyVulnerableAll (u32),
    ApplyWeak (u32),
    ApplyFrail (u32),
    GainStrength (u32),
    LoseStrength (u32), // Direct strength loss
    LoseStrengthAtEndOfTurn (u32),
    GainRitual (u32),
    AddSlimed (u32),
    AddCardToDrawPile (CardEnum),
    DrawCard (u32),
    Exhaust,
    ActivateEnrage (u32), // Activates Enrage listener for this enemy
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum BaseEffect {
    AttackToTarget {
        source: Entity,
        target: Entity,
        amount: u32,
        num_attacks: u32,
        strength_multiplier: u32,
    },
    AttackAllEnemies {
        source: Entity,
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
    ApplyVulnerableAll {
        duration: u32,
    },
    ApplyWeak {
        target: Entity,
        duration: u32,
    },
    ApplyFrail {
        target: Entity,
        duration: u32,
    },
    GainStrength {
        source: Entity,
        amount: u32,
    },
    LoseStrength {
        source: Entity,
        amount: u32,
    },
    LoseStrengthAtEndOfTurn {
        source: Entity,
        amount: u32,
    },
    GainRitual {
        source: Entity,
        amount: u32,
    },
    AddSlimed {
        target: Entity,
        count: u32,
    },
    AddCardToDrawPile {
        source: Entity,
        card: CardEnum,
    },
    DrawCard {
        source: Entity,
        count: u32,
    },
    Exhaust {
        source: Entity,
    },
    ActivateEnrage {
        source: Entity,
        amount: u32,
    },
}

impl BaseEffect {
    pub fn from_effect(effect: Effect, source: Entity, target: Entity) -> Self {
        match effect {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                BaseEffect::AttackToTarget { source, target, amount, num_attacks, strength_multiplier }
            }
            Effect::AttackAllEnemies { amount, num_attacks } => {
                BaseEffect::AttackAllEnemies { source, amount, num_attacks }
            }
            Effect::GainDefense(amount) => BaseEffect::GainDefense { source, amount },
            Effect::ApplyVulnerable(duration) => BaseEffect::ApplyVulnerable { target, duration },
            Effect::ApplyVulnerableAll(duration) => BaseEffect::ApplyVulnerableAll { duration },
            Effect::ApplyWeak(duration) => BaseEffect::ApplyWeak { target, duration },
            Effect::ApplyFrail(duration) => BaseEffect::ApplyFrail { target, duration },
            Effect::GainStrength(amount) => BaseEffect::GainStrength { source, amount },
            Effect::LoseStrength(amount) => BaseEffect::LoseStrength { source, amount },
            Effect::LoseStrengthAtEndOfTurn(amount) => BaseEffect::LoseStrengthAtEndOfTurn { source, amount },
            Effect::GainRitual(amount) => BaseEffect::GainRitual { source, amount },
            Effect::AddSlimed(count) => BaseEffect::AddSlimed { target, count },
            Effect::AddCardToDrawPile(card) => BaseEffect::AddCardToDrawPile { source, card },
            Effect::DrawCard(count) => BaseEffect::DrawCard { source, count },
            Effect::Exhaust => BaseEffect::Exhaust { source },
            Effect::ActivateEnrage(amount) => BaseEffect::ActivateEnrage { source, amount },
        }
    }
}
