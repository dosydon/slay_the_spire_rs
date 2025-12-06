use crate::battle::target::Entity;
use crate::game::card_enum::CardEnum;

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum Condition {
    // Target conditions
    TargetIsVulnerable,

    // Hand conditions
    HandAllAttacks,

    // Enemy state conditions
    EnemyIsAttacking,

    // Universal conditions
    True,
    False,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    AttackToTarget {
        amount: u32,
        num_attacks: u32,
        strength_multiplier: u32,
    },
    AttackToTargetWithBlock, // Deal damage equal to player's Block
    AttackToTargetWithScaling { base_damage: u32, scaling: u32 }, // Scaling damage attack (Rampage)
    AttackAllEnemies {
        amount: u32,
        num_attacks: u32,
    },
    GainDefense { amount: u32 },
    ApplyVulnerable { duration: u32 },
    ApplyVulnerableAll { duration: u32 },
    ApplyWeak { duration: u32 },
    ApplyFrail { duration: u32 },
    GainStrength { amount: u32 },
    DoubleStrength, // Double current Strength
    LoseStrengthSelf (u32), // Self strength loss (targets source)
    LoseStrengthTarget (u32), // Target strength loss (targets target)
    LoseStrengthAtEndOfTurn (u32),
    GainDexterity { amount: u32 },
    LoseDexteritySelf (u32), // Self dexterity loss (targets source)
    LoseDexterityTarget (u32), // Target dexterity loss (targets target)
    GainRitual (u32),
    AddSlimed (u32),
    AddCardToDrawPile (CardEnum),
    DrawCard { count: u32 },
    Exhaust,
    ActivateEnrage (u32), // Activates Enrage listener for this enemy
    ActivateEmbrace, // Activates Embrace listener for the player
    ActivateFeelNoPain { block_per_exhaust: u32 }, // Activates Feel No Pain listener for the player
    Heal (u32),
    HealAndIncreaseMaxHp (u32), // Heal and increase max HP by the same amount
    LoseHp (u32), // Direct HP loss
    GainPlatedArmor (u32), // Gain permanent armor that stacks
    DoubleBlock, // Double current block
    ActivateCombust (u32), // Activates Combust listener for dealing damage at end of turn
    ApplyDamageReduction (u32), // Target takes X% less damage (like Disarm)
    GainEnergy { amount: u32 }, // Gain energy
    ApplyWeakAll { duration: u32 }, // Apply Weak to all enemies
    Ethereal, // Card will be exhausted at end of turn
    AddCardToDiscard (CardEnum), // Add a card to discard pile
    AddUpgradedCardToDiscard (CardEnum), // Add an upgraded card to discard pile
    UpgradeAllCardsInHand, // Upgrade all cards in hand for the rest of combat
    AddCardToHand (CardEnum), // Add a card to hand
    EnterSelectCardInHand, // Transition to SelectCardInHand state
    EnterSelectCardInHandToPutOnDeck, // Transition to SelectCardInHandToPutOnDeck state
    ActivateBrutality, // Activates Brutality listener for drawing cards at start of turn
    PlayTopCard, // Play the top card of the draw pile
    PlayTopCardAndExhaust, // Play top card and exhaust it
    PutCardOnTopOfDrawPile(CardEnum), // Put a card on top of the draw pile
    EnterSelectCardInDiscard, // Transition to SelectCardInDiscard state
    PutRandomDiscardCardOnTop, // Put a random card from discard on top of draw pile
    ConditionalEffect(Condition, Box<Effect>), // Conditional effect that only triggers if condition is met
    ActivateCorruption, // Activates Corruption power for making skills cost 0 and exhaust them
    ActivateMetallicize { amount: u32 }, // Activates Metallicize power for end-of-turn block generation
    ActivateFlameBarrier { damage: u32 }, // Activates Flame Barrier for retaliation damage
    ActivateBurn { damage: u32 }, // Activates Burn for end-of-turn damage
    ActivateDemonForm { strength_per_turn: u32 }, // Activates Demon Form for turn-based Strength gain
    ActivateRage { block_per_attack: u32 }, // Activates Rage for gaining block when playing attacks
    AddRandomAttackToHand, // Add a random Attack card to hand
    ActivateEvolve, // Activates Evolve for drawing cards when Status cards are drawn
    ExhaustNonAttackCardsFromHand { block_per_card: u32 }, // Exhaust all non-Attack cards from hand, gain block per card
    ActivateRupture, // Activates Rupture for gaining Strength when losing HP
    EnterSelectCardToDuplicate { copies: u32 }, // Transition to SelectCardToDuplicate state to duplicate a card
    ActivateDoubleTap { remaining_attacks: u32 }, // Activates Double Tap for playing next Attack(s) twice
    EnterSelectCardInExhaust, // Transition to SelectCardInExhaust state to select card from exhaust pile
    HealOnKill { amount: u32 }, // Heal specified amount if the target enemy dies from this attack
    AttackAllEnemiesAndHeal { amount: u32, num_attacks: u32 }, // Deal damage to all enemies and heal for unblocked damage
    ExhaustHandForDamage { damage_per_card: u32, target: Entity }, // Exhaust all cards in hand and deal damage per card exhausted
    ActivateJuggernaut { damage_per_block: u32 }, // Activates Juggernaut for dealing damage when gaining block
    AttackRandomEnemy { amount: u32, num_attacks: u32, strength_multiplier: u32 }, // Deal damage to a random enemy
    AddFireBreathing { damage_per_status: u32 }, // Activates Fire Breathing for dealing damage when Status/Curse cards are drawn
    ShuffleDiscardIntoDraw, // Shuffle discard pile into draw pile
    AttackAllEnemiesForCurrentEnergy { amount_per_hit: u32 }, // Spend all energy and attack all enemies X times where X is energy spent
    AddStatusToDiscard { status_card: CardEnum }, // Add a status card to discard pile
    GainEnergyIfNoBlock { amount: u32 }, // Gain energy if player has no block
    ExhaustNonAttacksInHand, // Exhaust all non-Attack cards in hand
    GainStrengthIfEnemyAttacking { amount: u32 }, // Gain strength if enemy is attacking
    ActivateSentinel { energy_on_exhaust: u32 }, // Activates Sentinel listener for gaining energy when this card is exhausted
    WakeLagavulin { enemy_index: usize }, // Wake up Lagavulin from sleep state (transitions to stunned)
    TransitionLagavulinStunnedToAwake { enemy_index: usize }, // Transition Lagavulin from Stunned to Awake at start of turn
    RemoveMetallicize { enemy_index: usize }, // Remove Metallicize power from an enemy (used when Lagavulin wakes)
    GainArtifact { amount: u32 }, // Gain artifact charges (prevents debuffs)
    GainDefenseRandomAlly { amount: u32 }, // Grant defense to a random ally (used by Shield Gremlin)
    ActivateAngry { amount: u32 }, // Activates Angry listener for this enemy (gains Strength when damaged)
    StealGold { amount: u32 }, // Steal gold from the player (used by Looter)
    EnemyEscape, // Enemy escapes from combat (used by Looter)
}

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
    ActivateBurn {
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
        effect: Box<Effect>,
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
            Effect::AttackToTargetWithBlock => BaseEffect::AttackToTargetWithBlock { source, target },
            Effect::AttackToTargetWithScaling { base_damage, scaling } => BaseEffect::AttackToTargetWithScaling { source, target, base_damage, scaling },
            Effect::GainDefense { amount } => BaseEffect::GainDefense { source, amount },
            Effect::ApplyVulnerable { duration } => BaseEffect::ApplyVulnerable { target, duration },
            Effect::ApplyVulnerableAll { duration } => BaseEffect::ApplyVulnerableAll { duration },
            Effect::ApplyWeak { duration } => BaseEffect::ApplyWeak { target, duration },
            Effect::ApplyFrail { duration } => BaseEffect::ApplyFrail { target, duration },
            Effect::GainStrength { amount } => BaseEffect::GainStrength { source, amount },
            Effect::DoubleStrength => BaseEffect::DoubleStrength { source },
            Effect::LoseStrengthSelf(amount) => BaseEffect::LoseStrengthSelf { source, amount },
            Effect::LoseStrengthTarget(amount) => BaseEffect::LoseStrengthTarget { target, amount },
            Effect::LoseStrengthAtEndOfTurn(amount) => BaseEffect::LoseStrengthAtEndOfTurn { source, amount },
            Effect::GainDexterity { amount } => BaseEffect::GainDexterity { source, amount },
            Effect::LoseDexteritySelf(amount) => BaseEffect::LoseDexteritySelf { source, amount },
            Effect::LoseDexterityTarget(amount) => BaseEffect::LoseDexterityTarget { target, amount },
            Effect::GainRitual(amount) => BaseEffect::GainRitual { source, amount },
            Effect::AddSlimed(count) => BaseEffect::AddSlimed { target, count },
            Effect::AddCardToDrawPile(card) => BaseEffect::AddCardToDrawPile { source, card },
            Effect::DrawCard { count } => BaseEffect::DrawCard { source, count },
            Effect::Exhaust => BaseEffect::Exhaust { hand_index: 0 }, // hand_index should be set manually when queuing
            Effect::ActivateEnrage(amount) => BaseEffect::ActivateEnrage { source, amount },
            Effect::ActivateEmbrace => BaseEffect::ActivateEmbrace { source },
            Effect::ActivateFeelNoPain { block_per_exhaust } => BaseEffect::ActivateFeelNoPain { source, block_per_exhaust },
            Effect::ActivateBrutality => BaseEffect::ActivateBrutality { source },
            Effect::ActivateCorruption => BaseEffect::ActivateCorruption { source },
            Effect::ActivateMetallicize { amount } => BaseEffect::ActivateMetallicize { source, amount },
            Effect::ActivateFlameBarrier { damage } => BaseEffect::ActivateFlameBarrier { source, damage },
            Effect::ActivateBurn { damage } => BaseEffect::ActivateBurn { source, damage },
            Effect::ActivateDemonForm { strength_per_turn } => BaseEffect::ActivateDemonForm { source, strength_per_turn },
            Effect::ActivateRage { block_per_attack } => BaseEffect::ActivateRage { source, block_per_attack },
            Effect::AddRandomAttackToHand => BaseEffect::AddRandomAttackToHand { source },
            Effect::ActivateEvolve => BaseEffect::ActivateEvolve { source },
            Effect::Heal(amount) => BaseEffect::Heal { target, amount },
            Effect::HealAndIncreaseMaxHp(amount) => BaseEffect::HealAndIncreaseMaxHp { target, amount },
            Effect::LoseHp(amount) => BaseEffect::LoseHp { target: source, amount },
            Effect::GainPlatedArmor(amount) => BaseEffect::GainPlatedArmor { source, amount },
            Effect::DoubleBlock => BaseEffect::DoubleBlock { source },
            Effect::ActivateCombust(amount) => BaseEffect::ActivateCombust { source, amount },
            Effect::ApplyDamageReduction(percentage) => BaseEffect::ApplyDamageReduction { target, percentage },
            Effect::GainEnergy { amount } => BaseEffect::GainEnergy { source, amount },
            Effect::ApplyWeakAll { duration } => BaseEffect::ApplyWeakAll { duration },
            Effect::Ethereal => BaseEffect::Ethereal { hand_index: 0 }, // hand_index should be set manually when queuing
            Effect::AddCardToDiscard(card) => BaseEffect::AddCardToDiscard { card },
            Effect::AddUpgradedCardToDiscard(card) => BaseEffect::AddUpgradedCardToDiscard { card },
            Effect::UpgradeAllCardsInHand => BaseEffect::UpgradeAllCardsInHand { source },
            Effect::AddCardToHand(card) => BaseEffect::AddCardToHand { source, card },
            Effect::EnterSelectCardInHand => BaseEffect::EnterSelectCardInHand,
            Effect::EnterSelectCardInHandToPutOnDeck => BaseEffect::EnterSelectCardInHandToPutOnDeck,
            Effect::PlayTopCard => BaseEffect::PlayTopCard { source, target },
            Effect::PlayTopCardAndExhaust => BaseEffect::PlayTopCardAndExhaust { source, target },
            Effect::PutCardOnTopOfDrawPile(card) => BaseEffect::PutCardOnTopOfDrawPile { card },
            Effect::EnterSelectCardInDiscard => BaseEffect::EnterSelectCardInDiscard,
            Effect::PutRandomDiscardCardOnTop => BaseEffect::PutRandomDiscardCardOnTop,
            Effect::ConditionalEffect(condition, effect) => BaseEffect::ConditionalEffect {
                condition,
                effect,
                source,
                target,
            },
            Effect::ExhaustNonAttackCardsFromHand { block_per_card } => BaseEffect::ExhaustNonAttackCardsFromHand { block_per_card },
            Effect::ActivateRupture => BaseEffect::ActivateRupture,
            Effect::ActivateDoubleTap { remaining_attacks } => BaseEffect::ActivateDoubleTap { remaining_attacks },
            Effect::EnterSelectCardToDuplicate { copies } => BaseEffect::EnterSelectCardToDuplicate { copies },
            Effect::EnterSelectCardInExhaust => BaseEffect::EnterSelectCardInExhaust,
            Effect::HealOnKill { amount } => BaseEffect::HealOnKill { amount },
            Effect::AttackAllEnemiesAndHeal { amount, num_attacks } => BaseEffect::AttackAllEnemiesAndHeal { amount, num_attacks },
            Effect::ExhaustHandForDamage { damage_per_card, target } => BaseEffect::ExhaustHandForDamage { damage_per_card, target },
            Effect::ActivateJuggernaut { .. } => todo!("Implement Juggernaut when ready"),
            Effect::AttackRandomEnemy { amount, num_attacks, strength_multiplier } => BaseEffect::AttackRandomEnemy { amount, num_attacks, strength_multiplier },
            Effect::AddFireBreathing { damage_per_status } => BaseEffect::ActivateFireBreathing { source, damage_per_status },
            Effect::ActivateSentinel { energy_on_exhaust } => BaseEffect::ActivateSentinel { source, energy_on_exhaust },
            Effect::ShuffleDiscardIntoDraw => BaseEffect::ShuffleDiscardIntoDraw { source },
            Effect::AttackAllEnemiesForCurrentEnergy { amount_per_hit } => BaseEffect::AttackAllEnemiesForCurrentEnergy { amount_per_hit },
            // New effects for the implemented cards
            Effect::AddStatusToDiscard { status_card } => BaseEffect::AddCardToDiscard { card: status_card },
            Effect::GainEnergyIfNoBlock { amount } => BaseEffect::GainEnergy { source, amount },
            Effect::ExhaustNonAttacksInHand => BaseEffect::ExhaustNonAttackCardsFromHand { block_per_card: 0 },
            Effect::GainStrengthIfEnemyAttacking { amount } => BaseEffect::GainStrength { source, amount },
            Effect::WakeLagavulin { enemy_index } => BaseEffect::WakeLagavulin { enemy_index },
            Effect::TransitionLagavulinStunnedToAwake { enemy_index } => BaseEffect::TransitionLagavulinStunnedToAwake { enemy_index },
            Effect::RemoveMetallicize { enemy_index } => BaseEffect::RemoveMetallicize { enemy_index },
            Effect::GainArtifact { amount } => BaseEffect::GainArtifact { source, amount },
            Effect::GainDefenseRandomAlly { amount } => BaseEffect::GainDefenseRandomAlly { source, amount },
            Effect::ActivateAngry { amount } => BaseEffect::ActivateAngry { source, amount },
            Effect::StealGold { amount } => BaseEffect::StealGold { source, amount },
            Effect::EnemyEscape => BaseEffect::EnemyEscape { source },
        }
    }
}
