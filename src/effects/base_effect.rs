use crate::battle::target::Entity;
use crate::game::card_enum::CardEnum;
use super::condition::Condition;
use super::effect::BattleEffect;

#[derive(Debug, Clone, PartialEq)]
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
    AttackToTargetWithBlock {
        source: Entity,
        target: Entity,
    },
    AttackToTargetWithScaling {
        source: Entity,
        target: Entity,
        base_damage: u32,
        scaling: u32,
    },
    PerfectedStrike {
        source: Entity,
        target: Entity,
        base_damage: u32,
        damage_per_strike: u32,
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
    ApplyVulnerableToAll {
        duration: u32,
    },
    HealToFull,
    ApplyWeak {
        target: Entity,
        duration: u32,
    },
    ApplyFrail {
        target: Entity,
        duration: u32,
    },
    ApplyEntangled {
        target: Entity,
        duration: u32,
    },
    GainStrength {
        source: Entity,
        amount: u32,
    },
    DoubleStrength {
        source: Entity,
    },
    LoseStrengthSelf {
        source: Entity,
        amount: u32,
    },
    LoseStrengthTarget {
        target: Entity,
        amount: u32,
    },
    LoseStrengthAtEndOfTurn {
        source: Entity,
        amount: u32,
    },
    GainDexterity {
        source: Entity,
        amount: u32,
    },
    LoseDexteritySelf {
        source: Entity,
        amount: u32,
    },
    LoseDexterityTarget {
        target: Entity,
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
    ActivateFeelNoPain {
        source: Entity,
        block_per_exhaust: u32,
    },
    ActivateBrutality {
        source: Entity,
    },
    ActivateCorruption {
        source: Entity,
    },
    ActivateMetallicize {
        source: Entity,
        amount: u32,
    },
    ActivateFlameBarrier {
        source: Entity,
        damage: u32,
    },
    ActivateDemonForm {
        source: Entity,
        strength_per_turn: u32,
    },
    ActivateRage {
        source: Entity,
        block_per_attack: u32,
    },
    AddRandomAttackToHand {
        source: Entity,
    },
    ActivateEvolve {
        source: Entity,
    },
    Heal {
        target: Entity,
        amount: u32,
    },
    HealAndIncreaseMaxHp {
        target: Entity,
        amount: u32,
    },
    LoseHp {
        target: Entity,
        amount: u32,
    },
    GainPlatedArmor {
        source: Entity,
        amount: u32,
    },
    GainRegen {
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
    GainEnergy {
        source: Entity,
        amount: u32,
    },
    ApplyWeakAll {
        duration: u32,
    },
    Ethereal {
        hand_index: usize, // hand_index should be set manually when queuing
    },
    AddCardToDiscard {
        card: CardEnum,
    },
    AddUpgradedCardToDiscard {
        card: CardEnum,
    },
    UpgradeAllCardsInHand {
        source: Entity,
    },
    AddCardToHand {
        source: Entity,
        card: CardEnum,
    },
    EnterSelectCardInHand,
    EnterSelectCardInHandToPutOnDeck,
    PlayTopCard {
        source: Entity,
        target: Entity,
    },
    PlayTopCardAndExhaust {
        source: Entity,
        target: Entity,
    },
    PutCardOnTopOfDrawPile {
        card: CardEnum,
    },
    EnterSelectCardInDiscard,
    PutRandomDiscardCardOnTop,
    ConditionalEffect {
        condition: Condition,
        effect: Box<BattleEffect>,
        source: Entity,
        target: Entity,
    },
    ExhaustNonAttackCardsFromHand {
        block_per_card: u32,
    },
    ActivateRupture,
    ActivateDoubleTap {
        remaining_attacks: u32,
    },
    EnterSelectCardToDuplicate {
        copies: u32,
    },
    EnterSelectCardInExhaust,
    HealOnKill {
        amount: u32,
    },
    AttackAllEnemiesAndHeal {
        amount: u32,
        num_attacks: u32,
    },
    ExhaustHandForDamage {
        damage_per_card: u32,
        target: Entity,
    },
    // TODO: Uncomment when Juggernaut is implemented
    // ActivateJuggernaut {
    //     damage_per_block: u32,
    // },
    AttackRandomEnemy {
        amount: u32,
        num_attacks: u32,
        strength_multiplier: u32,
    },
    ActivateFireBreathing {
        source: Entity,
        damage_per_status: u32,
    },
    ActivateSentinel {
        source: Entity,
        energy_on_exhaust: u32,
    },
    ShuffleDiscardIntoDraw {
        source: Entity,
    },
    AttackAllEnemiesForCurrentEnergy {
        amount_per_hit: u32,
    },
    WakeLagavulin {
        enemy_index: usize,
    },
    TransitionLagavulinStunnedToAwake {
        enemy_index: usize,
    },
    RemoveMetallicize {
        enemy_index: usize,
    },
    GainArtifact {
        source: Entity,
        amount: u32,
    },
    GainDefenseRandomAlly {
        source: Entity,
        amount: u32,
    },
    ActivateAngry {
        source: Entity,
        amount: u32,
    },
    StealGold {
        source: Entity,
        amount: u32,
    },
    EnemyEscape {
        source: Entity,
    },
    SplitIntoMediumSlimes {
        source: Entity,
    },
    LoseHpPerCardInHand {
        source: Entity,
        damage_per_card: u32,
    },
    AddRandomAttackCardsToHand {
        source: Entity,
        num_choices: u32,
        num_copies: u32,
        cost: u32,
    },
    AddRandomSkillCardsToHand {
        source: Entity,
        num_choices: u32,
        num_copies: u32,
        cost: u32,
    },
    ActivateGrantRitualNextTurn {
        source: Entity,
        amount: u32,
    },

    // Event-specific base effects
    GainGold {
        amount: u32,
    },
    SpendGold {
        amount: u32,
    },
    ObtainRandomRelic,
    EnterSelectCardsToUpgrade {
        count: u32,
    },
    UpgradeRandomCards {
        count: u32,
    },
    EnterSelectCardsToRemove {
        count: u32,
    },
    EnterSelectCardsToTransform {
        count: u32,
    },
    TriggerCombatEvent,
}

impl BaseEffect {
    pub fn from_effect(effect: BattleEffect, source: Entity, target: Entity) -> Self {
        match effect {
            BattleEffect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                BaseEffect::AttackToTarget { source, target, amount, num_attacks, strength_multiplier }
            }
            BattleEffect::AttackAllEnemies { amount, num_attacks } => {
                BaseEffect::AttackAllEnemies { source, amount, num_attacks }
            }
            BattleEffect::AttackToTargetWithBlock => BaseEffect::AttackToTargetWithBlock { source, target },
            BattleEffect::AttackToTargetWithScaling { base_damage, scaling } => BaseEffect::AttackToTargetWithScaling { source, target, base_damage, scaling },
            BattleEffect::PerfectedStrike { base_damage, damage_per_strike } => BaseEffect::PerfectedStrike { source, target, base_damage, damage_per_strike },
            BattleEffect::GainDefense { amount } => BaseEffect::GainDefense { source, amount },
            BattleEffect::ApplyVulnerable { duration } => BaseEffect::ApplyVulnerable { target, duration },
            BattleEffect::ApplyVulnerableAll { duration } => BaseEffect::ApplyVulnerableAll { duration },
            BattleEffect::ApplyVulnerableToAll { duration } => BaseEffect::ApplyVulnerableToAll { duration },
            BattleEffect::HealToFull => BaseEffect::HealToFull,
            BattleEffect::ApplyWeak { duration } => BaseEffect::ApplyWeak { target, duration },
            BattleEffect::ApplyFrail { duration } => BaseEffect::ApplyFrail { target, duration },
            BattleEffect::ApplyEntangled { duration } => BaseEffect::ApplyEntangled { target, duration },
            BattleEffect::GainStrength { amount } => BaseEffect::GainStrength { source, amount },
            BattleEffect::DoubleStrength => BaseEffect::DoubleStrength { source },
            BattleEffect::LoseStrengthSelf(amount) => BaseEffect::LoseStrengthSelf { source, amount },
            BattleEffect::LoseStrengthTarget(amount) => BaseEffect::LoseStrengthTarget { target, amount },
            BattleEffect::LoseStrengthAtEndOfTurn(amount) => BaseEffect::LoseStrengthAtEndOfTurn { source, amount },
            BattleEffect::GainDexterity { amount } => BaseEffect::GainDexterity { source, amount },
            BattleEffect::LoseDexteritySelf(amount) => BaseEffect::LoseDexteritySelf { source, amount },
            BattleEffect::LoseDexterityTarget(amount) => BaseEffect::LoseDexterityTarget { target, amount },
            BattleEffect::GainRitual(amount) => BaseEffect::GainRitual { source, amount },
            BattleEffect::AddSlimed(count) => BaseEffect::AddSlimed { target, count },
            BattleEffect::AddCardToDrawPile(card) => BaseEffect::AddCardToDrawPile { source, card },
            BattleEffect::DrawCard { count } => BaseEffect::DrawCard { source, count },
            BattleEffect::Exhaust => BaseEffect::Exhaust { hand_index: 0 }, // hand_index should be set manually when queuing
            BattleEffect::ActivateEnrage(amount) => BaseEffect::ActivateEnrage { source, amount },
            BattleEffect::ActivateEmbrace => BaseEffect::ActivateEmbrace { source },
            BattleEffect::ActivateFeelNoPain { block_per_exhaust } => BaseEffect::ActivateFeelNoPain { source, block_per_exhaust },
            BattleEffect::ActivateBrutality => BaseEffect::ActivateBrutality { source },
            BattleEffect::ActivateCorruption => BaseEffect::ActivateCorruption { source },
            BattleEffect::ActivateMetallicize { amount } => BaseEffect::ActivateMetallicize { source, amount },
            BattleEffect::ActivateFlameBarrier { damage } => BaseEffect::ActivateFlameBarrier { source, damage },
            BattleEffect::ActivateDemonForm { strength_per_turn } => BaseEffect::ActivateDemonForm { source, strength_per_turn },
            BattleEffect::ActivateRage { block_per_attack } => BaseEffect::ActivateRage { source, block_per_attack },
            BattleEffect::AddRandomAttackToHand => BaseEffect::AddRandomAttackToHand { source },
            BattleEffect::ActivateEvolve => BaseEffect::ActivateEvolve { source },
            BattleEffect::Heal(amount) => BaseEffect::Heal { target, amount },
            BattleEffect::HealAndIncreaseMaxHp(amount) => BaseEffect::HealAndIncreaseMaxHp { target, amount },
            BattleEffect::LoseHp(amount) => BaseEffect::LoseHp { target: source, amount },
            BattleEffect::GainPlatedArmor(amount) => BaseEffect::GainPlatedArmor { source, amount },
            BattleEffect::GainRegen { amount } => BaseEffect::GainRegen { source, amount },
            BattleEffect::DoubleBlock => BaseEffect::DoubleBlock { source },
            BattleEffect::ActivateCombust(amount) => BaseEffect::ActivateCombust { source, amount },
            BattleEffect::ApplyDamageReduction(percentage) => BaseEffect::ApplyDamageReduction { target, percentage },
            BattleEffect::GainEnergy { amount } => BaseEffect::GainEnergy { source, amount },
            BattleEffect::ApplyWeakAll { duration } => BaseEffect::ApplyWeakAll { duration },
            BattleEffect::Ethereal => BaseEffect::Ethereal { hand_index: 0 }, // hand_index should be set manually when queuing
            BattleEffect::AddCardToDiscard(card) => BaseEffect::AddCardToDiscard { card },
            BattleEffect::AddUpgradedCardToDiscard(card) => BaseEffect::AddUpgradedCardToDiscard { card },
            BattleEffect::UpgradeAllCardsInHand => BaseEffect::UpgradeAllCardsInHand { source },
            BattleEffect::AddCardToHand(card) => BaseEffect::AddCardToHand { source, card },
            BattleEffect::EnterSelectCardInHand => BaseEffect::EnterSelectCardInHand,
            BattleEffect::EnterSelectCardInHandToPutOnDeck => BaseEffect::EnterSelectCardInHandToPutOnDeck,
            BattleEffect::PlayTopCard => BaseEffect::PlayTopCard { source, target },
            BattleEffect::PlayTopCardAndExhaust => BaseEffect::PlayTopCardAndExhaust { source, target },
            BattleEffect::PutCardOnTopOfDrawPile(card) => BaseEffect::PutCardOnTopOfDrawPile { card },
            BattleEffect::EnterSelectCardInDiscard => BaseEffect::EnterSelectCardInDiscard,
            BattleEffect::PutRandomDiscardCardOnTop => BaseEffect::PutRandomDiscardCardOnTop,
            BattleEffect::ConditionalEffect(condition, effect) => BaseEffect::ConditionalEffect {
                condition,
                effect,
                source,
                target,
            },
            BattleEffect::ExhaustNonAttackCardsFromHand { block_per_card } => BaseEffect::ExhaustNonAttackCardsFromHand { block_per_card },
            BattleEffect::ActivateRupture => BaseEffect::ActivateRupture,
            BattleEffect::ActivateDoubleTap { remaining_attacks } => BaseEffect::ActivateDoubleTap { remaining_attacks },
            BattleEffect::EnterSelectCardToDuplicate { copies } => BaseEffect::EnterSelectCardToDuplicate { copies },
            BattleEffect::EnterSelectCardInExhaust => BaseEffect::EnterSelectCardInExhaust,
            BattleEffect::HealOnKill { amount } => BaseEffect::HealOnKill { amount },
            BattleEffect::AttackAllEnemiesAndHeal { amount, num_attacks } => BaseEffect::AttackAllEnemiesAndHeal { amount, num_attacks },
            BattleEffect::ExhaustHandForDamage { damage_per_card, target } => BaseEffect::ExhaustHandForDamage { damage_per_card, target },
            BattleEffect::ActivateJuggernaut { .. } => todo!("Implement Juggernaut when ready"),
            BattleEffect::AttackRandomEnemy { amount, num_attacks, strength_multiplier } => BaseEffect::AttackRandomEnemy { amount, num_attacks, strength_multiplier },
            BattleEffect::AddFireBreathing { damage_per_status } => BaseEffect::ActivateFireBreathing { source, damage_per_status },
            BattleEffect::ActivateSentinel { energy_on_exhaust } => BaseEffect::ActivateSentinel { source, energy_on_exhaust },
            BattleEffect::ShuffleDiscardIntoDraw => BaseEffect::ShuffleDiscardIntoDraw { source },
            BattleEffect::AttackAllEnemiesForCurrentEnergy { amount_per_hit } => BaseEffect::AttackAllEnemiesForCurrentEnergy { amount_per_hit },
            // New effects for the implemented cards
            BattleEffect::AddStatusToDiscard { status_card } => BaseEffect::AddCardToDiscard { card: status_card },
            BattleEffect::GainEnergyIfNoBlock { amount } => BaseEffect::GainEnergy { source, amount },
            BattleEffect::ExhaustNonAttacksInHand => BaseEffect::ExhaustNonAttackCardsFromHand { block_per_card: 0 },
            BattleEffect::GainStrengthIfEnemyAttacking { amount } => BaseEffect::GainStrength { source, amount },
            BattleEffect::WakeLagavulin { enemy_index } => BaseEffect::WakeLagavulin { enemy_index },
            BattleEffect::TransitionLagavulinStunnedToAwake { enemy_index } => BaseEffect::TransitionLagavulinStunnedToAwake { enemy_index },
            BattleEffect::RemoveMetallicize { enemy_index } => BaseEffect::RemoveMetallicize { enemy_index },
            BattleEffect::GainArtifact { amount } => BaseEffect::GainArtifact { source, amount },
            BattleEffect::GainDefenseRandomAlly { amount } => BaseEffect::GainDefenseRandomAlly { source, amount },
            BattleEffect::ActivateAngry { amount } => BaseEffect::ActivateAngry { source, amount },
            BattleEffect::StealGold { amount } => BaseEffect::StealGold { source, amount },
            BattleEffect::EnemyEscape => BaseEffect::EnemyEscape { source },
            BattleEffect::SplitIntoMediumSlimes => BaseEffect::SplitIntoMediumSlimes { source },
            BattleEffect::LoseHpPerCardInHand { damage_per_card } => BaseEffect::LoseHpPerCardInHand { source, damage_per_card },
            BattleEffect::AddRandomAttackCardsToHand { num_choices, num_copies, cost } => BaseEffect::AddRandomAttackCardsToHand { source, num_choices, num_copies, cost },
            BattleEffect::AddRandomSkillCardsToHand { num_choices, num_copies, cost } => BaseEffect::AddRandomSkillCardsToHand { source, num_choices, num_copies, cost },
            BattleEffect::ActivateGrantRitualNextTurn { amount } => BaseEffect::ActivateGrantRitualNextTurn { source, amount },
        }
    }
}
