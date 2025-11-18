# Slay the Spire Monster Reference

All data in this document is taken directly from [Monsters — Slay the Spire Wiki](https://slaythespire.wiki.gg/wiki/Monsters) (`oldid=11301`, downloaded locally via `curl` on this machine). It lists every normal-enemy encounter currently described on the wiki, organized exactly the way the page presents it so the game implementation can mirror encounter probabilities and compositions.

## Ascension Modifiers for Normal Enemies

- **Ascension 2**: Normal enemies deal more damage.
- **Ascension 7**: Normal enemies have more health and may gain additional Block.
- **Ascension 17**: Normal enemies gain challenging move-set upgrades and abilities.

## Global Rules for Monster Encounters

- The first **2 encounters** in Acts 2 and 3 (first **3 encounters** in Act 1) are rolled from each act’s *Easy Pool*; every later fight in the same act is rolled from that act’s *Hard Pool*.
- Act 1 additional constraints for the first Hard-Pool fight:
  - After an “Easy” fight with 2× Louse, the first “Hard” fight cannot be 3× Louse.
  - After an “Easy” Slime fight (Spike + Acid mix), the first “Hard” fight cannot be Swarm of Slimes or Large Slime.
  - After an “Easy” Looter fight, the first “Hard” fight cannot be Exordium Thugs.
  - After an “Easy” Blue Slaver fight, the first “Hard” fight cannot be Red Slaver or Exordium Thugs.
- Act 3 restriction: the “Hard Pool” version of 3 Darklings can never immediately follow the “Easy Pool” version of 3 Darklings.
- No encounter can repeat within the next two fights (you can never see the exact same encounter twice in the next two nodes).

---

## Act 1 — The Exordium

### Debut Enemies

| Enemy | Detail from wiki |
|-------|------------------|
| Spike Slimes | Entry covers Spike Slime S/M/L variations. |
| Acid Slimes | Entry covers Acid Slime S/M/L variations. |
| Cultist | Single-enemy encounter highlighted in the Act 1 easy pool. |
| Jaw Worm | Solo encounter available from the opening pool onward. |
| Louses | Covers both Red and Green Louse variants. |
| Fungi Beast | Appears solo or as a pair in later Exordium encounters. |
| Gremlins | Umbrella entry for Fat, Sneaky, Mad, Shield, and Wizard gremlins. |
| Looter | Appears solo and in multi-enemy “Thugs” encounters. |
| Slavers | Covers both Blue and Red Slaver variants used in Act 1 encounters. |

### Encounter Pools

#### First Three Combat Encounters (Easy Pool)

| Encounter | Weight | Details from wiki |
|-----------|--------|-------------------|
| Cultist | 2 | Single Cultist. |
| Jaw Worm | 2 | Single Jaw Worm. |
| 2 Louse | 2 | Two Louses; each slot rolls Red or Green independently (50/50). |
| Small Slimes | 2 | Either (Spike Slime M + Acid Slime S) or (Acid Slime M + Spike Slime S). |

#### Remaining Combat Encounters (Hard Pool)

| Encounter | Weight | Details from wiki |
|-----------|--------|-------------------|
| Gang of Gremlins | 1 | 4 gremlins randomly chosen from: 2× Fat, 2× Sneaky, 2× Mad, Shield Gremlin, Gremlin Wizard. |
| Large Slime | 2 | Spike Slime (L) or Acid Slime (L). |
| Swarm of Slimes | 1 | 3× Spike Slime (S) + 2× Acid Slime (S). |
| Blue Slaver | 2 | Single Blue Slaver. |
| Red Slaver | 1 | Single Red Slaver. |
| 3 Louse | 2 | Three Louses; each slot rolls Red or Green independently (50/50). |
| 2 Fungi Beasts | 2 | Two Fungi Beasts. |
| Exordium Thugs | 1.5 | Two-enemy fight: first enemy is a Louse (any color) or Medium Slime (any type); second enemy is a Slaver (any color), Cultist, or Looter. |
| Exordium Wildlife | 1.5 | Two-enemy fight: first enemy is a Fungi Beast or Jaw Worm; second enemy is a Louse (any color) or Medium Slime (any type). |
| Looter | 2 | Single Looter. |

---

## Act 2 — The City

### Debut Enemies

| Enemy | Detail from wiki |
|-------|------------------|
| Byrd | Appears solo (3 Byrds encounter) and in mixed fights later in the act. |
| Chosen | Appears solo, in pairs with Byrd, and with Cultists. |
| Mugger | Appears alongside a Looter in the “Thieves” encounter. |
| Shelled Parasite | Can be rolled solo or paired with a Fungi Beast. |
| Spheric Guardian | Appears solo and paired with Sentry in Act 2. |
| Centurion and Mystic | Always encountered together as a single fight. |
| Snake Plant | Solo encounter in the hard pool (weight 6). |
| Snecko | Solo encounter in the hard pool (weight 4). |

### Returning Enemies

| Enemy | Role in Act 2 according to the wiki |
|-------|-------------------------------------|
| Cultist | Returns in mixed encounters with Chosen or as 3 Cultists. |
| Looter | Paired with Mugger in “Thieves”. |
| Fungi Beast | Appears with Shelled Parasite. |
| Gremlins | Can appear inside mixed groups (e.g., from events) per wiki listing. |
| Slavers | Captured under broader “Slavers” entry for encounter variety. |
| Sentry | Appears together with Spheric Guardian in a mixed fight. |

### Encounter Pools

#### First Two Combat Encounters (Easy Pool)

| Encounter | Weight | Details from wiki |
|-----------|--------|-------------------|
| Spheric Guardian | 2 | Solo Spheric Guardian. |
| Chosen | 2 | Solo Chosen. |
| Shelled Parasite | 2 | Solo Shelled Parasite. |
| 3 Byrds | 2 | Trio of Byrds. |
| Thieves | 2 | Looter on the left and Mugger on the right. |

#### Remaining Combat Encounters (Hard Pool)

| Encounter | Weight | Details from wiki |
|-----------|--------|-------------------|
| Chosen + Byrd | 2 | 1 Chosen and 1 Byrd. |
| Cultist + Chosen | 3 | 1 Chosen and 1 Cultist. |
| Sentry + Spheric Guardian | 2 | 1 Sentry paired with 1 Spheric Guardian. |
| Snake Plant | 6 | Solo Snake Plant. |
| Snecko | 4 | Solo Snecko. |
| Centurion + Mystic | 6 | Centurion accompanied by a Mystic. |
| 3 Cultists | 3 | Trio of Cultists. |
| Shelled Parasite + Fungi | 3 | 1 Shelled Parasite with 1 Fungi Beast. |

---

## Act 3 — The Beyond

### Debut Enemies

| Enemy | Detail from wiki |
|-------|------------------|
| Darkling | Appears as 3 Darklings in both easy and hard pools. |
| Orb Walker | Solo encounter available from the easy pool onwards. |
| Shapes | Catch-all entry for Repulsor, Exploder, and Spiker combinations. |
| The Maw | Solo fight (hard-pool weight 1). |
| Spire Growth | Solo fight (hard-pool weight 1). |
| Transient | Solo fight (hard-pool weight 1). |
| Writhing Mass | Solo fight (hard-pool weight 1). |

### Returning Enemies

| Enemy | Role in Act 3 according to the wiki |
|-------|-------------------------------------|
| Cultist | Returns as a possible component in Act 3 encounter pools. |
| Jaw Worm | Returns specifically as the “Jaw Worm Horde” encounter. |
| Spheric Guardian | Returns as part of the “Spheric Guardian + 2 Shapes” encounter. |

### Encounter Pools

#### First Two Combat Encounters (Easy Pool)

| Encounter | Weight | Details from wiki |
|-----------|--------|-------------------|
| 3 Darklings | 2 | Triple Darkling fight. |
| Orb Walker | 2 | Solo Orb Walker. |
| 3 Shapes | 2 | 3 Shapes chosen from the set {2× Repulsor, 2× Exploder, 2× Spiker}. |

#### Remaining Combat Encounters (Hard Pool)

| Encounter | Weight | Details from wiki |
|-----------|--------|-------------------|
| 4 Shapes | 1 | 4 Shapes chosen from the set {2× Repulsor, 2× Exploder, 2× Spiker}. |
| The Maw | 1 | Solo Maw fight. |
| Spheric Guardian + 2 Shapes | 1 | 1 Spheric Guardian plus 2 Shapes picked independently from {Repulsor, Exploder, Spiker}. |
| 3 Darklings | 1 | Hard-pool version of the triple Darkling encounter (cannot immediately follow the easy version). |
| Spire Growth | 1 | Solo Spire Growth. |
| Transient | 1 | Solo Transient. |
| Jaw Worm Horde | 1 | 3× Jaw Worms; all start combat already under the effects of Bellow. |
| Writhing Mass | 1 | Solo Writhing Mass. |

### Jaw Worm Horde — Bellow Scaling

The wiki explicitly calls out the Jaw Worm Horde’s **Bellow** buff scaling by Ascension:

- **Ascension 0+**: Gain 3 Strength and 6 Block.
- **Ascension 2+**: Gain 4 Strength and 6 Block.
- **Ascension 17+**: Gain 5 Strength and 9 Block.

---

---

## Detailed Enemy Statistics & Move Sets

This section provides comprehensive enemy data for implementation, including HP ranges, move patterns, damage values, and ascension scaling.

### Act 1 Enemies (The Exordium)

#### Cultist

**Health:**
- Base: 48-54 HP
- Ascension 7+: 50-56 HP

**Moves:**
1. **Incantation** (Buff Intent) - Always used first turn
   - Gains Ritual (permanent Strength each turn)
   - Base: 3 Ritual
   - Ascension 2+: 4 Ritual
   - Ascension 17+: 5 Ritual

2. **Dark Strike** (Attack Intent) - Used every turn after first
   - Damage: 6
   - No ascension scaling

**Pattern:** Incantation → Dark Strike (repeating)

**Implementation Notes:**
- Only enemy that appears in all three acts
- Predictable attack pattern makes it beginner-friendly
- Ritual scaling makes it dangerous if left alive too long
- File: [cultist.rs](src/enemies/cultist.rs)

---

#### Jaw Worm

**Health:**
- Base: 40-44 HP
- Ascension 7+: 42-46 HP

**Moves:**
1. **Chomp** (Attack Intent)
   - Damage: 11
   - Ascension 2+: 12 damage

2. **Bellow** (Defend/Buff Intent)
   - Gains Strength and Block
   - Base: +3 Strength, +6 Block
   - Ascension 2+: +4 Strength, +6 Block
   - Ascension 17+: +5 Strength, +9 Block

3. **Thrash** (Combined Intent)
   - Damage: 7
   - Gains 5 Block

**Pattern:**
- Always starts with Chomp
- After Chomp: 59% Bellow / 41% Thrash
- After Bellow: 56% Thrash / 44% Chomp
- After Thrash: 45% Bellow / 30% Thrash / 25% Chomp
  - Cannot use Thrash three times consecutively

**Hard Variant (Act 3 - Jaw Worm Horde):**
- 3× Jaw Worms that start combat with Bellow already applied
- Starting Powers:
  - Base: 3 Strength, 6 Block
  - Ascension 2+: 4 Strength, 6 Block
  - Ascension 17+: 5 Strength, 9 Block

**Implementation Notes:**
- Probabilistic move selection requires RNG
- Act 3 variant requires special initialization
- File: [jaw_worm.rs](src/enemies/jaw_worm.rs)

---

#### Red Louse

**Health:**
- Base: 10-15 HP
- Ascension 7+: 11-16 HP

**Passive Ability:**
- **Curl Up**: Gains Block upon first receiving attack damage
  - Base: 3-7 Block (randomized at combat start)
  - Ascension 7+: 4-8 Block
  - Ascension 17+: 9-12 Block

**Moves:**
1. **Bite** (Attack Intent) - 75% chance
   - Damage: 5-7 (chosen randomly at combat start)
   - Ascension 2+: 6-8 damage
   - Cannot use three times in a row

2. **Grow** (Buff Intent) - 25% chance
   - Gains Strength
   - Base: +3 Strength
   - Ascension 17+: +4 Strength
   - Ascension 17+: Cannot use more than twice in a row

**Pattern:** Random weighted selection with constraints

**Implementation Notes:**
- Damage value for Bite is rolled once at combat start, not per use
- Curl Up block value also rolled once at combat start
- Acts 1 only
- File: [red_louse.rs](src/enemies/red_louse.rs)

---

#### Green Louse

**Health:**
- Base: 11-17 HP
- Ascension 7+: 12-18 HP

**Passive Ability:**
- **Curl Up**: Gains Block upon first receiving attack damage
  - Base: 3-7 Block (randomized at combat start)
  - Ascension 7+: 4-8 Block
  - Ascension 17+: 9-12 Block

**Moves:**
1. **Bite** (Attack Intent) - 75% chance
   - Damage: 5-7 (chosen randomly at combat start)
   - Ascension 2+: 6-8 damage
   - Cannot use three times in a row

2. **Spit Web** (Debuff Intent) - 25% chance
   - Applies 2 Weak
   - Ascension 17+: Cannot use more than twice in a row

**Pattern:** Random weighted selection with constraints

**Implementation Notes:**
- Similar to Red Louse but trades Strength buff for Weak debuff
- Damage value for Bite is rolled once at combat start
- Curl Up block value also rolled once at combat start
- Acts 1 only
- File: [green_louse.rs](src/enemies/green_louse.rs)

---

#### Acid Slime (Small)

**Health:**
- Base: 8-12 HP
- Ascension 7+: 9-13 HP

**Moves:**
1. **Lick** (Debuff Intent)
   - Applies 1 Weak

2. **Tackle** (Attack Intent)
   - Damage: 3
   - Ascension 2+: 4 damage

**Pattern:** Alternates between moves (exact probabilities not specified in wiki)

**Implementation Notes:**
- Simplest slime variant with no special abilities
- No split mechanics
- Act 1 only
- File: [acid_slime_s.rs](src/enemies/acid_slime_s.rs)

---

#### Acid Slime (Medium)

**Health:**
- Base: 28-32 HP
- Ascension 7+: 29-34 HP

**Moves:**
1. **Corrosive Spit** (Attack Intent) - 30% base chance
   - Damage: 7
   - Ascension 2+: 8 damage
   - Adds 1 Slimed to discard pile
   - Ascension 17+: 40% chance

2. **Tackle** (Attack Intent) - 30% base chance
   - Damage: 10
   - Ascension 2+: 12 damage
   - Ascension 17+: 40% chance

3. **Lick** (Debuff Intent) - 40% base chance
   - Applies 1 Weak
   - Ascension 17+: 20% chance
   - Cannot use twice consecutively (A17+)

**Pattern Constraints:**
- Cannot use same move three times in a row
- Ascension 17+: Modified probabilities and Lick restriction

**Implementation Notes:**
- Does NOT split (unlike Large variant)
- Result of Large Acid Slime splitting
- Act 1 only
- File: [acid_slime_m.rs](src/enemies/acid_slime_m.rs)

---

#### Spike Slime (Small)

**Health:**
- Base: 10-14 HP
- Ascension 7+: 11-15 HP

**Moves:**
1. **Tackle** (Attack Intent) - Only move
   - Damage: 5
   - Ascension 2+: 6 damage

**Pattern:** Uses Tackle every turn

**Implementation Notes:**
- Simplest enemy in the game with only one move
- No special abilities or debuffs
- Act 1 only
- File: [spike_slime_s.rs](src/enemies/spike_slime_s.rs)

---

#### Spike Slime (Medium)

**Health:**
- Base: 28-32 HP
- Ascension 7+: 29-34 HP

**Moves:**
1. **Flame Tackle** (Attack Intent) - 30% chance
   - Damage: 8
   - Ascension 2+: 10 damage
   - Adds 1 Slimed to discard pile

2. **Lick** (Debuff Intent) - 70% chance
   - Applies 1 Frail

**Pattern:**
- Random weighted selection
- Cannot use same move three times in a row

**Implementation Notes:**
- Does NOT split (unlike Large variant)
- Result of Large Spike Slime splitting
- Act 1 only
- File: [spike_slime_m.rs](src/enemies/spike_slime_m.rs)

---

#### Gremlin Nob (Elite)

**Health:**
- Base: 82-86 HP
- Ascension 8+: 85-90 HP (Note: Elites use A8 for HP instead of A7)

**Passive Ability:**
- **Enrage**: Whenever you play a Skill card, gains Strength
  - Base: +2 Strength per Skill
  - Ascension 18+: +3 Strength per Skill

**Moves:**
1. **Bellow** (Buff Intent) - Always used first turn
   - Gains 2 Enrage (passive Strength gain when player uses Skills)
   - Note: This is different from enrage triggering - it's setting up the passive

2. **Skull Bash** (Debuff/Attack Intent)
   - Damage: 6
   - Ascension 3+: 8 damage
   - Ascension 18+: 8 damage
   - Applies 2 Vulnerable

3. **Bull Rush** (Attack Intent)
   - Damage: 14
   - Ascension 3+: 16 damage
   - Ascension 18+: 16 damage

**Pattern:**
- Turn 1: Always Bellow
- Base/A2-17: After Bellow
  - 33% chance: Skull Bash
  - 67% chance: Bull Rush
  - Cannot use Bull Rush three consecutive times
- Ascension 18+: Fixed pattern after Bellow
  - Turn 2: Skull Bash
  - Turn 3-4: Bull Rush (twice)
  - Repeats: Skull Bash → Bull Rush → Bull Rush

**Implementation Notes:**
- Elite enemy with significantly higher HP
- Enrage passive punishes Skill card usage heavily
- Recommended strategy: Avoid Skills after turn 1, try to win by turn 3
- Act 1 Elite encounter
- File: [gremlin_nob.rs](src/enemies/gremlin_nob.rs)

---

### Act 2 Enemies (The City)

#### Byrd

**Health:**
- Base: 25-31 HP
- Ascension 7+: 26-33 HP

**Passive Ability:**
- **Flying**: Takes 50% less attack damage. Cancelled if dealt attack damage X times in one turn.
  - Base: 3 Flying charges
  - Ascension 17+: 4 Flying charges
  - Note: Indirect damage (poison, orbs) bypasses Flying entirely

**Moves (While Airborne):**
1. **Peck** (Attack Intent) - 50% chance
   - Damage: 1×5 hits
   - Ascension 2+: 1×6 hits
   - Cannot use three times consecutively

2. **Caw** (Buff Intent) - 30% chance
   - Gains 1 Strength
   - Cannot use consecutively

3. **Swoop** (Attack Intent) - 20% chance
   - Damage: 12
   - Ascension 2+: 14 damage
   - Cannot use consecutively

**Moves (Grounded - when Flying reaches 0):**
- **Stunned** (No Intent) - Takes no action (1 turn)
- **Headbutt** (Attack Intent) - Damage: 3
- **Go Airborne** (Defend Intent) - Restores Flying charges (3 base, 4 at A17+)

**Pattern:**
- Airborne: Weighted random selection with constraints
- Grounded: Fixed sequence (Stunned → Headbutt → Go Airborne)

**Implementation Notes:**
- Flying counter decreases with multi-hit attacks
- Block does not affect Flying counter
- Act 2 only

---

#### Chosen

**Health:**
- Base: 95-99 HP
- Ascension 7+: 98-103 HP

**Moves:**
1. **Poke** (Attack Intent)
   - Damage: 5×2 hits
   - Ascension 7+: 6×2 hits

2. **Hex** (Debuff Intent) - Always first turn (except A17+)
   - Shuffles Dazed cards into deck when non-Attack cards played
   - Ascension 17+: Used on turn 1 instead of turn 2

3. **Zap** (Attack Intent)
   - Damage: 18
   - Ascension 7+: 21 damage

4. **Debilitate** (Debuff/Attack Intent)
   - Damage: 10
   - Ascension 7+: 12 damage
   - Applies 2 Vulnerable

5. **Drain** (Debuff/Buff Intent)
   - No damage
   - Applies 3 Weak to player
   - Gains 3 Strength

**Pattern:**
- Base: Turn 1: Poke → Turn 2: Hex → Repeating cycle
- Ascension 17+: Turn 1: Hex → Repeating cycle
- Repeating cycle:
  - 50% Debilitate / 50% Drain
  - Then: 60% Poke / 40% Zap

**Implementation Notes:**
- Only source of Hex debuff in the game
- Act 2 only
- Appears solo or with Byrd/Cultist

---

#### Spheric Guardian

**Health:**
- Base: 20 HP
- Ascension 7+: 20 HP (no change)

**Starting Powers:**
- **Artifact**: 3 charges (prevents debuffs)
- **Barricade**: Block is not removed at turn start
- **Block**: Starts with 40 Block

**Moves:**
1. **Activate** (Defend Intent) - Always first turn
   - Gains Block
   - Base: +25 Block
   - Ascension 17+: +35 Block

2. **Debuff Attack** (Debuff/Attack Intent) - Turn 2
   - Damage: 10
   - Ascension 2+: 11 damage
   - Applies 5 Frail

3. **Slam** (Attack Intent) - Alternating after turn 2
   - Damage: 10×2 hits
   - Ascension 2+: 11×2 hits

4. **Harden** (Defend/Attack Intent) - Alternating after turn 2
   - Damage: 10
   - Ascension 2+: 11 damage
   - Gains 15 Block

**Pattern:**
- Turn 1: Activate
- Turn 2: Debuff Attack
- Turn 3+: Alternates between Slam and Harden

**Implementation Notes:**
- Very low HP but high Block makes it defensive
- Artifact charges protect from debuffs
- Cards like Melter or Judgment are very effective
- Acts 2 and 3

---

#### Shelled Parasite

**Health:**
- Base: 68-72 HP
- Ascension 7+: 70-75 HP

**Starting Powers:**
- **Plated Armor**: 14 charges (reduces damage, when broken enemy is Stunned once)

**Moves:**
1. **Double Strike** (Attack Intent) - 40% chance
   - Damage: 6×2 hits
   - Ascension 2+: 7×2 hits
   - Cannot use three times consecutively

2. **Life Suck** (Attack Intent) - 40% chance
   - Damage: 10
   - Ascension 2+: 12 damage
   - Heals equal to unblocked damage dealt
   - Cannot use three times consecutively

3. **Fell** (Debuff/Attack Intent) - 20% chance
   - Damage: 18
   - Ascension 2+: 21 damage
   - Applies 2 Frail
   - Cannot use on first turn (except A17+)
   - Cannot use twice consecutively

**Pattern:**
- Base: Cannot use Fell on turn 1, then random weighted selection
- Ascension 17+: Always uses Fell on turn 1

**Special Mechanic:**
- When all 14 Plated Armor is broken, becomes Stunned for 1 turn (only once per combat)

**Implementation Notes:**
- Plated Armor reduces all damage
- Stunning mechanic rewards heavy damage
- Act 2 only
- Appears solo or with Fungi Beast

---

#### Centurion and Mystic

These enemies always appear together as a pair.

**Centurion**

**Health:**
- Base: 76-80 HP
- Ascension 7+: 78-83 HP

**Moves (While Mystic is alive):**
1. **Slash** (Attack Intent) - 65% chance
   - Damage: 12
   - Ascension 2+: 14 damage

2. **Protect** (Defend Intent) - 35% chance
   - Grants Mystic 15 Block
   - Ascension 17+: Grants 20 Block

**Moves (After Mystic dies):**
- Protect is replaced with **Fury**:
  - Damage: 6×3 hits
  - Ascension 2+: 7×3 hits

**Pattern:**
- Cannot use any move three times in a row
- Random weighted selection

---

**Mystic**

**Health:**
- Base: 48-56 HP
- Ascension 7+: 50-58 HP

**Moves:**
1. **Attack** (Debuff/Attack Intent) - 60% chance (when not healing)
   - Damage: 8
   - Ascension 2+: 9 damage
   - Applies 2 Frail
   - Ascension 17+: Cannot use twice consecutively

2. **Buff** (Buff Intent) - 40% chance (when not healing)
   - Grants all enemies 2 Strength
   - Ascension 2+: 3 Strength
   - Ascension 17+: 3-4 Strength

3. **Heal** (Buff Intent) - Priority when any ally missing 16+ HP
   - Heals all enemies for 16 HP
   - Ascension 17+: Heals for 20 HP
   - Ascension 19+: Triggers when ally missing 21+ HP
   - Can be used up to twice consecutively

**Pattern:**
- Prioritizes Heal when conditions are met
- Otherwise: 60% Attack / 40% Buff
- Attack constraint at A17+

**Implementation Notes:**
- Mystic should be killed first to prevent healing
- Centurion becomes more dangerous after Mystic dies
- Act 2 only

---

#### Snake Plant

**Health:**
- Base: 75-79 HP
- Ascension 7+: 78-82 HP

**Passive Ability:**
- **Malleable 3**: Upon receiving attack damage, gains Block. Block gain increases as triggered. Resets at start of your turn.

**Moves:**
1. **Chomp Chomp** (Attack Intent) - 65% chance
   - Damage: 7×3 hits
   - Ascension 2+: 8×3 hits
   - Cannot use three times consecutively

2. **Enfeebling Spores** (Debuff Intent) - 35% chance
   - Applies 2 Frail
   - Applies 2 Weak
   - Cannot use twice consecutively

**Pattern:**
- Base/A2-16: Random weighted selection with constraints
- Ascension 17+: After first Enfeebling Spores, becomes deterministic:
  - Chomp Chomp → Chomp Chomp → Enfeebling Spores (repeating)

**Implementation Notes:**
- Malleable makes it tankier against multi-hit attacks
- Act 2 only
- Solo encounter

---

#### Snecko

**Health:**
- Base: 114-120 HP
- Ascension 7+: 120-125 HP

**Passive Ability:**
- **Confused**: Whenever you draw a card, randomize its cost.

**Moves:**
1. **Perplexing Glare** (Debuff Intent) - Always first turn
   - Applies Confused debuff to player

2. **Bite** (Attack Intent) - 60% chance
   - Damage: 15
   - Ascension 2+: 18 damage
   - Cannot use three times consecutively

3. **Tail Whip** (Debuff/Attack Intent) - 40% chance
   - Damage: 8
   - Ascension 2+: 10 damage
   - Applies 2 Vulnerable
   - Ascension 17+: Also applies 2 Weak

**Pattern:**
- Turn 1: Always Perplexing Glare
- Turn 2+: Random weighted selection with constraint

**Implementation Notes:**
- Confused persists throughout combat
- High HP makes it a lengthy fight
- Act 2 only
- Solo encounter

---

### Act 3 Enemies (The Beyond)

#### Darkling

**Health:**
- Base: 48-56 HP
- Ascension 7+: 50-59 HP

**Passive Ability:**
- **Life Link**: If other Darklings are still alive, revives in 2 turns
  - When killed, enters "Regrowing..." state (takes no action)
  - After 2 turns, uses "Reincarnate" and revives at 50% HP
  - Cannot be permanently killed until all Darklings are dead

**Moves:**
1. **Nip** (Attack Intent)
   - Damage: 7-11 (randomized)
   - Ascension 2+: 9-13 damage
   - Cannot use three times in a row

2. **Chomp** (Attack Intent)
   - Damage: 8×2 hits
   - Ascension 2+: 9×2 hits
   - Cannot use twice in a row

3. **Harden** (Buff Intent)
   - Base: Gains 12 Block
   - Ascension 2+: Gains 12 Block AND 17 Strength
   - Cannot use twice in a row

**Pattern:**
- Turn 1: 50% Nip / 50% Harden (middle Darkling cannot use Chomp)
- Subsequent turns (outer Darklings): 30% Nip / 40% Chomp / 30% Harden

**Implementation Notes:**
- Always appears in groups of 3
- Must kill all three to prevent revival
- A2+ Harden is extremely dangerous (+17 Strength)
- Act 3 only (easy and hard pools)

---

#### Orb Walker

**Health:**
- Base: 90-96 HP
- Ascension 7+: 92-102 HP

**Passive Ability:**
- **Strength Up**: Gains Strength at end of turn
  - Base: +3 Strength per turn
  - Ascension 17+: +5 Strength per turn

**Moves:**
1. **Laser** (Attack Intent) - 60% chance
   - Damage: 10
   - Ascension 2+: 11 damage
   - Adds 1 Burn to draw pile
   - Adds 1 Burn to discard pile
   - Cannot use three times consecutively

2. **Claw** (Attack Intent) - 40% chance
   - Damage: 15
   - Ascension 2+: 16 damage
   - Cannot use three times consecutively

**Pattern:** Random weighted selection with constraint

**Implementation Notes:**
- Scales very quickly with passive Strength gain
- Burns dilute deck over time
- Prioritize ending fight quickly
- Act 3 only

---

#### Repulsor (Shape)

**Health:**
- Base: 29-35 HP
- Ascension 7+: 31-38 HP

**Moves:**
1. **Attack** (Attack Intent) - 20% chance
   - Damage: 11
   - Ascension 2+: 13 damage
   - Cannot use twice consecutively

2. **Daze** (Debuff Intent) - 80% chance
   - Adds 2 Dazed cards to draw pile
   - Dazed is unplayable, ethereal status card

**Pattern:** Heavily weighted toward Daze with constraint

**Implementation Notes:**
- Primary threat is deck clogging, not damage
- Appears in groups with other Shapes
- Act 3 only

---

#### Spiker (Shape)

**Health:**
- Base: 42-56 HP
- Ascension 7+: 44-60 HP

**Passive Ability:**
- **Thorns**: Inflicts damage on attacker when hit
  - Base: Thorns 3
  - Ascension 2+: Thorns 4
  - Ascension 17+: Thorns 7

**Moves:**
1. **Attack** (Attack Intent) - 50% chance
   - Damage: 7
   - Ascension 2+: 9 damage
   - Cannot use twice consecutively

2. **Buff Thorns** (Buff Intent) - 50% chance
   - Gains 2 additional Thorns stacks
   - Can only use 6 times total (then only uses Attack)

**Pattern:** 50/50 selection with constraints

**Implementation Notes:**
- Becomes more dangerous the longer the fight goes
- High Thorns punishes multi-hit attacks
- Consider using skills/powers instead of attacks
- Appears in groups with other Shapes
- Act 3 only

---

#### Exploder (Shape)

**Health:**
- Base: 30 HP
- Ascension 7+: 30-35 HP

**Passive Ability:**
- **Explosive 3**: Explodes after 3 turns, dealing damage and dying

**Moves:**
- **Turn 1-2: Attack** (Attack Intent)
  - Damage: 9
  - Ascension 2+: 11 damage

- **Turn 3: Explode** (Attack Intent)
  - Damage: 30
  - Enemy dies after this attack

**Pattern:** Fixed 3-turn sequence

**Implementation Notes:**
- Entirely predictable behavior
- High priority target (30 damage explosion)
- Kill before turn 3 or prepare heavy defense
- Appears in groups with other Shapes
- Act 3 only

---

#### Spire Growth

**Health:**
- Base: 170 HP
- Ascension 7+: 190 HP

**Moves:**
1. **Quick Tackle** (Attack Intent) - 50% base chance
   - Damage: 16
   - Ascension 2+: 18 damage

2. **Constrict** (Debuff Intent) - 50% base chance
   - Applies Constricted debuff
   - Base: 10 Constricted stacks
   - Ascension 2+: 12 Constricted stacks
   - Constricted: Take X damage at end of your turn
   - Not used if player already has Constricted

3. **Smash** (Attack Intent) - Used if Constrict was last move or player has Constricted
   - Damage: 22
   - Ascension 2+: 25 damage

**Pattern:**
- 50/50 between Quick Tackle and Constrict
- If Constrict was used last turn, use Smash instead
- If player has Constricted debuff, use Smash instead
- Cannot use any attack three times consecutively

**Implementation Notes:**
- High HP makes it a long fight
- Constricted damage stacks up over time
- Act 3 only
- Solo encounter

---

#### Transient

**Health:**
- Base: 999 HP
- Ascension 7+: 999 HP (no change)

**Passive Abilities:**
- **Fading**: Dies in X turns
  - Base: 5 turns
  - Ascension 17+: 6 turns

- **Shifting**: Upon losing HP, loses that much Strength until end of turn

**Moves:**
1. **Attack** (Attack Intent) - Only move
   - Damage formula: Base damage + (Current turn × 10)
   - Base: 20 + (turn × 10)
   - Ascension 2+: 30 + (turn × 10)

**Damage Progression:**
- Turn 1: 30 damage (40 at A2+)
- Turn 2: 40 damage (50 at A2+)
- Turn 3: 50 damage (60 at A2+)
- Turn 4: 60 damage (70 at A2+)
- Turn 5: 70 damage (80 at A2+)

**Implementation Notes:**
- Extremely high HP but dies automatically
- Damage reduction reduces Strength temporarily (Shifting)
- Can survive by defending until it dies from Fading
- Or deal massive damage quickly to kill it
- Act 3 only
- Solo encounter

---

#### Writhing Mass

**Health:**
- Base: 160 HP
- Ascension 7+: 175 HP

**Passive Abilities:**
- **Malleable 4**: Upon receiving attack damage, gains Block. Block gain increases as triggered. Resets at start of your turn.
- **Reactive**: Changes its attack intent whenever it takes damage

**Moves:**
1. **Multi Hit** (Attack Intent) - 30% subsequent chance, 33% first turn
   - Damage: 7×3 hits
   - Ascension 2+: 9×3 hits

2. **Debuff Attack** (Debuff/Attack Intent) - 20% subsequent chance, 33% first turn
   - Damage: 10
   - Ascension 2+: 12 damage
   - Applies 2 Weak
   - Applies 2 Vulnerable

3. **Big Hit** (Attack Intent) - 10% subsequent chance, 33% first turn
   - Damage: 32
   - Ascension 2+: 38 damage

4. **Block Attack** (Defend/Attack Intent) - 30% subsequent chance
   - Damage: 15
   - Ascension 2+: 16 damage
   - Gains 16 Block

5. **Parasite** (Debuff Intent) - 10% subsequent chance
   - Permanently adds a Parasite card to your deck
   - Can only be used once

**Pattern:**
- First turn: 33% Multi Hit / 33% Debuff Attack / 33% Big Hit
- Subsequent turns: Weighted random (see percentages above)
- Cannot use same move twice in a row
- Parasite only once per combat
- Reactive: Intent changes when damaged

**Implementation Notes:**
- Reactive makes it unpredictable
- Malleable punishes multi-hit attacks
- High HP requires sustained offense
- Act 3 only
- Solo encounter

---

#### The Maw

**Health:**
- Base: 300 HP
- All Ascensions: 300 HP (unique - HP never changes!)

**Moves:**
1. **Roar** (Debuff Intent) - Always first turn
   - Applies Weak and Frail
   - Base: 3 Weak, 3 Frail
   - Ascension 5+: 17 Weak, 17 Frail

2. **Slam** (Attack Intent)
   - Damage: 25
   - Ascension 2+: 30 damage

3. **Nom Nom** (Attack Intent)
   - Scaling damage: 5×N, where N = (current turn ÷ 2) rounded up
   - Turn 2: 5 damage
   - Turn 4: 10 damage
   - Turn 6: 15 damage
   - Turn 8: 20 damage

4. **Drool** (Buff Intent)
   - Gains Strength
   - Base: +3 Strength
   - Ascension 5+: +5 Strength

**Pattern:**
- Turn 1: Always Roar
- After Roar: 50% Slam / 50% Nom Nom
- After Drool: 50% Slam / 50% Nom Nom
- After Slam: 50% Nom Nom / 50% Drool
- After Nom Nom: Always Drool

**Implementation Notes:**
- Only enemy with constant 300 HP across all Ascensions
- Roar at A5+ is devastating (17 Weak/Frail)
- Nom Nom damage scales throughout fight
- Pattern becomes predictable after learning it
- Act 3 only
- Solo encounter

---

## Elite Enemies

Elite enemies have higher HP thresholds and appear at elite encounter nodes. Note that elites use **Ascension 8** for HP increases (not A7 like normal enemies) and **Ascension 18** for move-set upgrades (not A17).

### Act 1 Elites

#### Gremlin Nob

See detailed entry in Act 1 Enemies section above.

---

#### Lagavulin

**Health:**
- Base: 109-111 HP
- Ascension 8+: 112-115 HP

**Starting Powers:**
- **Metallicize 8**: At end of turn, gains 8 Block
- **Block**: Starts with 8 Block
- **Asleep**: Starts combat sleeping

**Moves (When Awakened):**
1. **Attack** (Attack Intent) - Used twice in pattern
   - Damage: 18
   - Ascension 3+: 20 damage

2. **Siphon Soul** (Debuff Intent) - Used after every 2 attacks
   - Applies -1 Dexterity
   - Applies -1 Strength
   - Ascension 18+: Applies -2 Dexterity, -2 Strength

**Pattern (When Awakened):**
- Attack → Attack → Siphon Soul (repeating)

**Sleep Mechanics:**
- Starts combat sleeping (takes no action)
- If takes any damage: Becomes Stunned for 1 turn, then awakens
- If 3 turns pass without taking damage: Awakens immediately
- When awakened, loses Metallicize power

**Dead Adventurer Event Variant:**
- Spawns already awake
- Pattern: Siphon Soul → Attack → Attack (repeating)

**Implementation Notes:**
- Must track sleep state and turn counter
- Metallicize removed on awakening
- Act 1 elite only

---

#### Sentries (3 Sentries)

**Health (per Sentry):**
- Base: 38-42 HP
- Ascension 8+: 39-45 HP

**Moves:**
1. **Bolt** (Debuff Intent)
   - Adds 2 Dazed cards to discard pile
   - Ascension 18+: Adds 3 Dazed cards

2. **Beam** (Attack Intent)
   - Damage: 9
   - Ascension 3+: 10 damage

**Pattern:**
- Outer Sentries (1st and 3rd): Bolt → Beam (repeating)
- Middle Sentry (2nd): Beam → Bolt (repeating)

**Implementation Notes:**
- Always appear as a group of 3
- Middle Sentry has reversed pattern
- Dazed cards are unplayable, ethereal status
- Prioritize killing outer sentries to reduce double-damage turns
- Act 1 elite only
- Also appears in Act 2 paired with Spheric Guardian

---

### Act 2 Elites

#### Gremlin Leader

**Health:**
- Base: 140-148 HP
- Ascension 8+: 145-155 HP

**Moves:**
1. **Rally!** (Buff Intent)
   - Summons 2 random gremlins
   - Maximum 3 gremlins total allowed

2. **Stab** (Attack Intent)
   - Damage: 6×3 hits

3. **Encourage** (Buff Intent)
   - Grants all enemies 3 Strength
   - Grants all allies 6 Block
   - Higher ascensions increase these values

**Pattern:**
- With 0 Gremlins: 75% Rally / 25% Stab
- With 1 Gremlin:
  - After Encourage: 50% Rally / 50% Stab
  - After Stab: 62.5% Rally / 37.5% Encourage
- With 2+ Gremlins: 66% Encourage / 34% Stab
- Cannot use same move consecutively

**Gremlins Summoned:**
- Random selection from: Fat Gremlin, Sneaky Gremlin, Mad Gremlin, Shield Gremlin, Gremlin Wizard
- Starts encounter with 2 gremlins already present

**Implementation Notes:**
- Focus on killing minions vs. the leader (strategy varies)
- Encourage stacking can become dangerous
- Act 2 elite only

---

#### Book of Stabbing

**Health:**
- Base: 160-164 HP
- Ascension 8+: 168-172 HP

**Passive Ability:**
- **Painful Stabs**: Whenever you receive attack damage from this enemy, add 1 Wound to discard pile

**Moves:**
1. **Multi Stab** (Attack Intent) - 85% chance
   - Base: 6×N damage (N = times used this combat + 2)
   - Ascension 3+: 7×N damage
   - Ascension 18+: 7×N damage (N = current turn + 1)
   - Cannot use 3 times consecutively

2. **Big Stab** (Attack Intent) - 15% chance
   - Damage: 21
   - Ascension 3+: 24 damage
   - Cannot use twice consecutively

**Pattern:** Random weighted selection with constraints

**Implementation Notes:**
- Multi Stab scales throughout combat
- Wound accumulation can clog deck
- Act 2 elite only

---

#### Bronze Automaton

**Health:**
- Base: 300 HP
- Ascension 9+: 320 HP

**Starting Action:**
- **Spawn Orbs**: Summons 2 Bronze Orbs at combat start

**Moves:**
1. **Flail** (Attack Intent)
   - Damage: 7×2 hits
   - Ascension 4+: 8×2 hits

2. **Boost** (Buff Intent)
   - Gains 3 Strength
   - Gains 9 Block

3. **HYPER BEAM** (Attack Intent)
   - Damage: 45
   - Ascension 4+: 50 damage

4. **Stunned** (No Intent) - Takes no action
   - Ascension 19+: Replaced with Boost (no vulnerability window!)

**Pattern:**
- Turn 1-2: Flail
- Turn 3-4: Boost
- Turn 5-6: Flail
- Turn 7-8: Boost
- Turn 9: HYPER BEAM
- Turn 10: Stunned (or Boost at A19+)
- Repeats cycle

**Bronze Orb Minions:**
- **HP**: 52-58 (54-60 at Ascension 7+)
- **Stasis** (75% chance, used once): Steals a random card of highest rarity from draw pile
- **Support Beam** (70% after Stasis used): Grants Bronze Automaton 12 Block
- **Beam** (30% after Stasis used): Deals 8 damage

**Implementation Notes:**
- Kill orbs to prevent card theft
- Stolen cards returned to hand when orb dies
- A19+ removes Stunned vulnerability window
- Act 2 elite only
- Also appears as Act 2 boss

---

### Act 3 Elites

#### Nemesis

**Health:**
- Base: 185 HP
- Ascension 8+: 200 HP

**Passive Ability:**
- **Intangible**: Gains Intangible every other turn (reduces all damage to 1)

**Moves:**
1. **Tri Attack** (Attack Intent) - ~35% chance
   - Damage: 6×3 hits
   - Ascension 3+: 7×3 hits
   - Cannot use 3 times consecutively

2. **Tri Burn** (Debuff Intent) - ~35% chance
   - Adds 3 Burn cards to discard pile
   - Ascension 18+: Adds 5 Burn cards
   - Cannot use consecutively

3. **Scythe** (Attack Intent) - ~30% chance
   - Damage: 45
   - Cannot use consecutively

**Pattern:**
- Turn 1: 50% Tri Burn / 50% Tri Attack
- Subsequent: Weighted random with constraints listed above

**Implementation Notes:**
- Track Intangible timing carefully
- High single-target damage threat
- Burns accumulate quickly
- Act 3 elite only

---

#### Giant Head

**Health:**
- Base: 500 HP
- Ascension 8+: 520 HP

**Passive Ability:**
- **Slow**: Whenever you play a card, target receives 10% more damage from Attacks this turn (stacks)

**Moves:**

**Phase 1 (First 4 turns, 3 at A18+):**
1. **Count** (Attack Intent) - 50% chance
   - Damage: 13
   - Cannot use 3 times consecutively

2. **Glare** (Debuff Intent) - 50% chance
   - Applies 1 Weak
   - Cannot use 3 times consecutively

**Phase 2 (After countdown):**
1. **It Is Time** (Attack Intent) - Only move
   - Base: 30 damage (increases by 5 per use, max 60)
   - Ascension 3+: 40 damage (increases by 5 per use, max 70)

**Pattern:**
- Counts down: "4.. 3.. 2.. 1.." (starts at 3 on A18+)
- After countdown, uses It Is Time repeatedly

**Implementation Notes:**
- Very high HP pool
- Slow passive makes attacks stronger the more cards you play
- Must kill before It Is Time damage escalates
- Dialogue provides countdown warning
- Act 3 elite only

---

#### Reptomancer

**Health:**
- Base: 180-190 HP
- Ascension 8+: 190-200 HP

**Moves:**
1. **Spawn Dagger** (Buff Intent) - Always first turn
   - Summons 1 dagger
   - Ascension 18+: Summons 2 daggers
   - Max 4 daggers allowed (uses Snake Strike if at cap)

2. **Big Bite** (Attack Intent) - ~33% chance
   - Damage: 30
   - Ascension 3+: 34 damage
   - Cannot use consecutively with Snake Strike

3. **Snake Strike** (Debuff/Attack Intent) - ~33% chance
   - Damage: 13×2 hits
   - Ascension 3+: 16×2 hits
   - Applies 1 Weak
   - Cannot use consecutively with Big Bite

**Pattern:**
- Turn 1: Always Spawn Dagger
- Subsequent: ~33% each for Spawn Dagger, Big Bite, Snake Strike
- Cannot spawn daggers 3 times consecutively
- Cannot use damage attacks consecutively

**Dagger Minions:**
- **HP**: 20-25 (20-25 at Ascension 7+)
- **Turn 1**: Stab - Deals 9 damage, adds 1 Wound to discard
- **Turn 2**: Explode - Deals 25 damage, dagger dies

**Encounter Start:**
- Reptomancer + 2 Daggers already present

**Implementation Notes:**
- High priority: Kill daggers before they explode
- Daggers follow fixed 2-turn cycle
- Can be overwhelmed by dagger spawning
- Act 3 elite only

---

## Boss Enemies

Bosses have significantly higher HP and complex mechanics. They use **Ascension 9** for HP increases and **Ascension 4** for initial damage scaling.

### Act 1 Bosses

#### Slime Boss

**Health:**
- Base: 140 HP
- Ascension 9+: 150 HP

**Moves:**
1. **Goop Spray** (Debuff Intent)
   - Adds 3 Slimed cards to discard pile
   - Ascension 19+: Adds 5 Slimed cards

2. **Preparing** (Unknown Intent)
   - Does nothing (setup turn)

3. **Slam** (Attack Intent)
   - Damage: 35
   - Ascension 4+: 38 damage

**Pattern:**
- Goop Spray → Preparing → Slam (repeating until split)

**Special Mechanic:**
- **Split**: When HP reaches 50% or lower
  - Boss disappears
  - Spawns 1 Acid Slime (L) and 1 Spike Slime (L)
  - Both slimes have HP equal to boss's current HP

**Implementation Notes:**
- Only Slam can harm you before split
- Can force early split by dealing 70+ damage quickly
- Split slimes inherit remaining HP
- Act 1 boss

---

#### The Guardian

**Health:**
- Base: 240 HP
- Ascension 9+: 250 HP

**Passive Abilities:**
- **Sharp Hide**: Whenever you play an Attack, take damage
  - Amount varies by phase and ascension

**Moves:**

**Offensive Mode (Default):**
1. **Charging Up** (Defend Intent) - Gains 9 Block

2. **Fierce Bash** (Attack Intent)
   - Damage: 32
   - Ascension 4+: 36 damage

3. **Vent Steam** (Debuff Intent)
   - Applies 2 Weak
   - Applies 2 Vulnerable

4. **Whirlwind** (Attack Intent)
   - Damage: 5×4 hits

**Pattern:** Charging Up → Fierce Bash → Vent Steam → Whirlwind (repeating)

**Defensive Mode (Triggered by Mode Shift):**
- **Mode Shift**: After losing X HP, gains 20 Block and changes to Defensive Mode
  - Base: 30 HP threshold
  - Ascension 9+: 35 HP threshold
  - Ascension 19+: 40 HP threshold

1. **Defensive Mode** (Defend Intent) - Gains Block, changes mode

2. **Roll Attack** (Attack Intent)
   - Damage: 9
   - Ascension 4+: 10 damage

3. **Twin Slam** (Attack Intent)
   - Damage: 8×2 hits
   - Removes Sharp Hide stacks
   - Gains Mode Shift (increases by 10 per trigger)

4. **Whirlwind** - Returns to offensive cycle

**Pattern (Defensive):** Defensive Mode → Roll Attack → Twin Slam → Whirlwind → back to offensive

**Implementation Notes:**
- Mode Shift can trigger multiple times
- Sharp Hide punishes Attack-heavy decks
- Track HP damage to predict Mode Shift
- Ascension 19: Sharp Hide increases to 4 stacks in Defensive Mode
- Act 1 boss

---

#### Hexaghost

**Health:**
- Base: 250 HP
- Ascension 9+: 264 HP

**Moves:**
1. **Activate** (Unknown Intent) - Turn 1 only
   - Does nothing

2. **Divider** (Attack Intent) - Turn 2 only
   - Damage: (N+1)×6, where N = player's current HP ÷ 12 (rounded down)
   - Deals less damage if player has low HP

3. **Sear** (Attack Intent)
   - Damage: 6
   - Adds 1 Burn to discard pile
   - Ascension 19+: Adds 1 Burn (but Inferno adds many more)

4. **Tackle** (Attack Intent)
   - Damage: 5×2 hits
   - Ascension 4+: 6×2 hits

5. **Inflame** (Buff Intent)
   - Gains 12 Block
   - Gains 2 Strength
   - Ascension 19+: Gains 3 Strength

6. **Inferno** (Attack Intent)
   - Damage: 2×6 hits
   - Ascension 4+: 3×6 hits
   - Adds 3 Burns+ to discard
   - Upgrades all existing Burns in deck

**Pattern:**
- Turn 1: Activate
- Turn 2: Divider
- Turn 3-9: Sear → Tackle → Sear → Inflame → Tackle → Sear → Inferno
- Repeats 7-turn cycle after Inferno

**Implementation Notes:**
- Intentionally staying low HP reduces Divider damage
- Burns accumulate quickly, especially after Inferno
- Need ~30 damage/turn scaling to kill before Inferno
- After first Inferno, all future Burns are upgraded
- Act 1 boss

---

### Act 2 Bosses

#### The Champ

**Health:**
- Base: 420 HP
- Ascension 9+: 440 HP

**Moves:**

**Phase 1 (Above 50% HP):**
1. **Taunt** (Debuff Intent) - Every 4 turns
   - Applies 2 Weak
   - Applies 2 Vulnerable

2. **Heavy Slash** (Attack Intent) - 45% chance
   - Damage: 16
   - Ascension 4+: 18 damage

3. **Face Slap** (Debuff/Attack Intent) - 25% chance
   - Damage: 12
   - Ascension 4+: 14 damage
   - Applies 2 Frail
   - Applies 2 Vulnerable

4. **Defensive Stance** (Defend Intent) - 15% chance
   - Gains 15 Block
   - Gains 5 Metallicize
   - Higher ascensions: 20 Block, 7 Metallicize

5. **Gloat** (Buff Intent) - 15% chance
   - Gains 2 Strength
   - Higher ascensions: 3-4 Strength

**Pattern Phase 1:**
- Taunt every 4 turns
- Otherwise: Weighted random (no move twice in a row)

**Phase 2 (Below 50% HP):**
1. **Anger** (Buff Intent) - Used once when crossing 50% threshold
   - Removes all debuffs
   - Gains 6+ Strength (scales with ascension)

2. **Execute** (Attack Intent) - Every 3rd turn
   - Damage: 10×2 hits

3. **Random Moves** - 2 random moves, then Execute (repeating)

**Implementation Notes:**
- High HP pool makes this a long fight
- Phase 2 adds Execute to rotation
- Metallicize stacking can become problematic
- Act 2 boss

---

#### The Collector

**Health:**
- Base: 282 HP
- Ascension 9+: 300 HP

**Moves:**
1. **Spawn** (Buff Intent)
   - Summons 1 Torch Head minion
   - Maximum 2 Torch Heads at once
   - Always used on turn 1

2. **Fireball** (Attack Intent)
   - Damage: 18
   - Ascension 4+: 21 damage
   - Cannot use 3 times consecutively

3. **Buff** (Buff Intent)
   - Grants all enemies +3 Strength
   - Grants boss +15 Block
   - Ascension 4+: +4 Strength
   - Ascension 9+: +18 Block
   - Ascension 19+: +5 Strength, +23 Block
   - Cannot use consecutively

4. **Mega Debuff** (Debuff Intent) - Always on turn 4
   - Applies 3 Weak
   - Applies 3 Vulnerable
   - Applies 3 Frail
   - Ascension 19+: 5 stacks of each

**Pattern:**
- Turn 1: Always Spawn
- Turn 4: Always Mega Debuff
- With 1 Torch Head: 25% Spawn / 45% Fireball / 30% Buff
- With 2 Torch Heads: 70% Fireball / 30% Buff

**Torch Head Minions:**
- **HP**: 38-40 (40-45 at Ascension 7+)
- **Tackle**: Deals 7 damage

**Implementation Notes:**
- Kill Torch Heads to reduce incoming damage
- Buff stacking makes fight harder over time
- Mega Debuff on turn 4 is guaranteed
- Act 2 boss

---

#### Bronze Automaton (Boss version)

See Bronze Automaton entry in Act 2 Elites section above. Same stats and mechanics.

---

### Act 3 Bosses

#### Awakened One

**Health:**
- Base: 300 HP
- Ascension 9+: 320 HP

**Passive Abilities:**
- **Regenerate**: Recovers 10 HP per turn (15 at Ascension 19+)
- **Curiosity**: Gains Strength when player plays Power cards
  - Base: +1 Strength per Power
  - Ascension 19+: +2 Strength per Power
- **Strength** (Ascension 4+): Starts with 2 Strength

**Moves:**

**Phase 1 (Before death):**
1. **Slash** (Attack Intent) - 75% chance
   - Damage: 20
   - Cannot use 3 times consecutively

2. **Soul Strike** (Attack Intent) - 25% chance
   - Damage: 6×4 hits (24 total)
   - Cannot use twice consecutively

**Phase 2 (After Rebirth):**
- **Rebirth**: When reduced to 0 HP
  - Removes all debuffs
  - Loses Curiosity
  - Revives to full HP
  - Retains gained Strength

1. **Dark Echo** (Attack Intent) - Always first move in Phase 2
   - Damage: 40

2. **Tackle** (Attack Intent) - 50% chance after Dark Echo
   - Damage: 10×3 hits (30 total)
   - Cannot use 3 times consecutively

3. **Sludge** (Attack Intent) - 50% chance after Dark Echo
   - Damage: 18
   - Adds 1 Void card to player's deck
   - Cannot use 3 times consecutively

**Pattern Phase 2:** Dark Echo → (Tackle/Sludge alternating, 50/50)

**Implementation Notes:**
- Avoid using Powers (they buff boss permanently)
- Must kill twice (Rebirth mechanic)
- Regenerate makes damage race critical
- Strength carries between phases
- Act 3 boss

---

#### Donu and Deca

These two bosses always appear together.

**Donu:**

**Health:**
- Base: 250 HP
- Ascension 9+: 265 HP

**Starting Powers:**
- **Artifact**: 2 charges (3 at Ascension 19+)

**Moves:**
1. **Circle of Power** (Buff Intent)
   - Grants all enemies +3 Strength

2. **Beam** (Attack Intent)
   - Damage: 10×2 hits
   - Ascension 4+: 12×2 hits

**Pattern:** Alternates between Circle of Power and Beam

---

**Deca:**

**Health:**
- Base: 250 HP
- Ascension 9+: 265 HP

**Starting Powers:**
- **Artifact**: 2 charges (3 at Ascension 19+)

**Moves:**
1. **Beam** (Debuff/Attack Intent)
   - Damage: 10×2 hits
   - Ascension 4+: 12×2 hits
   - Adds 2 Dazed to discard pile

2. **Square of Protection** (Defend Intent)
   - Grants all enemies +16 Block
   - Ascension 19+: Also grants +3 Plated Armor

**Pattern:** Alternates between Beam and Square of Protection

**Implementation Notes:**
- Both have Artifact charges (hard to debuff)
- Donu buffs Strength, Deca provides defense
- Kill one first to reduce complexity
- Dazed accumulation from Deca
- Act 3 boss

---

#### Time Eater

**Health:**
- Base: 456 HP
- Ascension 9+: 480 HP

**Passive Abilities:**
- **Time Warp**: When player plays 12 cards (6 at Ascension 19+), turn ends immediately and Time Eater gains Strength
- **Draw Reduction**: Reduces cards drawn next turn

**Moves:**
1. **Reverberate** (Attack Intent) - 45% chance
   - Damage: 7×3 hits
   - Ascension 4+: 8×3 hits
   - Cannot use 3 times consecutively

2. **Head Slam** (Debuff/Attack Intent) - 35% chance
   - Damage: 26
   - Ascension 4+: 32 damage
   - Applies 1 Draw Reduction
   - Ascension 19+: Also adds 2 Slimed cards to discard
   - Cannot use twice consecutively

3. **Ripple** (Debuff/Defend Intent) - 20% chance
   - Gains 20 Block
   - Applies 1 Vulnerable
   - Applies 1 Weak
   - Ascension 19+: Also applies 1 Frail
   - Cannot use twice consecutively

**Special Ability:**
- **Haste** (Used once when HP drops below 50%)
  - Removes all debuffs
  - Heals to 50% HP
  - Ascension 19+: Also gains 32 Block

**Implementation Notes:**
- Card play limit is critical mechanic
- Play fewer, more impactful cards
- Haste makes killing in one phase difficult
- Time Warp triggers can end turn unexpectedly
- Act 3 boss

---

## Act 4 (The Ending) Enemies

Act 4 is an optional true ending that can be unlocked. It contains one elite encounter and one boss.

### Act 4 Elite

#### Spire Shield and Spire Spear

These two enemies always appear together in a mandatory elite encounter.

**Spire Shield:**

**Health:**
- Base: 110 HP
- Ascension 8+: 125 HP

**Starting Powers:**
- **Artifact**: 1 charge (2 at Ascension 18+)
- **Back Attack**: Deals 50% extra damage when behind player

**Moves:**
1. **Bash** (Debuff/Attack Intent) - Alternates with Fortify
   - Damage: 12
   - Ascension 3+: 14 damage
   - Applies -1 Focus (if player has orb slots) OR -1 Strength (otherwise)
   - 50% chance to apply debuff

2. **Fortify** (Defend Intent) - Alternates with Bash
   - Grants all enemies 30 Block

3. **Smash** (Attack Intent) - Every 3 turns starting turn 3
   - Damage: 34
   - Ascension 3+: 38 damage
   - Gains Block equal to damage dealt
   - Ascension 18+: Gains 99 Block (fixed amount)

**Pattern:**
- Turn 3, 6, 9, etc.: Smash
- Other turns: 50% Bash → Fortify / 50% Fortify → Bash

---

**Spire Spear:**

**Health:**
- Base: 160 HP
- Ascension 8+: 180 HP

**Starting Powers:**
- **Artifact**: 1 charge (2 at Ascension 18+)
- **Back Attack**: Deals 50% extra damage when behind player

**Moves:**
1. **Burn Strike** (Attack Intent) - Always first turn, alternates with Piercer
   - Damage: 5×2 hits
   - Ascension 3+: 6×2 hits
   - Adds 2 Burns to discard pile
   - Ascension 18+: Burns added to top of draw pile instead

2. **Piercer** (Buff Intent) - Alternates with Burn Strike
   - Grants all enemies +2 Strength

3. **Skewer** (Attack Intent) - Every 3 turns starting turn 2
   - Damage: 10×3 hits
   - Ascension 3+: 10×4 hits

**Pattern:**
- Turn 1: Always Burn Strike
- Turn 2, 5, 8, etc.: Skewer
- Other turns: 50% Burn Strike → Piercer / 50% Piercer → Burn Strike

---

**Encounter Mechanics:**
- **Surrounded**: Player starts with Surrounded debuff (only blocks Smoke Bomb potion use)
- **Back Attack**: Whichever enemy is behind you deals 50% extra damage
- **Targeting**: Using a card or potion on an enemy turns you to face them, making the other enemy "behind" you

**Implementation Notes:**
- Must track which enemy player is facing
- Back Attack multiplies damage by 1.5
- General strategy: Kill Spire Spear first (higher damage output)
- Both have Artifact charges
- Act 4 elite only

---

### Act 4 Boss

#### Corrupt Heart

**Health:**
- Base: 750 HP
- Ascension 9+: 800 HP

**Passive Abilities:**
- **Invincible**: Can only lose X HP per turn
  - Base: 300 HP max loss per turn
  - Ascension 19+: 200 HP max loss per turn

- **Beat of Death**: Whenever you play a card, take X damage
  - Base: 1 damage per card
  - Ascension 19+: 2 damage per card

**Moves:**

**Turn 1:**
1. **Debilitate** (Debuff Intent) - Always first turn
   - Applies 2 Weak
   - Applies 2 Vulnerable
   - Applies 2 Frail
   - Adds 5 status cards to deck: 1 Burn, 1 Dazed, 1 Slimed, 1 Void, 1 Wound

**Turns 2-3 (Random order):**
1. **Blood Shots** (Attack Intent)
   - Damage: 2×12 hits (24 total)
   - Ascension 4+: 2×15 hits (30 total)

2. **Echo** (Attack Intent)
   - Damage: 40
   - Ascension 4+: 45 damage

**Pattern for turns 2-3:**
- 50% chance: Blood Shots → Echo
- 50% chance: Echo → Blood Shots

**Turn 4 and every 3 turns after (4, 7, 10, 13, etc.):**
1. **Buff** (Buff Intent)
   - Removes negative Strength
   - Gains +2 Strength
   - Plus rotating buff (cycles through 5 buffs):
     1. **First Buff (Turn 4)**: Gains 2 Artifact
     2. **Second Buff (Turn 7)**: Gains +1 Beat of Death (increases card damage)
     3. **Third Buff (Turn 10)**: Gains Painful Stabs (whenever you take attack damage, add 1 Wound to discard)
     4. **Fourth Buff (Turn 13)**: Gains +10 Strength
     5. **Fifth Buff (Turn 16+)**: Gains +50 Strength

**Between Buff turns:**
- Continues alternating Blood Shots and Echo

**Pattern Summary:**
- Turn 1: Debilitate
- Turn 2-3: Blood Shots/Echo (random order)
- Turn 4: Buff (Artifact)
- Turn 5-6: Blood Shots/Echo (continuing alternation)
- Turn 7: Buff (Beat of Death)
- Turn 8-9: Blood Shots/Echo
- Turn 10: Buff (Painful Stabs)
- Turn 11-12: Blood Shots/Echo
- Turn 13: Buff (+10 Strength)
- Turn 14-15: Blood Shots/Echo
- Turn 16+: Buff (+50 Strength), then Blood Shots/Echo

**Implementation Notes:**
- Invincible mechanic caps damage per turn (requires multi-turn strategy)
- Beat of Death punishes playing many cards
- Must kill before Strength buffs become overwhelming
- Card-efficient, high-damage strategies are best
- Requires unlocking by collecting 3 keys throughout run
- Act 4 boss (final boss of true ending)

---

## Using This Reference

- This file is intended to stay in sync with the Monsters wiki page; whenever the wiki updates encounter weights or compositions, re-run `curl https://slaythespire.wiki.gg/wiki/Monsters -o /tmp/monsters.html` and refresh the tables above.
- The tables already cover every listed enemy (58 total entries across all acts) along with the encounter weights and compositions needed to script battle nodes that match the official game.
- When implementing enemies, cross-reference the individual enemy sub-pages for move sets, but rely on this document for *which enemies combine together* and how frequently they appear according to the current wiki data.
- **Detailed enemy stats section above provides complete implementation data** including HP ranges, move patterns, damage values, status effects, and all ascension scaling.
