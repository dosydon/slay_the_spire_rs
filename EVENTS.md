# Events Implementation Status

This document tracks all events in Slay the Spire and their implementation status.

## Summary

- ❌ **0 events implemented**
- 📋 **60+ total events** across all acts
- 🎯 **Implementation Progress: 0%**

**Event Distribution:**
- **Shared Events**: 16 (appear in multiple acts)
- **Act 1 Exclusive**: 12 events
- **Act 2 Exclusive**: 16 events
- **Act 3 Exclusive**: 8 events
- **Shrines**: 5 events (upgrade/removal/transformation)

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

## Event Tables by Category

### Shared Events - Common (14 Events)

These events can appear in Acts 1, 2, and 3.

| Event Name | Type | Implemented | Description | Typical Rewards/Costs |
|------------|------|-------------|-------------|----------------------|
| ❌ A Note For Yourself | Utility | No | Find a note with advice/resources | Gold, card removal, or minor benefit |
| ❌ Bonfire Spirits | Rest | No | Spirit offers to help at a bonfire | HP heal or relic (Spirit Poop) |
| ❌ The Divine Fountain | Shrine | No | Fountain that removes curses | Remove all curses, or gain relic |
| ❌ Duplicator | Shrine | No | Machine that duplicates a card | Duplicate one card in deck |
| ❌ Golden Shrine | Shrine | No | Golden shrine with offerings | Gain gold for HP, or gain relic |
| ❌ Lab | Potion | No | Abandoned laboratory | Gain potions with risk of damage |
| ❌ Match and Keep | Gamble | No | Matching card game | Win gold or lose HP |
| ❌ Ominous Forge | Upgrade | No | Mysterious forge | Upgrade all Strikes/Defends, lose HP |
| ❌ Purifier | Shrine | No | Shrine that purifies deck | Remove 1-2 cards from deck |
| ❌ Transmogrifier | Shrine | No | Device that transforms | Transform 1-2 cards |
| ❌ Upgrade Shrine | Shrine | No | Shrine that upgrades cards | Upgrade 1 card |
| ❌ We Meet Again! | Special | No | Encounter recurring character | Various outcomes based on history |
| ❌ Wheel of Change | Gamble | No | Spin wheel for random effect | Random positive or negative effect |
| ❌ The Woman in Blue | Shop | No | Mysterious merchant | Buy special potions |

**Implementation Notes:**
- Shrine events are the simplest (direct card/deck manipulation)
- Gamble events require RNG systems
- "We Meet Again!" requires state tracking across runs

---

### Shared Events - Semi-Common (2 Events)

| Event Name | Type | Acts | Implemented | Description | Typical Rewards/Costs |
|------------|------|------|-------------|-------------|----------------------|
| ❌ Designer In-Spire | Special | 1, 2, 3 | No | Developer room easter egg | Varies widely |
| ❌ Face Trader | Trade | 2, 3 | No | Mysterious trader wants your face | Trade Max HP for relic (Face of Cleric) |

**Implementation Notes:**
- Designer In-Spire is a rare easter egg event
- Face Trader only appears in Acts 2-3

---

### Act 1 Exclusive Events (12 Events)

| Event Name | Type | Implemented | Description | Typical Rewards/Costs |
|------------|------|-------------|-------------|----------------------|
| ❌ Neow | Starting | No | First event, choose starting bonus | Various starting bonuses |
| ❌ Big Fish | Combat | No | Fight a big fish or leave it | Relic or potion from combat |
| ❌ The Cleric | Heal | No | Cleric offers healing services | Heal for gold cost |
| ❌ Dead Adventurer | Combat | No | Find dead adventurer's belongings | Fight awakened Lagavulin or flee |
| ❌ Golden Idol | Puzzle | No | Steal golden idol trap | Gain relic (Golden/Bloody Idol) or take damage |
| ❌ Hypnotizing Colored Mushrooms | Gamble | No | Eat strange mushrooms | Random effect (buff, debuff, or nothing) |
| ❌ Living Wall | Special | No | Mysterious living wall | Trade card for relic |
| ❌ Scrap Ooze | Combat | No | Encounter scrap ooze | Fight for relics |
| ❌ Shining Light | Upgrade | No | Divine light offers upgrade | Upgrade 2 cards or become Protected |
| ❌ The Ssssserpent | Trade | No | Giant serpent offers deal | Trade gold for relic (Ssserpent Head) |
| ❌ World of Goop | Debuff | No | Slime world debuffs you | Gain gold but add Slimed cards |
| ❌ Wing Statue | Puzzle | No | Statue with missing wing | Gain relic or lose HP |

**Implementation Notes:**
- Neow is mandatory first event with special starting bonuses
- Several require combat integration (Big Fish, Dead Adventurer, Scrap Ooze)
- Golden Idol grants event-specific relics

**Event-Specific Relics:**
- **Golden Idol**: Enemies drop 25% more gold
- **Bloody Idol**: Gain 5 gold when entering combat
- **Ssserpent Head**: Whenever you enter a ? room, gain 50 gold

---

### Act 2 Exclusive Events (16 Events)

| Event Name | Type | Implemented | Description | Typical Rewards/Costs |
|------------|------|-------------|-------------|----------------------|
| ❌ Ancient Writing | Upgrade | No | Ancient text with knowledge | Upgrade all Strikes and Defends |
| ❌ Augmenter | Upgrade | No | Augmentation machine | Transform or lose HP for relic (J.A.X.) |
| ❌ The Colosseum | Combat | No | Arena combat for rewards | Fight 2 Nobs or 3 Slavers for gold/relic |
| ❌ Council of Ghosts | Special | No | Ghost council offers power | Gain Apparition cards or refuse |
| ❌ Cursed Tome | Curse | No | Cursed book with power | Gain relics but also curses |
| ❌ Forgotten Altar | Sacrifice | No | Altar demands sacrifice | Gain relic for losing Max HP (Mark of the Bloom) |
| ❌ The Joust | Combat | No | Jousting tournament | Face powerful enemy for rewards |
| ❌ Knowing Skull | Trade | No | Skull offers forbidden knowledge | Choose between several outcomes |
| ❌ The Library | Card | No | Library with books/cards | Add cards to deck or skip |
| ❌ Masked Bandits | Combat | No | Bandits rob you | Lose all gold, fight for relic (Red Mask) |
| ❌ The Mausoleum | Special | No | Tomb with coffin | Open coffin for relic or leave (Ruby Key) |
| ❌ The Nest | Combat | No | Snake nest | Fight or take relic (Ritual Dagger) |
| ❌ N'loth | Sacrifice | No | Demon wants sacrifice | Trade relic for relic or gain negative relic |
| ❌ Old Beggar | Trade | No | Beggar wants gold | Trade gold for relic |
| ❌ Pleading Vagrant | Special | No | Vagrant pleads for help | Give gold for nothing or refuse |
| ❌ Vampires(?) | Transform | No | Vampires offer transformation | Become vampire (Bite cards, lose Max HP) |

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

| Event Name | Type | Implemented | Description | Typical Rewards/Costs |
|------------|------|-------------|-------------|----------------------|
| ❌ Falling | Special | No | Falling through void | Choose where to land for different effects |
| ❌ Mind Bloom | Special | No | Powerful mind-altering event | Choose between I Am War, I Am Rich, I Am Healthy |
| ❌ The Moai Head | Special | No | Giant stone head | Trade gold for relic or leave |
| ❌ Mysterious Sphere | Puzzle | No | Strange glowing sphere | Choose color for different relics |
| ❌ Secret Portal | Special | No | Portal to secret area | Enter portal to fight Act 4 boss early |
| ❌ Sensory Stone | Memory | No | Stone with memories | Gain colorless card based on memory |
| ❌ Tomb of Lord Red Mask | Special | No | Tomb of powerful lord | Take relic, fight, or leave |
| ❌ Winding Halls | Special | No | Choose path through halls | Madness (remove card) or Change (transform) |

**Implementation Notes:**
- Mind Bloom is one of the most impactful events (Boss relic, 999 gold, or full heal)
- Secret Portal allows early Act 4 access
- Sensory Stone grants character-specific colorless cards
- These events are generally more powerful than earlier acts

**Event-Specific Rewards:**
- Mind Bloom provides one of three major benefits
- Mysterious Sphere can grant various relics based on color choice
- Sensory Stone grants Watcher cards (Battle Hymn, Crescendo, etc.)

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

Neow is the first event of every run and offers 4 random starting bonuses:

**Common Blessings:**
- Choose a card to obtain
- Remove a card from deck
- Upgrade a card
- Gain 100 gold
- Obtain a random common relic
- Max HP +8
- Enemies in first 3 combats have 1 HP (Neow's Lament relic)

**Rare Blessings (higher Ascension):**
- Choose a rare card
- Obtain a random rare relic
- Transform 2 cards
- Remove 2 cards from deck

**Penalty Blessings (strong benefit with cost):**
- Obtain a random rare relic, obtain 1 curse
- Obtain 250 gold, lose 7 Max HP
- Choose a rare colorless card, lose 10% Max HP

**Implementation Priority:**
- High (required for runs to start)
- Must implement blessing selection system
- Neow's Lament relic (temporary effect for 3 combats)

---

### Mind Bloom Choices (Act 3)

Mind Bloom offers 3 powerful choices:

1. **I Am War**
   - Fight a boss from Act 1 or 2
   - Gain a Boss relic upon victory
   - No downside if you win

2. **I Am Rich**
   - Gain 999 gold immediately
   - No downside

3. **I Am Healthy**
   - Heal to full HP
   - Upgrade all cards in deck
   - No downside

**Implementation Notes:**
- One of the strongest events in the game
- Requires boss combat integration for "I Am War"
- "I Am Healthy" requires full deck upgrade functionality

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

None yet - events not implemented.

---

## References

- [Slay the Spire Wiki - Events](https://slay-the-spire.fandom.com/wiki/Events)
- [Event List - Official Wiki](https://slaythespire.wiki.gg/wiki/Events)
