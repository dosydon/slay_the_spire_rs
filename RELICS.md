# Relics Implementation Status

This document tracks the implementation status of all relics in the Slay the Spire Rust implementation.

## Summary

- ✅ **3 relics implemented** (1 Starter + 2 Common)
- ❌ **174 relics not yet implemented**
- 🎯 **Implementation Progress: ~1.7%** of all relics
- 📊 **Total Relics: 177** (4 Starter + 36 Common + 36 Uncommon + 33 Rare + 30 Boss + 20 Shop + 18 Event)

## Current Relic System Architecture

The relic system uses an event-driven architecture with two types of event listeners:

### Game Event Listeners
- Trigger on game-wide events (combat victory, map navigation, etc.)
- Used for relics that affect progression between battles
- Example: **Burning Blood** - Heals 6 HP after combat victory

### Battle Event Listeners
- Trigger on battle-specific events (turn start, end of turn, card played, etc.)
- Used for relics that affect individual combats
- Examples: **Anchor**, **Blood Vial** - Provide combat bonuses

## Relic Implementation Tables

### Starter Relics (4 Total)

| Relic Name | Character | Implemented | File Location | Effect |
|------------|-----------|-------------|---------------|--------|
| ✅ **Burning Blood** | Ironclad | Yes | `src/relics/burning_blood.rs` | At the end of combat, heal 6 HP |
| ❌ Cracked Core | Defect | No | - | At the start of each combat, Channel 1 Lightning |
| ❌ Pure Water | Watcher | No | - | At the start of each combat, add 1 Miracle to your hand |
| ❌ Ring of the Snake | Silent | No | - | At the start of each combat, draw 2 additional cards |

**Implementation Notes:**
- Burning Blood uses GameEventListener for post-combat healing
- Other starter relics require character-specific systems (Orbs for Defect, Stances for Watcher, card generation for Silent/Watcher)

---

### Common Relics (36 Total)

| Relic Name | Character | Implemented | File Location | Effect |
|------------|-----------|-------------|---------------|--------|
| ❌ Akabeko | All | No | - | Your first Attack each combat deals 8 additional damage |
| ✅ **Anchor** | All | Yes | `src/relics/anchor.rs` | Start each combat with 10 Block |
| ❌ Ancient Tea Set | All | No | - | Whenever you enter a Rest Site, start next combat with 2 extra Energy |
| ❌ Art of War | All | No | - | If you do not play Attacks during your turn, gain 1 Energy next turn |
| ❌ Bag of Marbles | All | No | - | At the start of each combat, apply 1 Vulnerable to ALL enemies |
| ❌ Bag of Preparation | All | No | - | At the start of each combat, draw 2 additional cards |
| ✅ **Blood Vial** | All | Yes | `src/relics/blood_vial.rs` | At the start of each combat, heal 2 HP |
| ❌ Bronze Scales | All | No | - | Whenever you take damage, deal 3 damage back (Thorns) |
| ❌ Centennial Puzzle | All | No | - | The first time you lose HP each combat, draw 3 cards |
| ❌ Ceramic Fish | All | No | - | Whenever you add a card to your deck, gain 9 Gold |
| ❌ Damaru | Watcher | No | - | At the start of your turn, gain 1 Mantra |
| ❌ Data Disk | Defect | No | - | Start each combat with 1 Focus |
| ❌ Dream Catcher | All | No | - | Whenever you rest, you may add a card to your deck |
| ❌ Happy Flower | All | No | - | Every 3 turns, gain 1 Energy |
| ❌ Juzu Bracelet | All | No | - | Regular combat encounters in ? rooms are no longer possible |
| ❌ Lantern | All | No | - | Gain 1 Energy on the first turn of each combat |
| ❌ Maw Bank | All | No | - | Whenever you climb a floor, gain 12 Gold. No longer works when you spend Gold at shop |
| ❌ Meal Ticket | All | No | - | Whenever you enter a shop, heal 15 HP |
| ❌ Nunchaku | All | No | - | Every time you play 10 Attacks, gain 1 Energy |
| ❌ Oddly Smooth Stone | All | No | - | At the start of each combat, gain 1 Dexterity |
| ❌ Omamori | All | No | - | Negate the next 2 Curses you obtain |
| ❌ Orichalcum | All | No | - | If you end your turn without Block, gain 6 Block |
| ❌ Pen Nib | All | No | - | Every 10th Attack you play deals double damage |
| ❌ Potion Belt | All | No | - | Upon pickup, gain 2 Potion slots |
| ❌ Preserved Insect | All | No | - | Enemies in Elite rooms have 25% less HP |
| ❌ Red Skull | Ironclad | No | - | While your HP is at or below 50%, you have 3 additional Strength |
| ❌ Regal Pillow | All | No | - | Heal an additional 15 HP when you Rest |
| ❌ Smiling Mask | All | No | - | The Merchant's card removal service now always costs 50 Gold |
| ❌ Snecko Skull | Silent | No | - | Whenever you apply Poison, apply 1 additional Poison |
| ❌ Strawberry | All | No | - | Raise your Max HP by 7 |
| ❌ The Boot | All | No | - | Whenever you deal 4 or less unblocked Attack damage, increase it to 5 |
| ❌ Tiny Chest | All | No | - | Every 4th ? room is a Treasure room |
| ❌ Toy Ornithopter | All | No | - | Whenever you use a Potion, heal 5 HP |
| ❌ Vajra | All | No | - | At the start of each combat, gain 1 Strength |
| ❌ War Paint | All | No | - | Upon pickup, Upgrade 2 random Skills |
| ❌ Whetstone | All | No | - | Upon pickup, Upgrade 2 random Attacks |

**Implementation Priority:**
- **High**: Vajra, Bag of Preparation, Lantern (simple combat start effects)
- **Medium**: Orichalcum, Akabeko, Nunchaku (combat mechanics tracking)
- **Low**: Ceramic Fish, Maw Bank, Dream Catcher (map/progression integration)

---

### Uncommon Relics (36 Total)

| Relic Name | Character | Implemented | File Location | Effect |
|------------|-----------|-------------|---------------|--------|
| ❌ Blue Candle | All | No | - | Curse cards can be played. When played, lose 1 HP and Exhaust |
| ❌ Bottled Flame | All | No | - | Upon pickup, choose an Attack. Start each combat with it in hand |
| ❌ Bottled Lightning | All | No | - | Upon pickup, choose a Skill. Start each combat with it in hand |
| ❌ Bottled Tornado | All | No | - | Upon pickup, choose a Power. Start each combat with it in hand |
| ❌ Darkstone Periapt | All | No | - | Whenever you obtain a Curse, increase your Max HP by 6 |
| ❌ Duality | Watcher | No | - | Whenever you play an Attack, gain 1 temporary Dexterity |
| ❌ Eternal Feather | All | No | - | For every 5 cards in your deck, heal 3 HP when you Rest |
| ❌ Frozen Egg | All | No | - | Whenever you add a Power to your deck, Upgrade it |
| ❌ Gold-Plated Cables | Defect | No | - | Your rightmost Orb triggers its passive ability an additional time |
| ❌ Gremlin Horn | All | No | - | Whenever an enemy dies, gain 1 Energy and draw 1 card |
| ❌ Horn Cleat | All | No | - | At the start of your 2nd turn, gain 14 Block |
| ❌ Ink Bottle | All | No | - | Every time you play 10 cards, draw 1 card |
| ❌ Kunai | All | No | - | Every time you play 3 Attacks in a single turn, gain 1 Dexterity |
| ❌ Letter Opener | All | No | - | Every time you play 3 Skills in a single turn, deal 5 damage to ALL enemies |
| ❌ Matryoshka | All | No | - | The next 2 chests you open contain 2 Relics (excludes boss chests) |
| ❌ Meat on the Bone | All | No | - | If your HP is at or below 50% at the end of combat, heal 12 HP |
| ❌ Mercury Hourglass | All | No | - | At the start of your turn, deal 3 damage to ALL enemies |
| ❌ Molten Egg | All | No | - | Whenever you add an Attack to your deck, Upgrade it |
| ❌ Mummified Hand | All | No | - | Whenever you play a Power, a random card in hand costs 0 this turn |
| ❌ Ninja Scroll | Silent | No | - | Start each combat with 3 Shivs in hand |
| ❌ Ornamental Fan | All | No | - | Every time you play 3 Attacks in a single turn, gain 4 Block |
| ❌ Pantograph | All | No | - | At the start of boss combats, heal 25 HP |
| ❌ Paper Krane | Silent | No | - | Enemies with Weak deal 40% less damage instead of 25% |
| ❌ Paper Phrog | Ironclad | No | - | Enemies with Vulnerable take 75% more damage instead of 50% |
| ❌ Pear | All | No | - | Raise your Max HP by 10 |
| ❌ Question Card | All | No | - | Future card reward screens have 1 additional card to choose from |
| ❌ Self-Forming Clay | Ironclad | No | - | Whenever you lose HP in combat, gain 3 Block next turn |
| ❌ Shuriken | All | No | - | Every time you play 3 Attacks in a single turn, gain 1 Strength |
| ❌ Singing Bowl | All | No | - | When adding cards to your deck, you may gain +2 Max HP instead |
| ❌ Strike Dummy | All | No | - | Cards containing 'Strike' deal 3 additional damage |
| ❌ Sundial | All | No | - | Every 3 times you shuffle your deck, gain 2 Energy |
| ❌ Symbiotic Virus | Defect | No | - | At the start of each combat, Channel 1 Dark |
| ❌ Teardrop Locket | Watcher | No | - | Start each combat in Calm |
| ❌ The Courier | All | No | - | Merchants no longer run out of cards, relics, or potions. Prices are reduced by 20% |
| ❌ Toxic Egg | All | No | - | Whenever you add a Skill to your deck, Upgrade it |
| ❌ White Beast Statue | All | No | - | Potions always drop after combat |

**Implementation Priority:**
- **High**: Mercury Hourglass, Horn Cleat, Gremlin Horn (combat event triggers)
- **Medium**: Kunai, Shuriken, Ornamental Fan (card counter mechanics)
- **Low**: Bottled relics, Egg relics (card selection and upgrade systems)

---

### Rare Relics (33 Total)

| Relic Name | Character | Implemented | File Location | Effect |
|------------|-----------|-------------|---------------|--------|
| ❌ Bird-Faced Urn | All | No | - | Whenever you play a Power, heal 2 HP |
| ❌ Calipers | All | No | - | At the start of your turn, lose 15 Block rather than all Block |
| ❌ Captain's Wheel | All | No | - | At the start of your 3rd turn, gain 18 Block |
| ❌ Champion Belt | Ironclad | No | - | Whenever you apply Vulnerable, also apply 1 Weak |
| ❌ Charon's Ashes | Ironclad | No | - | Whenever you Exhaust a card, deal 3 damage to ALL enemies |
| ❌ Cloak Clasp | Watcher | No | - | At the end of your turn, gain 1 Block for each card in hand |
| ❌ Dead Branch | All | No | - | Whenever you Exhaust a card, add a random card to your hand |
| ❌ Du-Vu Doll | All | No | - | For each Curse in your deck, start each combat with 1 Strength |
| ❌ Emotion Chip | Defect | No | - | If you lost HP during the previous turn, trigger all Orb passives |
| ❌ Fossilized Helix | All | No | - | Prevent the first time you would lose HP in combat |
| ❌ Gambling Chip | All | No | - | At combat start, discard any number of cards, then draw that many |
| ❌ Ginger | All | No | - | You can no longer become Weakened |
| ❌ Girya | All | No | - | Gain 1 Strength (can be used at Rest Sites up to 3 times) |
| ❌ Golden Eye | Watcher | No | - | Whenever you Scry, Scry 2 additional cards |
| ❌ Ice Cream | All | No | - | Energy no longer resets at the end of turn |
| ❌ Incense Burner | All | No | - | Every 6 turns, gain 1 Intangible |
| ❌ Lizard Tail | All | No | - | When you die, revive with 50% HP (once per combat) |
| ❌ Magic Flower | Ironclad | No | - | Healing is 50% more effective during combat |
| ❌ Mango | All | No | - | Raise your Max HP by 14 |
| ❌ Old Coin | All | No | - | Gain 300 Gold |
| ❌ Peace Pipe | All | No | - | Can remove a card from deck at Rest Sites |
| ❌ Pocketwatch | All | No | - | Whenever you draw a Status or Curse, draw 1 additional card |
| ❌ Prayer Wheel | All | No | - | Normal enemy combats award an additional card reward |
| ❌ Shovel | All | No | - | Can dig at Rest Sites for 1 Relic |
| ❌ Stone Calendar | All | No | - | At the end of turn 7, gain 2 Energy |
| ❌ The Specimen | Silent | No | - | Whenever you defeat an enemy with Poison, add a card to your deck |
| ❌ Thread and Needle | All | No | - | At the start of combat, gain 4 Plated Armor |
| ❌ Tingsha | Silent | No | - | When you discard a card (outside of end of turn), deal 3 damage to a random enemy |
| ❌ Torii | All | No | - | Whenever you take Attack damage over 5, reduce it to 1 |
| ❌ Tough Bandages | Silent | No | - | When you discard a card, gain 3 Block |
| ❌ Tungsten Rod | All | No | - | Whenever you lose HP from a card or relic, lose 1 less |
| ❌ Turnip | All | No | - | Deal 2 more damage with Attacks for every card in discard pile (max 6) |
| ❌ Unceasing Top | All | No | - | Whenever you have no cards in hand during your turn, draw 1 card |
| ❌ Wing Boots | All | No | - | Can ignore paths when choosing next room (3 charges) |

**Implementation Priority:**
- **High**: Ice Cream, Calipers, Girya (unique mechanics with high impact)
- **Medium**: Dead Branch, Fossilized Helix, Incense Burner (complex triggers)
- **Low**: Peace Pipe, Shovel, Wing Boots (map/rest site integration)

---

### Boss Relics (30 Total)

| Relic Name | Character | Implemented | File Location | Effect | Drawback |
|------------|-----------|-------------|---------------|--------|----------|
| ❌ Astrolabe | All | No | - | Upon pickup, choose and Transform 3 cards, then Upgrade them | None |
| ❌ Black Blood | Ironclad | No | - | Replaces Burning Blood. At end of combat, heal 12 HP | None (upgrade) |
| ❌ Black Star | All | No | - | Elites drop 2 Relics when defeated | None |
| ❌ Busted Crown | All | No | - | Gain 1 Energy at start of turn | Card rewards have 2 fewer cards to choose from |
| ❌ Calling Bell | All | No | - | Upon pickup, obtain 3 Relics, 1 Curse, and Curse 3 random cards | Random Curse added |
| ❌ Coffee Dripper | All | No | - | Gain 1 Energy at start of turn | Cannot Rest to heal at Rest Sites |
| ❌ Cursed Key | All | No | - | Gain 1 Energy at start of turn | Obtain 1 Curse when opening non-boss chests |
| ❌ Ectoplasm | All | No | - | Gain 1 Energy at start of turn | Cannot gain Gold |
| ❌ Empty Cage | All | No | - | Upon pickup, remove 2 cards from deck | None |
| ❌ Frozen Core | Defect | No | - | Replaces Cracked Core. If you have an empty Orb slot, Channel 1 Frost at turn start | None (upgrade) |
| ❌ Fusion Hammer | All | No | - | Gain 1 Energy at start of turn | Can no longer Smith at Rest Sites |
| ❌ Holy Water | Watcher | No | - | Replaces Pure Water. At combat start, add 3 Miracles to hand | None (upgrade) |
| ❌ Hovering Kite | Silent | No | - | Replaces Ring of the Snake. At combat start, gain 1 Energy and draw 1 card | None (upgrade) |
| ❌ Inserter | Defect | No | - | Every turn, add a random Orb slot | None |
| ❌ Mark of Pain | Ironclad | No | - | Gain 1 Energy at start of turn | Obtain 2 Curses, gain 1 Wound when losing HP |
| ❌ Nuclear Battery | Defect | No | - | At start of each combat, Channel 1 Plasma | None |
| ❌ Pandora's Box | All | No | - | Transform all Strikes and Defends | Transformation is random |
| ❌ Philosopher's Stone | All | No | - | Gain 1 Energy at start of turn | All enemies start combat with 1 Strength |
| ❌ Ring of the Serpent | Silent | No | - | Replaces Ring of the Snake. At combat start, draw 1 card. At combat end, heal 2 HP | None (upgrade) |
| ❌ Runic Cube | Ironclad | No | - | Whenever you lose HP, draw 1 card | Can no longer draw at end of turn |
| ❌ Runic Dome | All | No | - | Gain 1 Energy at start of turn | Cannot see enemy Intents |
| ❌ Runic Pyramid | All | No | - | At end of turn, retain your hand | None |
| ❌ Sacred Bark | All | No | - | Potions have double effect | None |
| ❌ Slaver's Collar | All | No | - | In Elite combats, gain 1 Energy at start of turn | None |
| ❌ Snecko Eye | All | No | - | Draw 2 additional cards. Start each combat Confused | All cards cost 0-3 Energy randomly |
| ❌ Sozu | All | No | - | Gain 1 Energy at start of turn | No longer gain Potions |
| ❌ Tiny House | All | No | - | Gain 1 Potion, 50 Gold, 1 card, +5 Max HP, Upgrade 1 card | None |
| ❌ Velvet Choker | All | No | - | Gain 1 Energy at start of turn | Can only play 6 cards per turn |
| ❌ Violet Lotus | Watcher | No | - | Whenever you exit Calm, gain 1 Energy | None |
| ❌ Wrist Blade | Silent | No | - | Attacks that cost 0 deal 4 additional damage | None |

**Implementation Notes:**
- Most boss relics grant +1 Energy with trade-offs
- Character-specific boss relics typically upgrade starter relics
- Requires sophisticated drawback mechanics (Cursed Key, Mark of Pain, etc.)

**Implementation Priority:**
- **High**: Runic Pyramid, Empty Cage, Astrolabe (no drawbacks, simpler)
- **Medium**: Snecko Eye, Coffee Dripper, Cursed Key (drawback mechanics)
- **Low**: Runic Dome, Busted Crown, Velvet Choker (complex restrictions)

---

### Shop Relics (20 Total)

| Relic Name | Character | Implemented | File Location | Effect | Cost |
|------------|-----------|-------------|---------------|--------|------|
| ❌ Brimstone | Ironclad | No | - | At combat start, gain 2 Strength. At combat end, gain 1 Slimed | 300 Gold |
| ❌ Cauldron | All | No | - | Upon pickup, brew 5 random Potions | 250 Gold |
| ❌ Chemical X | All | No | - | X-cost cards cost 2 additional Energy when played | 250 Gold |
| ❌ Clockwork Souvenir | All | No | - | At combat start, gain 1 Artifact | 250 Gold |
| ❌ Dolly's Mirror | All | No | - | Upon pickup, duplicate a card in deck | 300 Gold |
| ❌ Frozen Eye | All | No | - | When viewing your Draw Pile, cards are now shown in order | 250 Gold |
| ❌ Hand Drill | All | No | - | Attacks that cost 2+ Energy deal 4 additional damage | 250 Gold |
| ❌ Lee's Waffle | All | No | - | Raise Max HP by 7, heal all HP | 250 Gold |
| ❌ Medical Kit | All | No | - | Status cards can be played. Playing one Exhausts it | 250 Gold |
| ❌ Melange | Watcher | No | - | Whenever you shuffle, Scry 3 | 250 Gold |
| ❌ Membership Card | All | No | - | 50% discount at Shop | 250 Gold |
| ❌ Orange Pellets | All | No | - | If you play an Attack, Skill, and Power in the same turn, remove debuffs | 250 Gold |
| ❌ Orrery | All | No | - | Choose 1 of 5 cards instead of 3 at card rewards | 300 Gold |
| ❌ Prismatic Shard | All | No | - | Combat rewards have a chance to include colorless/other class cards | 300 Gold |
| ❌ Runic Capacitor | Defect | No | - | Gain 3 Orb slots | 300 Gold |
| ❌ Sling of Courage | All | No | - | At combat start, gain 2 Strength. When you enter a ? room, lose 1 Strength | 250 Gold |
| ❌ Strange Spoon | All | No | - | Cards that Exhaust have a 50% chance to return to hand instead | 250 Gold |
| ❌ The Abacus | All | No | - | Whenever you shuffle, gain 6 Block | 250 Gold |
| ❌ Toolbox | All | No | - | At combat start, choose 1 of 3 random colorless cards to add to hand | 250 Gold |
| ❌ Twisted Funnel | Silent | No | - | At combat start, apply 4 Poison to ALL enemies | 300 Gold |

**Implementation Notes:**
- All shop relics cost 250-300 Gold
- Can only be obtained from Merchant
- Mix of combat effects and meta-progression bonuses

**Implementation Priority:**
- **High**: Lee's Waffle, Cauldron, Clockwork Souvenir (simple effects)
- **Medium**: Chemical X, Hand Drill, The Abacus (mechanical modifiers)
- **Low**: Prismatic Shard, Orrery, Dolly's Mirror (deck/reward system changes)

---

### Event Relics (18 Total)

| Relic Name | Implemented | File Location | Effect | Obtained From |
|------------|-------------|---------------|--------|---------------|
| ❌ Bloody Idol | No | - | Gain 5 Gold when entering battle | Golden Idol event |
| ❌ Cultist Headpiece | No | - | At start of combat, gain 1 Ritual | Cultist event |
| ❌ Enchiridion | No | - | At combat start, add a random Power to hand | ? event |
| ❌ Face of Cleric | No | - | At third Rest Site, heal all HP | Face Trader event |
| ❌ Golden Idol | No | - | Enemies drop 25% more Gold | Golden Idol event |
| ❌ Gremlin Visage | No | - | Start combat with 1 Weak | Gremlin event |
| ❌ Mark of the Bloom | No | - | Cannot heal except via Resting | Mind Bloom event |
| ❌ Mutagenic Strength | No | - | At combat start, gain 3 Strength. Lose 1 HP per turn | Mutagenic event |
| ❌ N'loth's Gift | No | - | Gain 3 Strength | N'loth event (positive) |
| ❌ N'loth's Hungry Face | No | - | Whenever you gain Max HP, lose 1 Max HP | N'loth event (negative) |
| ❌ Necronomicon | No | - | The first Attack you play twice each turn is played twice | Necronomicon event |
| ❌ Neow's Lament | No | - | Enemies in first 3 combats have 1 HP | Neow's Blessing |
| ❌ Nilry's Codex | No | - | Can duplicate 3 cards at Rest Sites | Nilry event |
| ❌ Odd Mushroom | No | - | Whenever you lose HP from a card, gain 1 Vulnerable | Shroom event |
| ❌ Red Mask | No | - | At combat start, apply 1 Weak to ALL enemies | Masked Bandits event |
| ❌ Spirit Poop | No | - | It's unpleasant. (Negative relic - no effect) | Spirit Poop event |
| ❌ Ssserpent Head | No | - | Whenever you enter a ? room, gain 50 Gold | Sssssss event |
| ❌ Warped Tongs | No | - | At combat start, Upgrade a random card in hand for combat | ? event |

**Implementation Notes:**
- Event relics have unique acquisition methods
- Some have negative effects (Mark of the Bloom, Spirit Poop)
- Neow's Lament is time-limited (3 combats only)

**Implementation Priority:**
- **High**: Necronomicon, Golden Idol, Enchiridion (impactful effects)
- **Medium**: Neow's Lament, Cultist Headpiece, Red Mask (combat bonuses)
- **Low**: Spirit Poop, N'loth's Hungry Face (negative/tracking relics)

---

## Special Relics (2 Total)

| Relic Name | Implemented | File Location | Effect |
|------------|-------------|---------------|--------|
| ❌ Circlet | No | - | Placeholder relic when you'd obtain a duplicate Boss/Shop relic |
| ❌ Red Circlet | No | - | Placeholder relic (second tier) |

**Implementation Notes:**
- Circlet appears when duplicate prevention triggers
- Acts as a marker, provides no mechanical benefit
- Red Circlet is rarer version

---

## Implementation Framework Features

### ✅ Completed Features
- Event-driven relic system with GameEventListener and BattleEventListener traits
- Game-wide effects (Burning Blood - post-combat healing)
- Combat effects (Anchor - starting block, Blood Vial - starting healing)
- Relic registration and factory pattern for creating event listeners
- Character-specific starter relics
- Relic storage in game and battle state

### ❌ Required Framework Features

#### High Priority (Many Relics Need These)
- ❌ **Turn start/end events** - Many relics trigger at specific turn timings
- ❌ **Energy manipulation** - +1 Energy relics, energy gain/loss
- ❌ **Card play counting** - Track cards played (Nunchaku, Ink Bottle, etc.)
- ❌ **HP loss/gain events** - Trigger on damage/healing
- ❌ **Card counter mechanics** - Count Attacks/Skills played per turn
- ❌ **Status effect manipulation** - Apply status at combat start
- ❌ **Max HP modification** - Increase/decrease Max HP
- ❌ **Draw mechanics** - Draw additional cards at specific times
- ❌ **Exhaust events** - Trigger when cards are exhausted
- ❌ **Potion system** - Potion slots, effects, acquisition

#### Medium Priority
- ❌ **Gold system** - Gain gold, spend gold tracking
- ❌ **Map system integration** - Rest sites, shops, events
- ❌ **Card selection UI** - Choose cards for Bottled relics
- ❌ **Deck manipulation** - Add/remove/transform cards
- ❌ **Upgrade mechanics** - Upgrade cards from relics
- ❌ **Curse system** - Add curses, negate curses
- ❌ **Block retention** - Calipers mechanic
- ❌ **Damage modification** - Increase/decrease damage dealt

#### Low Priority (Complex Systems)
- ❌ **Orb system** - Defect orb mechanics (Cracked Core, etc.)
- ❌ **Stance system** - Watcher stance mechanics (Teardrop Locket, etc.)
- ❌ **Scry mechanics** - Watcher card preview (Golden Eye, etc.)
- ❌ **Artifact system** - Negate debuffs
- ❌ **Intangible system** - Reduce all damage to 1
- ❌ **Mantra system** - Watcher Mantra generation
- ❌ **Confusion system** - Randomize card costs (Snecko Eye)
- ❌ **Revival mechanics** - Lizard Tail effect
- ❌ **Poison modification** - Enhance poison (Snecko Skull)

---

## Recommended Implementation Order

### Phase 1: Simple Combat Start Effects (10 relics)
1. **Vajra** - Gain 1 Strength at combat start
2. **Bag of Preparation** - Draw 2 cards at combat start
3. **Lantern** - Gain 1 Energy on turn 1
4. **Oddly Smooth Stone** - Gain 1 Dexterity at combat start
5. **Bag of Marbles** - Apply 1 Vulnerable to ALL at combat start
6. **Strawberry** - Raise Max HP by 7
7. **Pear** - Raise Max HP by 10
8. **Mango** - Raise Max HP by 14
9. **Old Coin** - Gain 300 Gold
10. **Empty Cage** - Remove 2 cards from deck on pickup

### Phase 2: Turn-Based Effects (8 relics)
1. **Mercury Hourglass** - Deal 3 damage to ALL at turn start
2. **Horn Cleat** - Gain 14 Block on turn 2
3. **Captain's Wheel** - Gain 18 Block on turn 3
4. **Happy Flower** - Every 3 turns, gain 1 Energy
5. **Orichalcum** - If no Block at turn end, gain 6 Block
6. **Stone Calendar** - At end of turn 7, gain 2 Energy
7. **Calipers** - Lose 15 Block at turn start instead of all
8. **Ice Cream** - Energy no longer resets at turn end

### Phase 3: Card Counter Mechanics (9 relics)
1. **Nunchaku** - Every 10 Attacks, gain 1 Energy
2. **Ink Bottle** - Every 10 cards, draw 1 card
3. **Pen Nib** - Every 10 Attacks, double damage
4. **Kunai** - Every 3 Attacks in turn, gain 1 Dexterity
5. **Shuriken** - Every 3 Attacks in turn, gain 1 Strength
6. **Ornamental Fan** - Every 3 Attacks in turn, gain 4 Block
7. **Letter Opener** - Every 3 Skills in turn, deal 5 damage to ALL
8. **Akabeko** - First Attack each combat deals +8 damage
9. **Strike Dummy** - Strikes deal +3 damage

### Phase 4: Event-Driven Relics (10 relics)
1. **Gremlin Horn** - When enemy dies, gain 1 Energy and draw 1 card
2. **Pantograph** - At boss combat start, heal 25 HP
3. **Meat on the Bone** - If ≤50% HP at combat end, heal 12 HP
4. **Bird-Faced Urn** - When playing Power, heal 2 HP
5. **Charon's Ashes** - When exhausting card, deal 3 damage to ALL
6. **Dead Branch** - When exhausting card, add random card to hand
7. **Mummified Hand** - When playing Power, random card costs 0 this turn
8. **Bronze Scales** - When taking damage, deal 3 damage back
9. **Fossilized Helix** - Prevent first HP loss in combat
10. **Sundial** - Every 3 shuffles, gain 2 Energy

### Phase 5: Boss Relics (Energy +1) (6 relics)
1. **Coffee Dripper** - +1 Energy, cannot Rest to heal
2. **Cursed Key** - +1 Energy, obtain Curse when opening chests
3. **Ectoplasm** - +1 Energy, cannot gain Gold
4. **Fusion Hammer** - +1 Energy, cannot Smith
5. **Busted Crown** - +1 Energy, -2 card choices
6. **Philosopher's Stone** - +1 Energy, enemies start with +1 Strength

---

## Recently Implemented Relics

### ✅ Burning Blood (Starter - Ironclad)
- **File**: `src/relics/burning_blood.rs`
- **Effect**: At the end of combat, heal 6 HP
- **Implementation**: GameEventListener for CombatVictory event
- **Test Coverage**: ✅ Complete

### ✅ Anchor (Common)
- **File**: `src/relics/anchor.rs`
- **Effect**: Start each combat with 10 Block
- **Implementation**: BattleEventListener for CombatStart event
- **Test Coverage**: ✅ Complete

### ✅ Blood Vial (Common)
- **File**: `src/relics/blood_vial.rs`
- **Effect**: At the start of each combat, heal 2 HP
- **Implementation**: BattleEventListener for CombatStart event
- **Test Coverage**: ✅ Complete

---

## References

- [Slay the Spire Wiki - Relics List](https://slaythespire.wiki.gg/wiki/Relics_List)
- [Fandom Wiki - Relics](https://slay-the-spire.fandom.com/wiki/Relics)
- [Relic Data Module](https://slay-the-spire.fandom.com/wiki/Module:Relics/data)
