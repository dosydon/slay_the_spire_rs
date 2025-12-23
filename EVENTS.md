# Events Implementation Status

This document tracks all events in Slay the Spire and their implementation status.

## Table of Contents

1. [Summary](#summary)
2. [Quick Reference for Developers](#quick-reference-for-developers)
3. [About Events](#about-events)
4. [Event Choice System](#event-choice-system)
5. [Implemented Events - Detailed Choice Information](#implemented-events---detailed-choice-information)
6. [Event Tables by Category](#event-tables-by-category)
7. [Event Implementation Framework](#event-implementation-framework)
8. [Recommended Implementation Order](#recommended-implementation-order)
9. [Recently Implemented Events](#recently-implemented-events)
10. [How to Add New Events](#how-to-add-new-events)
11. [References](#references)

---

## Summary

- ✅ **7 events implemented**
- 📋 **60+ total events** across all acts
- 🎯 **Implementation Progress: ~12%**

**Event Distribution:**
- **Shared Events**: 16 (appear in multiple acts)
- **Act 1 Exclusive**: 12 events (7 implemented)
- **Act 2 Exclusive**: 16 events
- **Act 3 Exclusive**: 8 events
- **Shrines**: 5 events (upgrade/removal/transformation)

## Quick Reference for Developers

**Key Files**:
- [events/map_events/mod.rs](slay_the_spire/src/events/map_events/mod.rs) - MapEvent enum and choice system
- [events/encounter_events.rs](slay_the_spire/src/events/encounter_events.rs) - Combat encounter events
- [game/effect.rs](slay_the_spire/src/game/effect.rs) - Available effects for event outcomes
- Individual event files in [events/map_events/](slay_the_spire/src/events/map_events/)

**Implemented Events** (7 total):
1. Big Fish - 3 choices (Max HP, Heal, Relic)
2. The Cleric - Context-aware healing for gold
3. Dead Adventurer - Combat or flee
4. Golden Idol - Risk/reward idol stealing
5. Shining Light - Card upgrades
6. World of Goop - Gold for deck pollution
7. Wing Statue - RNG-based outcomes

**To Add an Event**: See [How to Add New Events](#how-to-add-new-events) section below

---

## About Events

Events are special scenarios that occur when entering an unknown location (? rooms). They present the player with narrative choices that can:
- Grant or remove cards
- Provide relics
- Heal or damage the player
- Give or take gold
- Apply curses or blessings
- Transform cards
- Provide unique one-time benefits

Events are a core part of the roguelike experience, offering risk/reward decisions that shape each run.

---

## Event Choice System

### How Event Choices Work

Events in this implementation use a structured choice system:

1. **EventChoice Structure**: Each choice contains:
   - `text`: Display text shown to the player (e.g., "Banana (Gain 5 Max HP)")
   - `outcome`: What happens when selected

2. **EventOutcome Types**:
   - `Effects(Vec<Effect>)`: Apply immediate effects and end event
   - `NextChoices(Vec<EventChoice>)`: Transition to new choices (for multi-stage events)

3. **Effect Types**: Events can trigger various effects:
   - `Heal(amount)`: Restore HP
   - `HealAndIncreaseMaxHp(amount)`: Gain Max HP
   - `LoseHp(amount)`: Take damage
   - `ObtainRandomRelic`: Gain a relic
   - `TriggerCombatEvent`: Start a combat encounter
   - More effects available in game/effect.rs

4. **Context-Aware Choices**: Events use `EventContext` to adjust based on game state:
   - Player HP and Max HP
   - Current floor
   - Gold amount
   - Ascension level

### Example: Big Fish Event

The Big Fish event demonstrates the choice system:

```rust
pub fn big_fish_choices(player_max_hp: u32) -> Vec<EventChoice> {
    vec![
        EventChoice {
            text: "Banana (Gain 5 Max HP)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::HealAndIncreaseMaxHp(5),
            ]),
        },
        EventChoice {
            text: format!("Donut (Heal {} HP)", player_max_hp / 3),
            outcome: EventOutcome::Effects(vec![
                Effect::Heal(0),  // 0 = calculate as max_hp / 3
            ]),
        },
        EventChoice {
            text: "Box (Obtain a random relic)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::ObtainRandomRelic,
            ]),
        },
    ]
}
```

This shows:
- Fixed effects (Banana: always +5 Max HP)
- Dynamic text (Donut heal amount shown in text)
- Context-dependent effects (heal based on player's Max HP)
- Multiple effect types (heal, Max HP, relics)

---

## Implemented Events - Detailed Choice Information

### Big Fish (Act 1)

**Source**: [Slay the Spire Wiki - Big Fish](https://slay-the-spire.fandom.com/wiki/Big_Fish)

**Description**: "A massive fish emerges from the depths, its scales shimmering with an otherworldly glow. It regards you with ancient, knowing eyes."

**Choices**:
1. **Banana** - Gain 5 Max HP
   - Effect: `HealAndIncreaseMaxHp(5)`
   - No downside

2. **Donut** - Heal for 1/3 of Max HP
   - Effect: `Heal(player_max_hp / 3)`
   - Displays calculated amount in choice text
   - Example: With 80 Max HP → "Donut (Heal 26 HP)"

3. **Box** - Obtain a random relic
   - Effect: `ObtainRandomRelic`
   - High variance option

**Implementation File**: [big_fish.rs](slay_the_spire/src/events/map_events/big_fish.rs)

---

### The Cleric (Act 1)

**Source**: [Slay the Spire Wiki - The Cleric](https://slay-the-spire.fandom.com/wiki/The_Cleric)

**Description**: "A wandering cleric offers their healing services for a price."

**Choices**: Context-aware based on player HP and floor
- Gold cost scales with current floor
- Healing choices adjust based on missing HP
- Option to decline (Leave)

**Implementation Details**:
- Uses `EventContext` to determine available healing amounts
- Gold cost: `floor × scaling_factor`
- Multiple healing tiers available based on current HP

**Implementation File**: [the_cleric.rs](slay_the_spire/src/events/map_events/the_cleric.rs)

---

### Dead Adventurer (Act 1)

**Source**: [Slay the Spire Wiki - Dead Adventurer](https://slay-the-spire.fandom.com/wiki/Dead_Adventurer)

**Description**: "You find the corpse of an adventurer, their belongings scattered around them. Strange sounds echo from deeper in the cave..."

**Choices**:
1. **Fight the creature** - Enter combat
   - Effect: `TriggerCombatEvent`
   - Spawns a Lagavulin fight
   - Reward: Loot from combat victory

2. **Flee** - Leave safely
   - Effect: None (empty effects list)
   - No risk, no reward

**Implementation File**: [dead_adventurer.rs](slay_the_spire/src/events/map_events/dead_adventurer.rs)

---

### Golden Idol (Act 1)

**Source**: [Slay the Spire Wiki - Golden Idol](https://slay-the-spire.fandom.com/wiki/Golden_Idol)

**Description**: "A gleaming golden idol sits atop a pedestal, surrounded by suspicious tiles. This is clearly a trap, but the idol looks valuable..."

**Choices**:
1. **Take the idol carefully** - Obtain a relic, chance of damage
   - Effect: `ObtainRandomRelic` (should be Golden Idol specifically)
   - TODO: Add 25-75% chance of damage based on dexterity
   - Golden Idol: Enemies drop 25% more gold

2. **Destroy the trap** - Take 25 damage, obtain a relic
   - Effects: `LoseHp(25)`, `ObtainRandomRelic` (should be Bloody Idol)
   - Guaranteed damage but guaranteed relic
   - Bloody Idol: Gain 5 gold when entering combat

3. **Leave** - No effect
   - Effect: None
   - Safe option

**Relic Details**:
- **Golden Idol**: Enemies drop 25% more gold
- **Bloody Idol**: Gain 5 gold when entering combat

**Implementation File**: [golden_idol.rs](slay_the_spire/src/events/map_events/golden_idol.rs)

---

### Shining Light (Act 1)

**Source**: [Slay the Spire Wiki - Shining_Light](https://slay-the-spire.fandom.com/wiki/Shining_Light)

**Description**: "Divine light offers to upgrade your cards."

**Choices**: Context-aware based on Max HP and Ascension
- **Upgrade cards** - Upgrade 2 cards from your deck
- **Become Protected** - Alternative option (varies by ascension)

**Implementation Details**:
- Uses `EventContext` for ascension-specific variations
- Ascension 15+: Different costs/benefits
- Card selection UI required for upgrade choice

**Implementation File**: [shining_light.rs](slay_the_spire/src/events/map_events/shining_light.rs)

---

### World of Goop (Act 1)

**Source**: [Slay the Spire Wiki - World of Goop](https://slay-the-spire.fandom.com/wiki/World_of_Goop)

**Description**: "A strange world made entirely of slime. It's oddly inviting..."

**Choices**:
- **Accept the gold** - Gain gold but receive Slimed status cards
- **Decline** - Leave without benefit or penalty

**Implementation Details**:
- Gold amount varies
- Slimed cards are added to deck (status cards)
- Trade-off between immediate resources and deck pollution

**Implementation File**: [world_of_goop.rs](slay_the_spire/src/events/map_events/world_of_goop.rs)

---

### Wing Statue (Act 1)

**Source**: [Slay the Spire Wiki - Wing Statue](https://slay-the-spire.fandom.com/wiki/Wing_Statue)

**Description**: "An ancient statue with a missing wing stands before you."

**Choices**:
- **Try to fix it** - Chance of relic or HP loss
- **Pray to it** - Alternative option
- **Leave** - No effect

**Implementation Details**:
- RNG-based outcomes
- Risk/reward decision

**Implementation File**: [wing_statue.rs](slay_the_spire/src/events/map_events/wing_statue.rs)

---

### Implemented Events - Choice Comparison Table

| Event | # Choices | Primary Reward | Risk/Cost | Special Features |
|-------|-----------|----------------|-----------|------------------|
| Big Fish | 3 | +5 Max HP, 1/3 Max HP heal, or relic | None | Fixed, safe choices |
| The Cleric | Variable | HP healing | Gold (scales with floor) | Context-aware pricing |
| Dead Adventurer | 2 | Combat rewards | Combat difficulty | Triggers Lagavulin fight |
| Golden Idol | 3 | Golden/Bloody Idol relic | Chance of damage or 25 HP | Two paths to different relics |
| Shining Light | 2+ | Upgrade 2 cards | Varies by ascension | Card selection UI needed |
| World of Goop | 2 | Gold | Slimed cards in deck | Deck pollution trade-off |
| Wing Statue | 3 | Relic | HP loss (RNG) | Random outcomes |

**Key Patterns**:
- **Safe Events**: Big Fish (all choices beneficial)
- **Trade Events**: The Cleric (gold for healing)
- **Risk Events**: Golden Idol, Wing Statue (chance of negative outcome)
- **Combat Events**: Dead Adventurer (optional combat encounter)
- **Deck Impact Events**: World of Goop (adds status cards)

---

## Event Tables by Category

### Shared Events - Common (14 Events)

These events can appear in Acts 1, 2, and 3.

#### A Note For Yourself
**Acts**: 1, 2, 3
**Choices**:
1. **Ignore** - Gain 75 (90 A15+) gold
2. **Write** - Choose a card to obtain

---

#### Bonfire Spirits
**Acts**: 1, 2, 3
**Choices**:
1. **Offer** - Lose all gold, obtain Spirit Poop relic
2. **Stoke** - Heal 10 HP
3. **Leave** - No effect

**Relic**: Spirit Poop (does nothing, flavor relic)

---

#### The Divine Fountain
**Acts**: 1, 2, 3
**Description**: Shrine that cleanses curses
**Choices**:
1. **Drink** - Remove all Curses from your deck
2. **Leave** - No effect

---

#### Duplicator
**Acts**: 1, 2, 3
**Description**: Ancient machine that creates copies
**Choices**:
1. **Duplicate** - Select a card in your deck to duplicate
2. **Leave** - No effect

---

#### Golden Shrine
**Acts**: 1, 2, 3
**Choices**:
1. **Pray** - Gain 100 (125 A15+) gold, lose 10% Max HP
2. **Destroy** - Gain Golden Idol relic (25% more gold from combat)
3. **Leave** - No effect

---

#### Lab
**Acts**: 1, 2, 3
**Description**: Abandoned laboratory with potions
**Choices**:
1. **Take** - Obtain 3 random potions, 12.5% chance to take 9-13 damage (3 times)
2. **Leave** - No effect

---

#### Match and Keep!
**Acts**: 1, 2, 3
**Description**: Card matching game
**Choices**:
1. **Play** - 50 gold to play
   - **Win**: Gain 200 (150 A15+) gold
   - **Lose**: Lose the 50 gold bet
2. **Leave** - No effect

**Mechanics**: Match 2 cards from 6 face-down cards (50% to win with optimal play)

---

#### Ominous Forge
**Acts**: 1, 2, 3
**Choices**:
1. **Stoke** - Upgrade all Strikes and Defends, lose 10% Max HP
2. **Leave** - No effect

---

#### Purifier
**Acts**: 1, 2, 3
**Description**: Shrine that removes cards
**Choices**:
1. **Pray** - Remove a card from your deck
2. **Leave** - No effect

---

#### Transmogrifier
**Acts**: 1, 2, 3
**Description**: Device that transforms cards
**Choices**:
1. **Pray** - Transform a card
2. **Leave** - No effect

---

#### Upgrade Shrine
**Acts**: 1, 2, 3
**Choices**:
1. **Pray** - Upgrade a card
2. **Leave** - No effect

---

#### We Meet Again!
**Acts**: 1, 2, 3
**Description**: Recurring merchant who remembers past encounters
**Choices** (First Encounter):
1. **Buy** - Lose 45 (50 A15+) gold, gain 10 Max HP
2. **Leave** - No effect

**Choices** (Second Encounter):
1. **Buy** - Lose 75 gold, remove a card
2. **Rob** - Lose 10 Max HP, gain 150 (175 A15+) gold
3. **Leave** - No effect

**Choices** (Third+ Encounter):
1. **Attack!** - Turns into an easy combat encounter, grants rewards

---

#### Wheel of Change
**Acts**: 1, 2, 3
**Description**: Mysterious wheel with random effects
**Choices**:
1. **Spin** - Random outcome:
   - 20%: Gain a random common relic
   - 16%: Transform 2 random cards
   - 16%: Lose all gold
   - 16%: Upgrade a random card
   - 16%: Gain 50 gold
   - 16%: Remove a random card
2. **Leave** - No effect

---

#### The Woman in Blue
**Acts**: 1, 2, 3
**Description**: Mysterious potion seller
**Choices**:
1. **Buy Potions** - Choose from 2-5 random potions at 50 gold each
2. **Leave** - No effect

**Note**: Potion selection refreshes as you buy

---

**Implementation Summary:**

| Event Name | Type | Implemented | # Choices | Key Features |
|------------|------|-------------|-----------|--------------|
| ❌ A Note For Yourself | Utility | No | 2 | Gold or card reward |
| ❌ Bonfire Spirits | Rest | No | 3 | Heal or spend gold for relic |
| ❌ The Divine Fountain | Shrine | No | 2 | Remove all curses |
| ❌ Duplicator | Shrine | No | 2 | Duplicate any card |
| ❌ Golden Shrine | Shrine | No | 3 | Gold for HP trade |
| ❌ Lab | Potion | No | 2 | 3 potions with damage risk |
| ❌ Match and Keep | Gamble | No | 2 | 50g bet for 200g reward |
| ❌ Ominous Forge | Upgrade | No | 2 | Upgrade basics for HP cost |
| ❌ Purifier | Shrine | No | 2 | Remove 1 card |
| ❌ Transmogrifier | Shrine | No | 2 | Transform 1 card |
| ❌ Upgrade Shrine | Shrine | No | 2 | Upgrade 1 card |
| ❌ We Meet Again! | Special | No | Varies | Changes on each encounter |
| ❌ Wheel of Change | Gamble | No | 2 | 6 random outcomes |
| ❌ The Woman in Blue | Shop | No | 2 | Buy potions |

**Implementation Notes:**
- Shrine events (Purifier, Transmogrifier, Upgrade Shrine, Divine Fountain) are simplest - card selection UI only
- Golden Shrine requires HP percentage calculation
- Match and Keep needs card-matching minigame
- We Meet Again! requires state tracking across multiple encounters
- Wheel of Change needs RNG with weighted outcomes
- Lab has 12.5% damage chance checked 3 times independently

---

### Shared Events - Semi-Common (2 Events)

#### Designer In-Spire
**Acts**: 1, 2, 3 (very rare)
**Description**: Developer room easter egg
**Choices**:
1. **Open** - Choose from 2 random Common cards, 1 Uncommon, and 1 Rare card
2. **Leave** - No effect

**Note**: Extremely rare event

---

#### Face Trader
**Acts**: 2, 3
**Choices**:
1. **Trade** - Lose 10% Max HP, gain Face of Cleric relic
2. **Leave** - No effect

**Relic**: Face of Cleric - Restore all HP on the 3rd rest site

---

**Implementation Summary:**

| Event Name | Type | Acts | Implemented | # Choices | Key Features |
|------------|------|------|-------------|-----------|--------------|
| ❌ Designer In-Spire | Special | 1, 2, 3 | No | 2 | Easter egg, rare cards |
| ❌ Face Trader | Trade | 2, 3 | No | 2 | Max HP for powerful relic |

---

### Act 1 Exclusive Events (12 Events)

#### Neow (Mandatory Starting Event)
**Acts**: 1 only (first event of every run)
**Description**: The whale who offers starting bonuses

**Choices**: Select 1 of 4 random blessings. See [Neow's Blessings](#neows-blessings-starting-event) section for complete list.

**Implementation Status**: ❌ Not implemented

---

#### Big Fish
**Implementation Status**: ✅ **Implemented**
See [Big Fish (Act 1)](#big-fish-act-1) in Implemented Events section

---

#### The Cleric
**Implementation Status**: ✅ **Implemented**
See [The Cleric (Act 1)](#the-cleric-act-1) in Implemented Events section

---

#### Dead Adventurer
**Implementation Status**: ✅ **Implemented**
See [Dead Adventurer (Act 1)](#dead-adventurer-act-1) in Implemented Events section

---

#### Golden Idol
**Implementation Status**: ✅ **Implemented**
See [Golden Idol (Act 1)](#golden-idol-act-1) in Implemented Events section

---

#### Hypnotizing Colored Mushrooms
**Acts**: 1
**Choices**:
1. **Eat** - Random outcome:
   - 30%: Heal 25 HP
   - 40%: Gain 1 random potion
   - 30%: Take 11-15 damage
2. **Leave** - No effect

**Implementation Status**: ❌ Not implemented

---

#### Living Wall
**Acts**: 1
**Choices**:
1. **Forget** - Remove a card from your deck
2. **Change** - Transform a card in your deck
3. **Grow** - Upgrade a card in your deck

**Implementation Status**: ❌ Not implemented

---

#### Scrap Ooze
**Acts**: 1
**Description**: Fight a Scrap Ooze in combat
**Choices**:
1. **Fight** - Enter combat against 4-5 Small Acid or Spike Slimes
   - Reward: Obtain a random relic
2. **Leave** - No effect

**Implementation Status**: ❌ Not implemented

---

#### Shining Light
**Implementation Status**: ✅ **Implemented**
See [Shining Light (Act 1)](#shining-light-act-1) in Implemented Events section

---

#### The Ssssserpent
**Acts**: 1
**Choices**:
1. **Agree** - Lose 50% of current gold (min 125g, 175g A15+), gain Ssserpent Head relic
2. **Disagree** - No effect

**Relic**: Ssserpent Head - Whenever you enter a ? room, gain 50 gold

**Implementation Status**: ❌ Not implemented

---

#### World of Goop
**Implementation Status**: ✅ **Implemented**
See [World of Goop (Act 1)](#world-of-goop-act-1) in Implemented Events section

---

#### Wing Statue
**Implementation Status**: ✅ **Implemented**
See [Wing Statue (Act 1)](#wing-statue-act-1) in Implemented Events section

---

**Implementation Summary:**

| Event Name | Type | Implemented | # Choices | Key Features |
|------------|------|-------------|-----------|--------------|
| ❌ Neow | Starting | No | 4 | Mandatory starting bonuses |
| ✅ Big Fish | Special | Yes | 3 | Max HP, heal, or relic |
| ✅ The Cleric | Heal | Yes | Variable | Gold for healing |
| ✅ Dead Adventurer | Combat | Yes | 2 | Lagavulin fight |
| ✅ Golden Idol | Puzzle | Yes | 3 | Risk for idol relics |
| ❌ Hypnotizing Mushrooms | Gamble | No | 2 | 30% heal/40% potion/30% damage |
| ❌ Living Wall | Special | No | 2 | Trade card for rare |
| ❌ Scrap Ooze | Combat | No | 2 | Combat for relic |
| ✅ Shining Light | Upgrade | Yes | 2+ | Upgrade cards |
| ❌ The Ssssserpent | Trade | No | 2 | Gold for ? room gold relic |
| ✅ World of Goop | Debuff | Yes | 2 | Gold for Slimed cards |
| ✅ Wing Statue | Puzzle | Yes | 3 | RNG relic or HP loss |

**Progress**: 7/12 Act 1 events implemented (58%)

**Event-Specific Relics:**
- **Golden Idol** (Golden Idol event): Enemies drop 25% more gold
- **Bloody Idol** (Golden Idol event): Gain 5 gold when entering combat
- **Ssserpent Head** (The Ssssserpent event): Gain 50 gold when entering ? rooms
- **Face of Cleric** (Face Trader event): Restore all HP on 3rd rest site

---

### Act 2 Exclusive Events (16 Events)

#### Ancient Writing
**Acts**: 2
**Choices**:
1. **Read** - Upgrade all Strikes and Defends in your deck
2. **Leave** - No effect

---

#### Augmenter
**Acts**: 2
**Choices**:
1. **Transform** - Transform 2 cards
2. **Relic** - Lose 25% (33% A15+) Max HP, gain J.A.X. relic
3. **Leave** - No effect

**Relic**: J.A.X. - At the start of combat, lose 3 HP and gain 2 Strength (3 Strength when upgraded)

---

#### The Colosseum
**Acts**: 2
**Choices**:
1. **Fight Nob** - Fight 2 Gremlin Nobs, gain 2000 gold (1000g A15+)
2. **Fight Slavers** - Fight 3 Slavers, gain Red Mask relic
3. **Leave** - No effect

**Relic**: Red Mask - At the start of combat, apply 1 Weak to ALL enemies

**Note**: Very difficult combat encounters

---

#### Council of Ghosts
**Acts**: 2
**Choices**:
1. **Accept** - Remove a card, gain 5 Apparition cards
2. **Refuse** - No effect

**Apparition**: Colorless Skill (0 cost) - Gain 1 Intangible. Exhaust. Ethereal.

**Note**: One of the most powerful events

---

#### Cursed Tome
**Acts**: 2
**Choices**:
1. **Read** - Gain 1 random relic and 1 random curse, OR gain 3 relics and 3 curses
2. **Leave** - No effect

**A15+ Changes**: Read option becomes gain 1 relic and 2 curses, OR gain 2 relics and 3 curses

---

#### Forgotten Altar
**Acts**: 2
**Choices**:
1. **Pray** - Lose 25% (37.5% A15+) Max HP, gain Mark of the Bloom relic
2. **Desecrate** - Gain 250 (275 A15+) gold
3. **Leave** - No effect

**Relic**: Mark of the Bloom - You can no longer heal (except by resting)

**Note**: Extremely risky relic

---

#### The Joust
**Acts**: 2
**Choices**:
1. **Fight** - Fight a tough enemy, gain random relic on victory
2. **Bet on yourself** - Bet 50 gold, fight same enemy, gain random relic + gold on victory
3. **Leave** - No effect

---

#### Knowing Skull
**Acts**: 2
**Choices**:
1. **Ask about the future** - Choose and remove a card from deck
2. **Ask about my self** - Gain 50 (75 A15+) gold, add 2 Regrets to deck
3. **I have a bad feeling** - Heal 25 (35 A15+) HP, lose 6 Max HP
4. **Leave** - No effect

**Regret**: Curse (1 cost) - Unplayable. Ethereal.

---

#### The Library
**Acts**: 2
**Choices**:
1. **Read** - Add 1 Apotheosis card (colorless rare) to deck
2. **Leave** - Choose and add 1 of 20 random cards to deck, OR skip

**Apotheosis**: Colorless Skill (2 cost) - Upgrade ALL cards in your deck. Exhaust.

---

#### Masked Bandits
**Acts**: 2
**Description**: Bandits ambush and rob you
**Choices**:
1. **Fight!** - Lose all gold, enter combat against 2-3 Bandits
   - Victory: Gain Red Mask relic
2. **Give them what you have** - Lose all gold, no combat

**Relic**: Red Mask - At the start of combat, apply 1 Weak to ALL enemies

---

#### The Mausoleum
**Acts**: 2
**Choices**:
1. **Open** - Gain Ruby Key relic
2. **Leave** - No effect

**Relic**: Ruby Key - Required to access Act 4

**Note**: No downside to taking it

---

#### The Nest
**Acts**: 2
**Choices**:
1. **Fight** - Fight 3 Snakes (Chosen, not random)
   - Victory: Gain Ritual Dagger card
2. **Take the Dagger** - Gain Ritual Dagger card, take 6 damage per card, 1 per relic owned
3. **Leave** - No effect

**Ritual Dagger**: Uncommon Attack (1 cost) - Deal 15 damage. Whenever this kills an enemy, permanently gain 3 (5) damage.

---

#### N'loth
**Acts**: 2
**Description**: Demon offers a trade
**Choices**:
1. **Give relic** - Lose a relic (you choose), gain a random relic
2. **Take relic** - Gain N'loth's Hungry Face relic
3. **Leave** - No effect

**Relic**: N'loth's Hungry Face - When you gain Max HP, lose 1 Max HP instead

**Note**: Only appears if you have at least 1 relic

---

#### Old Beggar
**Acts**: 2
**Choices**:
1. **Give 75g** - Lose 75 gold, gain random relic
2. **Refuse** - No effect

---

#### Pleading Vagrant
**Acts**: 2
**Choices**:
1. **Help** - Lose 75 (85 A15+) gold, no reward
2. **Refuse** - No effect

**Note**: This event has no positive outcome - it's a trap

---

#### Vampires(?)
**Acts**: 2
**Choices**:
1. **Accept** - Remove all Strikes, lose 30% Max HP, gain 5 Bite cards
2. **Refuse** - No effect

**Bite**: Colorless Attack (1 cost) - Deal 7 (8) damage. Heal 2 (3) HP.

**Note**: Extremely impactful deck transformation

---

**Implementation Summary:**

| Event Name | Type | Implemented | # Choices | Key Features |
|------------|------|-------------|-----------|--------------|
| ❌ Ancient Writing | Upgrade | No | 2 | Upgrade all basics |
| ❌ Augmenter | Upgrade | No | 3 | Transform or J.A.X. relic |
| ❌ The Colosseum | Combat | No | 3 | 2000g or Red Mask relic |
| ❌ Council of Ghosts | Special | No | 2 | 5 Apparition cards |
| ❌ Cursed Tome | Curse | No | 2 | Relics + curses trade |
| ❌ Forgotten Altar | Sacrifice | No | 3 | Bloom relic or gold |
| ❌ The Joust | Combat | No | 3 | Combat for relic |
| ❌ Knowing Skull | Trade | No | 4 | Multiple outcomes |
| ❌ The Library | Card | No | 2 | Apotheosis or 20 cards |
| ❌ Masked Bandits | Combat | No | 2 | Lose gold, get Red Mask |
| ❌ The Mausoleum | Special | No | 2 | Free Ruby Key |
| ❌ The Nest | Combat | No | 3 | Ritual Dagger acquisition |
| ❌ N'loth | Sacrifice | No | 3 | Relic trade |
| ❌ Old Beggar | Trade | No | 2 | 75g for relic |
| ❌ Pleading Vagrant | Special | No | 2 | Trap event (lose gold) |
| ❌ Vampires(?) | Transform | No | 2 | Become vampire |

**Progress**: 0/16 Act 2 events implemented (0%)

**Implementation Notes:**
- Many events grant unique event-specific relics
- The Colosseum and The Joust require elite-level combat encounters
- Council of Ghosts provides Apparition cards (colorless, gain Intangible)
- Vampires(?) changes deck composition significantly

**Event-Specific Relics:**
- **J.A.X.**: Lose 3 HP, gain 2(3) Strength (skill card as relic)
- **Mark of the Bloom**: Cannot heal HP except by Resting
- **Red Mask**: At combat start, apply 1 Weak to ALL enemies
- **Ritual Dagger**: Attack that permanently grows stronger with kills

**Event-Specific Cards:**
- **Apparition** (from Council of Ghosts): Colorless skill, gain 1 Intangible, Exhaust, Ethereal
- **Bite** (from Vampires): Colorless attack, deal damage and heal

---

### Act 3 Exclusive Events (8 Events)

#### Falling
**Acts**: 3
**Description**: Falling through the void
**Choices**:
1. **Skill** - Remove a card from your deck
2. **Attack** - Remove a card from your deck
3. **Strike** - Lose 10% Max HP
4. **Leave** (land safely) - No effect

**Note**: Each landing spot has slightly different flavor text but same mechanical effect (remove card or lose HP)

---

#### Mind Bloom
**Acts**: 3
**Description**: One of the most powerful events in the game
**Choices**:
1. **I Am War** - Fight an Act 1 or Act 2 Boss, gain a Boss Relic on victory
2. **I Am Rich** - Gain 999 gold
3. **I Am Healthy** - Heal to full HP, upgrade all cards in your deck

**Note**: All three choices are extremely powerful with no downside (except combat difficulty for War)

---

#### The Moai Head
**Acts**: 3
**Choices**:
1. **Give 333g** - Lose 333 gold, gain random relic
2. **Give 555g** - Lose 555 gold, gain Golden Idol relic
3. **Leave** - No effect

**Note**: Golden Idol from this event is the same as Act 1 Golden Idol event

---

#### Mysterious Sphere
**Acts**: 3
**Description**: Strange glowing orb with colored options
**Choices**:
1. **Red** - Gain Red Mask relic
2. **Blue** - Gain Frozen Eye relic
3. **Green** - Gain Runic Capacitor relic
4. **Prismatic** - Gain 1 random rare relic
5. **Leave** - No effect

**Relics**:
- **Red Mask**: Apply 1 Weak to ALL enemies at combat start
- **Frozen Eye**: When you draw a card, you can see the top 3 cards of your draw pile
- **Runic Capacitor**: Start each combat with 3 additional Focus

---

#### Secret Portal
**Acts**: 3
**Description**: Portal to Act 4
**Choices**:
1. **Enter** - Immediately go to Act 4 (skip Act 3 boss)
2. **Leave** - Continue normally

**Note**: Only way to access Act 4 besides getting all 3 keys

---

#### Sensory Stone
**Acts**: 3
**Description**: Stone showing memories, grants character-specific cards
**Choices**: Select 1 of 10 character-specific colorless cards

**Ironclad Options**:
- Battle Trance, Rampage, Offering, etc. (rare Ironclad cards as colorless)

**Silent Options**:
- Expertise, Burst, Tactician, etc.

**Defect Options**:
- Skim, Heatsinks, Amplify, etc.

**Watcher Options**:
- Battle Hymn, Conjure Blade, Swivel, etc.

**Note**: Very powerful since you get character cards without deck restrictions

---

#### Tomb of Lord Red Mask
**Acts**: 3
**Choices**:
1. **Open** - Fight 3 Elite enemies (Golden Mask, Priestess, Executioner)
   - Victory: Gain Red Mask relic
2. **Smash** - Take 35 (45 A15+) damage, gain Red Mask relic
3. **Leave** - No effect

**Relic**: Red Mask - Apply 1 Weak to ALL enemies at combat start

**Note**: Very difficult fight but guaranteed Red Mask

---

#### Winding Halls
**Acts**: 3
**Choices**:
1. **Madness** - Remove 2 cards from your deck
2. **Change** - Transform 2 cards
3. **Leave** - No effect

---

**Implementation Summary:**

| Event Name | Type | Implemented | # Choices | Key Features |
|------------|------|-------------|-----------|--------------|
| ❌ Falling | Special | No | 4 | Remove card or lose HP |
| ❌ Mind Bloom | Special | No | 3 | Boss relic/999g/full heal+upgrades |
| ❌ The Moai Head | Special | No | 3 | 333g or 555g for relics |
| ❌ Mysterious Sphere | Puzzle | No | 5 | Choose relic by color |
| ❌ Secret Portal | Special | No | 2 | Skip to Act 4 |
| ❌ Sensory Stone | Memory | No | 10 | Character-specific colorless cards |
| ❌ Tomb of Red Mask | Combat | No | 3 | Elite fight for Red Mask |
| ❌ Winding Halls | Special | No | 3 | Remove 2 or transform 2 |

**Progress**: 0/8 Act 3 events implemented (0%)

**Implementation Notes:**
- Mind Bloom is one of the most impactful events - all choices are extremely powerful
- Secret Portal requires Act 4 implementation
- Sensory Stone needs character-specific card pools
- Mysterious Sphere offers choice between specific powerful relics
- These events are generally more powerful than Acts 1-2

---

### Shrine Events (5 Special Events)

Shrine events are simple utility events focused on deck manipulation.

| Shrine Name | Effect | Appears In | Implemented |
|-------------|--------|------------|-------------|
| ❌ Purifier | Remove 1-2 cards from deck | Acts 1, 2, 3 | No |
| ❌ Transmogrifier | Transform 1-2 cards | Acts 1, 2, 3 | No |
| ❌ Upgrade Shrine | Upgrade 1 card | Acts 1, 2, 3 | No |
| ❌ The Divine Fountain | Remove all curses | Acts 1, 2, 3 | No |
| ❌ Golden Shrine | Trade HP for gold or relic | Acts 1, 2, 3 | No |

**Implementation Notes:**
- Shrines are the simplest events to implement
- Direct card/deck manipulation with no RNG
- Essential for deck optimization

---

## Special Event Mechanics

### Neow's Blessings (Starting Event)

Neow is the first event of every run. The player chooses 1 blessing from 4 random options.

#### Blessing Categories

**Category 1: Simple Rewards (No Cost)**
1. **Choose a card** - Choose 1 card from 3 random cards (any rarity) to add to deck
2. **Remove a card** - Remove 1 card from your starting deck
3. **Upgrade a card** - Upgrade 1 card in your deck
4. **Gain gold** - Gain 100 gold
5. **Obtain common relic** - Gain 1 random common relic
6. **Gain Max HP** - Gain 6-8 Max HP (Ascension dependent)
7. **Enemies weakened** - Gain Neow's Lament relic (Enemies in first 3 combats have 1 HP)

**Category 2: Better Rewards (No Cost) - A1-A14**
8. **Choose a rare card** - Choose 1 rare card from 3 options
9. **Obtain rare relic** - Gain 1 random rare relic
10. **Transform 2 cards** - Transform 2 cards in your deck
11. **Remove 2 cards** - Remove 2 cards from your deck

**Category 3: Exceptional Rewards with Cost (All Ascensions)**
12. **Rare relic + curse** - Gain 1 random rare relic, add 1 random curse to deck
13. **Gold for HP** - Gain 250 gold, lose 7 Max HP
14. **Rare colorless + HP** - Choose 1 rare colorless card, lose 10% Max HP
15. **Boss relic** - Lose starting relic, gain 1 random boss relic

#### Blessing Pool by Ascension

**A0-A14**:
- 4 random blessings selected from all 15 options
- Category 2 (better rewards) available

**A15+**:
- 4 random blessings from Categories 1 and 3 only
- Category 2 (better no-cost rewards) removed
- Makes early game slightly harder

#### Special Mechanics

**Neow's Lament Relic**:
- Enemies in the next 3 combats have 1 HP
- Relic disappears after 3 combats
- Makes early game extremely safe
- One of the most popular choices for risky strategies

**Boss Relic Swap**:
- Trade your starting relic (Burning Blood, Ring of the Snake, Cracked Core, Pure Water)
- Gain a random boss relic
- High variance - can be amazing or terrible
- Only appears in Category 3 (with cost)

#### Strategy Notes

**Most Popular Choices**:
1. **Remove 2 cards** (A0-A14) - Deck thinning is very powerful
2. **Rare relic + curse** - High value if you can remove curse later
3. **Transform 2 cards** (A0-A14) - Remove Strikes/Defends for better cards
4. **Boss relic swap** - High risk, high reward

**Least Popular**:
- **Gain gold** - 100 gold is relatively weak compared to other options
- **Upgrade 1 card** - Limited impact early game

**Implementation Status**: ❌ Not implemented

**Implementation Requirements**:
- UI to display 4 random blessing choices
- RNG to select blessings based on ascension
- Neow's Lament relic (temporary, combat-counting effect)
- Boss relic swap mechanism
- Card selection UI (for card/relic choices)

---

### Mind Bloom Choices (Act 3)

See detailed information in [Mind Bloom](#mind-bloom) under Act 3 Exclusive Events section.

Mind Bloom is one of the most powerful events in the game with 3 exceptional choices:
- **I Am War**: Fight boss, gain boss relic
- **I Am Rich**: Gain 999 gold
- **I Am Healthy**: Full heal + upgrade ALL cards

All choices are extremely powerful with no real downside.

---

## Event Implementation Framework

### Required Systems

#### High Priority
- ❌ **Event room system** - Trigger events at ? nodes
- ❌ **Choice UI** - Present options and handle selection
- ❌ **Card selection UI** - Choose cards from deck (upgrade, remove, transform)
- ❌ **Relic acquisition** - Grant relics from events
- ❌ **HP modification** - Heal or damage player
- ❌ **Gold system** - Give and take gold
- ❌ **Curse system** - Add curses to deck

#### Medium Priority
- ❌ **Combat triggers** - Some events start combat encounters
- ❌ **Card transformation** - Transform specific cards
- ❌ **Potion system** - Grant potions from events
- ❌ **State tracking** - Remember event history (We Meet Again!)
- ❌ **RNG outcomes** - Random event results (Wheel of Change, mushrooms)

#### Low Priority
- ❌ **Special card generation** - Event-specific cards (Apparition, Bite)
- ❌ **Temporary effects** - Neow's Lament (3 combat duration)
- ❌ **Easter eggs** - Designer In-Spire
- ❌ **Act 4 access** - Secret Portal integration

---

## Recommended Implementation Order

### Phase 1: Simple Shrines (5 events)
1. **Upgrade Shrine** - Upgrade 1 card (simplest)
2. **Purifier** - Remove 1 card from deck
3. **The Divine Fountain** - Remove all curses
4. **Transmogrifier** - Transform 1 card
5. **Golden Shrine** - Trade HP for gold

**Why first:** Simple UI, direct effects, no RNG, no combat

---

### Phase 2: Neow's Blessings (1 event)
1. **Neow** - Starting event with multiple blessing options

**Why second:** Required for runs to start, teaches event choice system

---

### Phase 3: Simple Trade Events (6 events)
1. **The Cleric** - Heal for gold
2. **Old Beggar** - Trade gold for relic
3. **The Ssssserpent** - Trade gold for relic (Ssserpent Head)
4. **The Moai Head** - Trade gold for relic
5. **Face Trader** - Trade Max HP for relic (Face of Cleric)
6. **The Woman in Blue** - Buy potions

**Why third:** Simple resource trades, no complex mechanics

---

### Phase 4: Upgrade/Modification Events (4 events)
1. **Shining Light** - Upgrade 2 cards
2. **Ancient Writing** - Upgrade all Strikes/Defends
3. **Ominous Forge** - Upgrade all Strikes/Defends, lose HP
4. **Winding Halls** - Remove or transform card

**Why fourth:** Builds on shrine mechanics with variations

---

### Phase 5: Combat Events (8 events)
1. **Big Fish** - Simple combat for reward
2. **Scrap Ooze** - Combat for relics
3. **Dead Adventurer** - Optional Lagavulin fight
4. **The Nest** - Combat or take relic
5. **Masked Bandits** - Forced combat
6. **The Colosseum** - Arena combat
7. **The Joust** - Tournament combat
8. **Mind Bloom (I Am War)** - Boss combat option

**Why fifth:** Requires combat system integration

---

### Phase 6: Complex/RNG Events (remaining events)
- Gamble events (Match and Keep, Wheel of Change)
- Transform events (Vampires, Augmenter)
- Puzzle events (Golden Idol, Wing Statue)
- Special mechanics (Council of Ghosts, Sensory Stone)

**Why last:** Complex mechanics, character-specific content, special cards

---

## Event-Specific Content Summary

### Relics Granted by Events

| Relic Name | Source Event | Effect |
|------------|--------------|--------|
| Golden Idol | Golden Idol | Enemies drop 25% more gold |
| Bloody Idol | Golden Idol (alternate) | Gain 5 gold when entering combat |
| Ssserpent Head | The Ssssserpent | Gain 50 gold when entering ? room |
| Face of Cleric | Face Trader | Heal all HP at 3rd Rest Site |
| Spirit Poop | Bonfire Spirits | No effect (joke relic) |
| J.A.X. | Augmenter | Lose 3 HP, gain 2(3) Strength (colorless skill) |
| Mark of the Bloom | Forgotten Altar | Cannot heal except by Resting |
| Red Mask | Masked Bandits | Apply 1 Weak to ALL enemies at combat start |
| Ritual Dagger | The Nest | Attack card that grows with kills |
| Neow's Lament | Neow | Enemies in first 3 combats have 1 HP |
| Cultist Headpiece | Various | Gain 1 Ritual at combat start |
| Gremlin Visage | Various | Start combat with 1 Weak |
| N'loth's Gift | N'loth (positive) | Gain 3 Strength |
| N'loth's Hungry Face | N'loth (negative) | Lose 1 Max HP whenever gaining Max HP |
| Warped Tongs | Various | Upgrade random card in hand at combat start |

### Cards Granted by Events

| Card Name | Source Event | Type | Effect |
|-----------|--------------|------|--------|
| Apparition | Council of Ghosts | Colorless Skill | Gain 1 Intangible. Exhaust. Ethereal |
| Bite | Vampires(?) | Colorless Attack | Deal 7(8) damage. Heal 2(3) HP |
| Battle Hymn | Sensory Stone | Watcher Uncommon | Add Smite to hand |
| Crescendo | Sensory Stone | Watcher Common | Gain 1(2) Mantra |
| Discovery | Various | Colorless Skill | Discover colorless card |

---

## Implementation Notes

### Character-Specific Considerations

Some events have character-specific variations:
- **Sensory Stone** grants different cards for each character
- **Vampires** may have character-specific implications
- Some event rewards (Focus reduction, etc.) only affect certain characters

### Ascension Scaling

Many events change at higher Ascensions:
- More difficult trade-offs
- Higher costs
- Better or worse outcomes
- Different encounter rates

### State Requirements

**Events Need:**
- Current Act tracking
- Player HP/Max HP
- Player gold
- Player deck (for removal, transform, upgrade)
- Player relics
- Combat system integration
- Potion system
- Curse system

---

## Recently Implemented Events

**7 Act 1 Events** (as of initial implementation):
- Big Fish
- The Cleric
- Dead Adventurer
- Golden Idol
- Shining Light
- World of Goop
- Wing Statue

---

## How to Add New Events

### Step-by-Step Guide

1. **Add Event Variant** to [map_events/mod.rs](slay_the_spire/src/events/map_events/mod.rs):
   ```rust
   pub enum MapEvent {
       // ... existing events
       NewEventName,
   }
   ```

2. **Create Event Module** (e.g., `new_event_name.rs`):
   ```rust
   use crate::game::effect::Effect;
   use crate::events::map_events::{EventChoice, EventOutcome};

   pub fn new_event_choices() -> Vec<EventChoice> {
       vec![
           EventChoice {
               text: "Choice 1 text".to_string(),
               outcome: EventOutcome::Effects(vec![
                   Effect::Heal(10),
               ]),
           },
           EventChoice {
               text: "Choice 2 text".to_string(),
               outcome: EventOutcome::Effects(vec![
                   Effect::LoseHp(5),
                   Effect::ObtainRandomRelic,
               ]),
           },
       ]
   }

   pub fn new_event_description() -> &'static str {
       "Event description text here..."
   }
   ```

3. **Add to mod.rs** imports and match statements:
   ```rust
   mod new_event_name;

   impl MapEvent {
       pub fn get_choices(&self) -> Vec<EventChoice> {
           match self {
               // ... existing events
               MapEvent::NewEventName => new_event_name::new_event_choices(),
           }
       }

       pub fn get_description(&self) -> &'static str {
           match self {
               // ... existing events
               MapEvent::NewEventName => new_event_name::new_event_description(),
           }
       }
   }
   ```

4. **Add to event pool** in `sample_sls_event()`:
   ```rust
   let act1_events = vec![
       // ... existing events
       MapEvent::NewEventName,
   ];
   ```

### Best Practices

**Choice Text Guidelines**:
- Be clear about what the player gets/loses
- Include numbers when relevant: "Heal 20 HP" not just "Heal"
- Use parentheses for effects: "Accept (Gain 50 gold, lose 5 HP)"
- Format: "[Action] ([Effects])"

**Effect Combinations**:
```rust
// Multiple effects execute in order
EventOutcome::Effects(vec![
    Effect::LoseHp(10),      // First take damage
    Effect::GainGold(75),    // Then gain gold
    Effect::ObtainRandomRelic, // Then get relic
])
```

**Context-Aware Events**:
```rust
// Accept EventContext parameter for dynamic choices
pub fn event_choices(ctx: &EventContext) -> Vec<EventChoice> {
    let heal_cost = ctx.floor * 5;  // Cost scales with floor
    vec![
        EventChoice {
            text: format!("Heal (Costs {} gold)", heal_cost),
            outcome: EventOutcome::Effects(vec![
                Effect::LoseGold(heal_cost),
                Effect::Heal(ctx.player_max_hp / 2),
            ]),
        },
    ]
}
```

**Multi-Stage Events**:
```rust
// First choice leads to more choices
EventChoice {
    text: "Open the chest".to_string(),
    outcome: EventOutcome::NextChoices(vec![
        EventChoice {
            text: "Take the gold".to_string(),
            outcome: EventOutcome::Effects(vec![Effect::GainGold(100)]),
        },
        EventChoice {
            text: "Take the relic".to_string(),
            outcome: EventOutcome::Effects(vec![Effect::ObtainRandomRelic]),
        },
    ]),
}
```

**Testing**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_has_correct_choices() {
        let choices = new_event_choices();
        assert_eq!(choices.len(), 3);
        assert!(choices[0].text.contains("Expected text"));
    }
}
```

### Common Event Patterns

**Simple Trade Event** (gold for benefit):
```rust
vec![
    EventChoice {
        text: "Pay 50 gold (Heal 20 HP)".to_string(),
        outcome: EventOutcome::Effects(vec![
            Effect::LoseGold(50),
            Effect::Heal(20),
        ]),
    },
    EventChoice {
        text: "Leave".to_string(),
        outcome: EventOutcome::Effects(vec![]),
    },
]
```

**Risk/Reward Event** (chance of good or bad):
```rust
vec![
    EventChoice {
        text: "Take the risk (50% chance of relic or 10 damage)".to_string(),
        outcome: EventOutcome::Effects(vec![
            Effect::RandomOutcome(0.5,
                vec![Effect::ObtainRandomRelic],
                vec![Effect::LoseHp(10)]
            ),
        ]),
    },
    EventChoice {
        text: "Play it safe".to_string(),
        outcome: EventOutcome::Effects(vec![]),
    },
]
```

**Combat Event**:
```rust
vec![
    EventChoice {
        text: "Fight (Enter combat)".to_string(),
        outcome: EventOutcome::Effects(vec![
            Effect::TriggerCombatEvent,
        ]),
    },
    EventChoice {
        text: "Flee".to_string(),
        outcome: EventOutcome::Effects(vec![]),
    },
]
```

---

## References

- [Slay the Spire Wiki - Events](https://slay-the-spire.fandom.com/wiki/Events)
- [Event List - Official Wiki](https://slaythespire.wiki.gg/wiki/Events)
