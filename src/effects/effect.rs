use crate::battle::target::Entity;
use crate::game::card_enum::CardEnum;
use super::condition::Condition;

#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    AttackToTarget {
        amount: u32,
        num_attacks: u32,
        strength_multiplier: u32,
    },
    AttackToTargetWithBlock, // Deal damage equal to player's Block
    AttackToTargetWithScaling { base_damage: u32, scaling: u32 }, // Scaling damage attack (Rampage)
    PerfectedStrike { base_damage: u32, damage_per_strike: u32 }, // Deal damage + bonus per Strike card in deck
    AttackAllEnemies {
        amount: u32,
        num_attacks: u32,
    },
    GainDefense { amount: u32 },
    ApplyVulnerable { duration: u32 },
    ApplyVulnerableAll { duration: u32 },
    ApplyWeak { duration: u32 },
    ApplyFrail { duration: u32 },
    ApplyEntangled { duration: u32 }, // Prevents Attack card plays
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
    SplitIntoMediumSlimes, // Split into 2 medium slimes (used by large slimes on death)
    LoseHpPerCardInHand { damage_per_card: u32 }, // Lose HP for each card in hand (used by Regret)

    // Event-specific effects
    GainGold { amount: u32 }, // Gain gold (for events)
    SpendGold { amount: u32 }, // Spend/lose gold (for events)
    ObtainRandomRelic, // Obtain a random relic
    EnterSelectCardsToUpgrade { count: u32 }, // Transition to SelectCardsToUpgrade state (for upgrading multiple cards)
    UpgradeRandomCards { count: u32 }, // Upgrade random cards from deck (no player selection)
    EnterSelectCardsToRemove { count: u32 }, // Transition to SelectCardsToRemove state (for removing cards from deck)
    EnterSelectCardsToTransform { count: u32 }, // Transition to SelectCardsToTransform state (for transforming cards)
    TriggerCombatEvent, // Trigger a combat encounter from an event
}
