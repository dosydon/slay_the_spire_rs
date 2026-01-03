# Colorless Cards Implementation Status

This document tracks the implementation status of all Colorless cards in the Slay the Spire Rust implementation.

## Summary

- ✅ **14 cards implemented**
- 🎯 **57 total Colorless cards** (22 Uncommon + 18 Rare + 17 Special)
- 📋 **Implementation Progress: 24.6%** of Colorless cards

**Note:** All tables include Cost, Cost+ (upgraded cost), Base Effects, and Upgraded Effects columns for clarity.

## About Colorless Cards

Colorless cards are neutral utility cards available to all characters. They are typically obtained through:
- **Shop purchases** (Uncommon and Rare cards)
- **Events** (Special cards)
- **Relics** (Special cards)
- **Other card effects** (Special cards)

Unlike character-specific cards, colorless cards do not appear in normal card reward screens and must be acquired through specific means.

## Card Implementation Status

### Uncommon Cards (22 Total)

| Card Name | Type | Cost | Cost+ | Implemented | File Location | Base Effects | Upgraded Effects | Acquisition |
|-----------|------|------|-------|-------------|---------------|--------------|------------------|-------------|
| ✅ Bandage Up | Skill | 0 | 0 | Yes | [bandage_up.rs](src/cards/colorless/bandage_up.rs) | Heal 4 HP. Exhaust | Heal 6 HP. Exhaust | Shop |
| ✅ Blind | Skill | 0 | 0 | Yes | [blind.rs](src/cards/colorless/blind.rs) | Apply 2 Weak to ALL enemies | Apply 2 Weak to ALL enemies | Shop |
| ✅ Dark Shackles | Skill | 0 | 0 | Yes | [dark_shackles.rs](src/cards/colorless/dark_shackles.rs) | Enemy loses 9 Strength this turn (restored at end of turn). Exhaust | Enemy loses 15 Strength this turn (restored at end of turn). Exhaust | Shop |
| ✅ Deep Breath | Skill | 0 | 0 | Yes | [deep_breath.rs](src/cards/colorless/deep_breath.rs) | Shuffle discard pile into draw pile. Draw 1 card | Shuffle discard pile into draw pile. Draw 2 cards | Shop |
| ❌ Discovery | Skill | 1 | 1 | No | - | Discover 1 of 3 random cards to add to hand, costs 0 this turn. Exhaust | Discover 1 of 3 random cards to add to hand, costs 0 this turn. Exhaust | Shop |
| ✅ Dramatic Entrance | Attack | 0 | 0 | Yes | [dramatic_entrance.rs](src/cards/colorless/dramatic_entrance.rs) | Innate. Deal 8 damage to ALL enemies. Exhaust | Innate. Deal 12 damage to ALL enemies. Exhaust | Shop |
| ❌ Enlightenment | Skill | 0 | 0 | No | - | Reduce cost of cards in hand to 1 this turn | Reduce cost of cards in hand to 1 this combat | Shop |
| ✅ Finesse | Skill | 0 | 0 | Yes | [finesse.rs](src/cards/colorless/finesse.rs) | Gain 2 Block. Draw 1 card | Gain 4 Block. Draw 1 card | Shop |
| ✅ Flash of Steel | Attack | 0 | 0 | Yes | [flash_of_steel.rs](src/cards/colorless/flash_of_steel.rs) | Deal 3 damage. Draw 1 card | Deal 6 damage. Draw 1 card | Shop |
| ❌ Forethought | Skill | 0 | 0 | No | - | Place card(s) from hand on bottom of draw pile. Those cards cost 0 until played | Place card(s) from hand on bottom of draw pile. Those cards cost 0 until played | Shop |
| ✅ Good Instincts | Skill | 0 | 0 | Yes | [good_instincts.rs](src/cards/colorless/good_instincts.rs) | Gain 6 Block | Gain 9 Block | Shop |
| ✅ Impatience | Skill | 0 | 0 | Yes | [impatience.rs](src/cards/colorless/impatience.rs) | If you have no Attack cards in hand, draw 2 cards | If you have no Attack cards in hand, draw 3 cards | Shop |
| ❌ Jack of All Trades | Skill | 0 | 0 | No | - | Add 1 random Colorless card to hand. Exhaust | Add 2 random Colorless cards to hand. Exhaust | Shop |
| ❌ Madness | Skill | 1 | 0 | No | - | A random card in hand costs 0 for rest of combat. Exhaust | A random card in hand costs 0 for rest of combat. Exhaust | Shop |
| ❌ Mind Blast | Attack | 2 | 1 | No | - | Innate. Deal damage equal to number of cards in draw pile | Innate. Deal damage equal to number of cards in draw pile | Shop |
| ✅ Panacea | Skill | 0 | 0 | Yes | [panacea.rs](src/cards/colorless/panacea.rs) | Gain 1 Artifact. Exhaust | Gain 2 Artifact. Exhaust | Shop |
| ✅ Panic Button | Skill | 0 | 0 | Yes | [panic_button.rs](src/cards/colorless/panic_button.rs) | Gain 30 Block. Exhaust | Gain 40 Block. Exhaust | Shop |
| ❌ Purity | Skill | 0 | 0 | No | - | Choose and Exhaust 3 cards in hand. Exhaust | Choose and Exhaust 5 cards in hand. Exhaust | Shop |
| ✅ Swift Strike | Attack | 0 | 0 | Yes | [swift_strike.rs](src/cards/colorless/swift_strike.rs) | Deal 7 damage | Deal 10 damage | Shop |
| ✅ Trip | Skill | 0 | 0 | Yes | [trip.rs](src/cards/colorless/trip.rs) | Apply 2 Vulnerable to ALL enemies | Apply 2 Vulnerable to ALL enemies | Shop |

**Note:** Many uncommon colorless cards cost 0 energy, making them flexible utility options.

### Rare Cards (18 Total)

| Card Name | Type | Cost | Cost+ | Implemented | File Location | Base Effects | Upgraded Effects | Acquisition |
|-----------|------|------|-------|-------------|---------------|--------------|------------------|-------------|
| ❌ Apotheosis | Skill | 2 | 1 | No | - | Upgrade ALL cards for rest of combat. Exhaust | Upgrade ALL cards for rest of combat. Exhaust | Shop |
| ❌ Chrysalis | Skill | 2 | 2 | No | - | Add 3 random Skills to draw pile. They cost 0 this combat. Exhaust | Add 5 random Skills to draw pile. They cost 0 this combat. Exhaust | Shop |
| ❌ Hand of Greed | Attack | 2 | 2 | No | - | Deal 20 damage. If this kills a non-minion enemy, gain 20 Gold | Deal 25 damage. If this kills a non-minion enemy, gain 25 Gold | Shop |
| ❌ Magnetism | Power | 2 | 1 | No | - | At start of turn, add random colorless card to hand | At start of turn, add random colorless card to hand | Shop |
| ✅ Master of Strategy | Skill | 0 | 0 | Yes | [master_of_strategy.rs](src/cards/colorless/master_of_strategy.rs) | Draw 3 cards. Exhaust | Draw 4 cards. Exhaust | Shop |
| ❌ Mayhem | Power | 2 | 1 | No | - | At start of turn, play top card of draw pile | At start of turn, play top card of draw pile | Shop |
| ❌ Metamorphosis | Skill | 2 | 2 | No | - | Add 3 random Attacks to draw pile. They cost 0 this combat. Exhaust | Add 5 random Attacks to draw pile. They cost 0 this combat. Exhaust | Shop |
| ❌ Panache | Power | 0 | 0 | No | - | Every 5 cards played in one turn, deal 10 damage to ALL enemies | Every 5 cards played in one turn, deal 14 damage to ALL enemies | Shop |
| ❌ Sadistic Nature | Power | 0 | 0 | No | - | Whenever you apply a debuff, deal 5 damage to that enemy | Whenever you apply a debuff, deal 7 damage to that enemy | Shop |
| ❌ Secret Technique | Skill | 0 | 0 | No | - | Choose a Skill from draw pile and place it in hand. Exhaust | Choose a Skill from draw pile and place it in hand. Exhaust | Shop |
| ❌ Secret Weapon | Skill | 0 | 0 | No | - | Choose an Attack from draw pile and place it in hand. Exhaust | Choose an Attack from draw pile and place it in hand. Exhaust | Shop |
| ❌ The Bomb | Skill | 2 | 2 | No | - | At end of 3 turns, deal 40 damage to ALL enemies | At end of 3 turns, deal 50 damage to ALL enemies | Shop |
| ❌ Thinking Ahead | Skill | 0 | 0 | No | - | Draw 2 cards. Place 1 card from hand on top of draw pile. Exhaust | Draw 2 cards. Place 1 card from hand on top of draw pile. Exhaust | Shop |
| ❌ Transmutation | Skill | X | X | No | - | Add X random upgraded Colorless cards to hand. They cost 0 this turn. Exhaust | Add X random upgraded Colorless cards to hand. They cost 0 this turn. Exhaust | Shop |
| ❌ Violence | Skill | 0 | 0 | No | - | Place 3 random Attacks from draw pile into hand. Exhaust | Place 4 random Attacks from draw pile into hand. Exhaust | Shop |

**Note:** Rare colorless cards offer powerful effects like permanent upgrades (Apotheosis), card generation (Chrysalis, Metamorphosis), and advanced deck manipulation.

### Special Cards (17 Total)

Special cards cannot be purchased from the shop and are only obtained through specific events, relics, or other card effects.

| Card Name | Type | Cost | Cost+ | Implemented | File Location | Base Effects | Upgraded Effects | How Obtained |
|-----------|------|------|-------|-------------|---------------|--------------|------------------|--------------|
| ❌ Apparition | Skill | 1 | 1 | No | - | Gain 1 Intangible. Exhaust. Ethereal | Gain 1 Intangible. Exhaust. Ethereal | Council of Ghosts event |
| ❌ Beta | Skill | 2 | 1 | No | - | Shuffle an Omega into draw pile. Exhaust | Shuffle an Omega into draw pile. Exhaust | Generated by Alpha card |
| ❌ Bite | Attack | 1 | 1 | No | - | Deal 7 damage. Heal 2 HP | Deal 8 damage. Heal 3 HP | Vampires event |
| ❌ Expunger | Attack | 1 | 1 | No | - | Deal 9 damage X times | Deal 15 damage X times | Generated by Conjure Blade |
| ❌ Insight | Skill | 0 | 0 | No | - | Retain. Draw 2 cards. Exhaust | Retain. Draw 3 cards. Exhaust | Evaluate, Pray, Study cards |
| ❌ J.A.X. | Skill | 0 | 0 | No | - | Lose 3 HP. Gain 2 Strength | Lose 3 HP. Gain 3 Strength | Augmenter event |
| ❌ Miracle | Skill | 0 | 0 | No | - | Retain. Gain 1 Energy. Exhaust | Retain. Gain 2 Energy. Exhaust | Collect, Deus Ex Machina, Pure Water, Holy Water |
| ❌ Omega | Power | 3 | 3 | No | - | At end of turn, deal 50 damage to ALL enemies | At end of turn, deal 60 damage to ALL enemies | Generated by Beta card |
| ❌ Ritual Dagger | Attack | 1 | 1 | No | - | Deal 15 damage. Permanently gain 3 damage if kills enemy | Deal 15 damage. Permanently gain 5 damage if kills enemy | The Nest event |
| ❌ Safety | Skill | 1 | 1 | No | - | Retain. Gain 12 Block. Exhaust | Retain. Gain 16 Block. Exhaust | Deceive Reality card |
| ❌ Shiv | Attack | 0 | 0 | No | - | Deal 4 damage. Exhaust | Deal 6 damage. Exhaust | Blade Dance, Cloak and Dagger, Infinite Blades, etc. |
| ❌ Smite | Attack | 1 | 1 | No | - | Retain. Deal 12 damage. Exhaust | Retain. Deal 16 damage. Exhaust | Carve Reality, Battle Hymn |
| ❌ Through Violence | Attack | 0 | 0 | No | - | Retain. Deal 20 damage. Exhaust | Retain. Deal 30 damage. Exhaust | Reach Heaven card |

**Note:** Special cards are event-exclusive or generated by other cards/relics. Many have Retain, allowing them to persist between turns.
