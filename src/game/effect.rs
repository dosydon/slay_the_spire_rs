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
    ActivateEmbrace, // Activates Embrace listener for the player
    Heal (u32),
    GainPlatedArmor (u32), // Gain permanent armor that stacks
    DoubleBlock, // Double current block
    ActivateCombust (u32), // Activates Combust listener for dealing damage at end of turn
    ApplyDamageReduction (u32), // Target takes X% less damage (like Disarm)
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
        hand_index: usize,
    },
    ActivateEnrage {
        source: Entity,
        amount: u32,
    },
    ActivateEmbrace {
        source: Entity,
    },
    Heal {
        target: Entity,
        amount: u32,
    },
    GainPlatedArmor {
        source: Entity,
        amount: u32,
    },
    DoubleBlock {
        source: Entity,
    },
    ActivateCombust {
        source: Entity,
        amount: u32,
    },
    ApplyDamageReduction {
        target: Entity,
        percentage: u32, // percentage reduction (25 for Disarm)
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
            Effect::Exhaust => BaseEffect::Exhaust { hand_index: 0 }, // hand_index should be set manually when queuing
            Effect::ActivateEnrage(amount) => BaseEffect::ActivateEnrage { source, amount },
            Effect::ActivateEmbrace => BaseEffect::ActivateEmbrace { source },
            Effect::Heal(amount) => BaseEffect::Heal { target, amount },
            Effect::GainPlatedArmor(amount) => BaseEffect::GainPlatedArmor { source, amount },
            Effect::DoubleBlock => BaseEffect::DoubleBlock { source },
            Effect::ActivateCombust(amount) => BaseEffect::ActivateCombust { source, amount },
            Effect::ApplyDamageReduction(percentage) => BaseEffect::ApplyDamageReduction { target, percentage },
        }
    }
}
