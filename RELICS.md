# Relics Implementation Status

This document tracks the implementation status of all relics in the Slay the Spire Rust implementation.

## Summary

- ✅ **3 relics implemented** (1 Starter + 2 Shop)
- ❌ **200+ relics not yet implemented** (Common, Uncommon, Rare, Boss, Event, Blight, Special)
- 🎯 **Implementation Progress: ~1.5%** of all relics

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

## Implemented Relics

### Starter Relics
| Relic Name | Character | Implemented | File Location | Effect |
|------------|-----------|-------------|---------------|---------|
| ✅ **Burning Blood** | Ironclad | Yes | `src/relics/burning_blood.rs` | At the end of combat, heal 6 HP |
| ❌ Cracked Core | Defect | No | - | At the start of each combat, Channel 1 Lightning |
| ❌ Pure Water | Silent | No | - | At the start of each combat, gain 1 |

### Shop Relics
| Relic Name | Rarity | Implemented | File Location | Effect |
|------------|--------|-------------|---------------|---------|
| ✅ **Anchor** | Common | Yes | `src/relics/anchor.rs` | Start each combat with 10 Block |
| ✅ **Blood Vial** | Common | Yes | `src/relics/blood_vial.rs` | Heal 2 HP at the start of combat |
| ❌ Boot | Common | No | - | Enemies cannot have Block |

### Unimplemented Relics by Category

#### Common Relics (40+ missing)
- Bag of Marbles - Gain 1 Strength at the start of each combat
- Bronze Scales - Gain 1 Thorns at the start of each combat
- Cracked Core - Channel 1 Lightning at start of combat (Defect starter)
- Damaru - Start each combat with 1 Artifact
- Ginger - At the start of combat, heal 2 HP
- Hovering Kite - Gain 1 Dexterity at the start of each combat
- Holy Water - At the start of each combat, enemies lose 1 Strength
- Horn Cleat - Take 1 less damage from attacks
- Incense Burner - Start each combat with 1 Ritual
- Maw Bank - At the start of each combat, gain 2 Block
- Pocket Watch - At the start of combat, if you're at max HP, gain 1 extra energy
- Red Mask - Start each combat with 1 Strength
- Snecko Eye - All cards cost 0-3 energy
- Strike Dummy - Start each combat with 1 Strength

#### Uncommon Relics (60+ missing)
- Art of War - The first time you play an Attack each combat, gain 1 Strength
- Bag of Preparation - Draw 2 additional cards at the start of combat
- Busted Crown - Cards cost 1 less for the rest of combat
- Calipers - The first time you play a Skill each combat, gain 1 Dexterity
- Captains Wheel - Gain 1 Strength at the start of your turn
- Circlet - If you end your turn with cards in hand, gain 1 Block
- Coffee Dripper - Gain 1 Energy at the start of your turn
- Dream Catcher - Gain 1 Max HP at the start of each combat
- Frozen Egg - At the start of combat, Channel 1 Frost
- Gambling Chip - At the start of combat, gain 9 Block, lose 9 Block next turn
- Girya - Can upgrade cards for free at rest sites
- Golden Idol - When you pick up a relic, gain 2 Max HP
- Happy Flower - Gain 1 Max HP when a card is Exhausted
- Inserter - The first time a card is Exhausted each turn, gain 1 Block
- Lantern - Cards cost 1 less if they are not in your starting deck
- Lizard Tail - When you die, instead of dying, heal to 50% Max HP
- Letter Opener - At the start of combat, add 1 Peer into your hand
- Meat on the Bone - When you die, retain your Gold and 1 random relic
- Metronome - At the start of combat, randomize the cost of cards in your hand
- Molten Egg - At the start of combat, Channel 1 Fire
- Mutating Power - Whenever you play a Power, add a random Power to your discard pile
- Nickel and Dime - Gain 1 additional energy per turn
- Odd Mushroom - At the start of combat, gain 1 Strength and 1 Dexterity
- Ornamental Fan - At the start of each combat, enemies gain 1 Weak
- Papercutter - At the start of each combat, draw 2 additional cards
- Penny - At the start of each combat, gain 15 Gold
- Pocket Watch - At the start of combat, if you're at max HP, gain 1 additional energy
- Red Skull - When you end your turn with no cards in hand, gain 2 Strength
- Rolling Die - Whenever you play an Attack, gain 1 Dexterity
- Rune Cylinder - The first time you play a Skill each combat, gain 1 Strength
- Self-Forming Clay - At the start of combat, add 1 copy of a card in your hand to your discard pile
- Shovel - Open chests for free
- Slavers Collar - Start each combat with 1 Strength
- Symbiotic Virus - When you die, retain 1 random card from your hand
- Tungsten Rod - Gain 1 additional Max HP
- Unceasing Top - Whenever a Power is played from hand, gain 1 Block
- War Paint - Gain 1 Strength whenever you play an Attack
- Whip - Enemies have 1 less HP

#### Rare Relics (70+ missing)
- Abacus - Cards that cost 0 energy or less become Ethereal
- Blue Candle - Start each combat with 1 additional energy
- Calling Bell - At the start of combat, give each enemy 1 Vulnerable and 1 Weak
- Captain's Wheel - Gain 1 Strength at the start of your turn
- Centennial Puzzle - At the start of your turn, if there are exactly 10 cards in your discard pile, gain 3 energy
- Clockwork Souvenir - At the start of each combat, gain 1 Block and 1 Block each turn
- Coffee Dripper - Gain 1 energy at the start of your turn
- Cone - Whenever you shuffle your draw pile, deal 10 damage to ALL enemies
- Cursed Key - You cannot gain Max HP
- Dead Branch - Whenever you play a Skill, add a random Attack to your hand
- Du-Vu Doll - When you take unblocked damage, gain 1 Strength
- Ectoplasm - You cannot heal
- Eternal Feather - At the start of each combat, gain 1 Flight
- Fusion Hammer - Upgrading a card also exhausts it
- Golden Idol - When you pick up a relic, gain 2 Max HP
- Gremlin Horn - At the end of your turn, gain 1 Strength
- Happy Flower - Gain 1 Max HP when a card is Exhausted
- Hand Drill - Whenever you play a non-Starter card, gain 1 Block
- Holy Water - At the start of each combat, enemies lose 1 Strength
- Hovering Kite - Gain 1 Dexterity at the start of each combat
- Ice Cream - At the end of your turn, gain 3 Block
- Ink Bottle - Cards cost 1 less for this turn
- Juzu Bracelet - Gain 1 additional Max HP whenever a card is Exhausted
- Lunar Stone - At the start of each combat, transform a card in your draw pile
- Meat on the Bone - When you die, retain your Gold and 1 random relic
- Nuclear Battery - When you gain Block, also gain 1 HP
- Orange Pellets - Whenever you take unblocked damage, deal 3 damage to ALL enemies
- Pandora's Box - At the start of each combat, lose 1 Max HP, gain 3 energy and draw 3 cards
- Peacock Feather - Start each combat with 2 additional cards in hand
- Philosopher's Stone - At the start of each combat, transform a card
- Potent Poison - Whenever you apply Poison, apply 1 additional Poison
- Question Card - At the start of each combat, add 1 random colorless card to your hand
- Ring of the Snake - Start each combat with 1 Artifact
- Sacred Bark - At the start of your turn, if you have no powers, gain 1 Strength
- Shuriken - The first Attack you play each combat deals 4 additional damage
- Snecko Eye - All cards cost 0-3 energy
- Soft Stone - At the end of your turn, gain 1 Block
- Thread and Needle - Start each combat with 1 additional card in hand
- Toy Ornithopter - At the start of your turn, draw 1 card
- Turtle Shell - Start each combat with 5 additional Block
- Velcro Choker - Start each combat with 1 additional energy
- War Paint - Gain 1 Strength whenever you play an Attack

#### Boss Relics (40+ missing)
- Astrolabe - Gain 1 of 3 random stats
- Black Blood - When you take unblocked damage, gain 1 Strength
- Black Star - At the start of combat, enemies lose 1 Max HP
- Bottle of Lightning - Start each combat with 1 additional energy
- Ectoplasm - Cannot heal HP
- Empty Cage - Cannot heal HP
- Guardian's Heart - At the start of combat, if you have 100% HP, gain 1 additional energy
- Lizard Tail - When you die, heal to 50% Max HP (once per run)
- Mark of the Bloom - At the start of each combat, gain 2 Strength
- Panda_Platform - At the start of each combat, add 1 random Power to your hand
- Red Circlet - At the start of combat, if your HP is 50% or less, gain 1 additional energy
- Runic Dome - Start each combat with 10 Block
- Slavers Collar - Start each combat with 1 Strength
- Sozu - Cannot gain status effects (Curses excluded)
- Spirit Poop - Take 1 less damage from attacks
- Thought Process - At the start of combat, gain 3 Block
- Tiny House - Cards cost 1 less for this turn

#### Event Relics (15+ missing)
- Black Blood - When you take unblocked damage, gain 1 Strength
- Face of Cleric - At the start of combat, heal 6 HP
- Golden Idol - Gain 2 Max HP when obtained
- Gremlin Horn - At the end of your turn, gain 1 Strength
- Happy Flower - Gain 1 Max HP when a card is Exhausted
- Lizard Tail - When you die, heal to 50% HP
- Meat on the Bone - When you die, retain Gold and relics
- Medical Kit - Heal 25 HP (only available at low HP)
- Omamori - Negate damage 1 time
- Sacred Bark - Gain 1 Strength when you have no powers
- Tiny House - Cards cost 1 less

#### Special Relics (10+ missing)
- Frozen Eye - Cannot draw cards
- Mark of Pain - Whenever you draw a card, lose 1 HP
- Nloths Anchor - At the start of combat, if HP is 50% or less, gain 1 additional energy
- Strange Spoon - Non-Starter cards cost 1 less

## Implementation Notes

### Current Features
- ✅ **Event-driven relic system** with GameEventListener and BattleEventListener traits
- ✅ **Game-wide effects** (Burning Blood - post-combat healing)
- ✅ **Combat effects** (Anchor - starting block, Blood Vial - starting healing)
- ✅ **Relic registration** and factory pattern for creating event listeners
- ✅ **Character-specific starter relics** (Burning Blood for Ironclad)
- ✅ **Relic storage** in game and battle state

### Required Framework Features
- ❌ **More complex battle events** (card played, damage dealt, etc.)
- ❌ **Status effect manipulation** (relics that modify status applications)
- ❌ **Energy manipulation** (relics that provide additional energy)
- ❌ **HP manipulation** (healing, max HP changes)
- ❌ **Card cost manipulation** (reducing or modifying card costs)
- ❌ **Block manipulation** (block generation and modification)
- ❌ **Deck manipulation** (card transformation, addition, removal)
- ❌ **Combat flow integration** (turn start/end events)
- ❌ **Relic acquisition systems** (chests, events, boss rewards)
- ❌ **Visual relic representation** (icons, descriptions)

### Technical Debt
- **Battle Events**: Need to implement missing battle events (CardPlayed, DamageDealt, etc.)
- **Combat Flow**: Need to integrate relic listeners into turn flow mechanics
- **State Management**: Need to store and manage relic states across saves/loads
- **UI Integration**: Need to display relics and their effects in the interface

## Next Priority Relics

Based on implementation impact and technical complexity, the next relics to implement should be:

### High Priority (Core Mechanics)
1. **Girya** - Introduces card upgrade mechanics
2. **Snecko Eye** - Introduces cost randomization mechanics
3. **Bag of Preparation** - Simple starting combat effect
4. **Holy Water** - Enemy debuff mechanics
5. **Cracked Core** - Defect starter relic (energy generation)
6. **Maw Bank** - Block generation mechanics

### Medium Priority (Advanced Effects)
1. **Calling Bell** - Complex enemy status manipulation
2. **Dead Branch** - Card generation from skills
3. **War Paint** - Attack-triggered stat gains
4. **Self-Forming Clay** - Deck manipulation
5. **Ink Bottle** - Cost manipulation mechanics

### Low Priority (Complex/Niche)
1. **Clockwork Souvenir** - Multi-turn block accumulation
2. **Centennial Puzzle** - Conditional energy generation
3. **Nuclear Battery** - HP-to-Block conversion
4. **Orange Pellets** - Damage reflection mechanics
5. **Pandora's Box** - Risk/reward mechanics

This progression introduces core game mechanics in a logical order, starting with simple combat effects and building up to more complex interactions involving deck manipulation, cost modification, and advanced state management.