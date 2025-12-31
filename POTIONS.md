# Slay the Spire - Potions Documentation

This document lists all potions available in Slay the Spire, organized by rarity and character availability.

## Implementation Status

| Potion Name | Rarity | Character | Status |
|------------|--------|-----------|---------|
| **Common Potions** |
| Attack Potion | Common | All | ✅ Implemented |
| Blessing of the Forge | Common | All | ❌ Not Implemented |
| Block Potion | Common | All | ✅ Implemented |
| Colorless Potion | Common | All | ❌ Not Implemented |
| Dexterity Potion | Common | All | ✅ Implemented |
| Energy Potion | Common | All | ✅ Implemented |
| Explosive Potion | Common | All | ✅ Implemented |
| Fear Potion | Common | All | ✅ Implemented |
| Fire Potion | Common | All | ✅ Implemented |
| Flex Potion | Common | All | ❌ Not Implemented |
| Power Potion | Common | All | ❌ Not Implemented |
| Skill Potion | Common | All | ❌ Not Implemented |
| Speed Potion | Common | All | ❌ Not Implemented |
| Strength Potion | Common | All | ✅ Implemented |
| Swift Potion | Common | All | ✅ Implemented |
| Weak Potion | Common | All | ✅ Implemented |
| **Character-Specific Common** |
| Blood Potion | Common | Ironclad | ✅ Implemented |
| Poison Potion | Common | Silent | ❌ Not Implemented |
| Focus Potion | Common | Defect | ❌ Not Implemented |
| Bottled Miracle | Common | Watcher | ❌ Not Implemented |
| **Uncommon Potions** |
| Ancient Potion | Uncommon | All | ✅ Implemented |
| Distilled Chaos | Uncommon | All | ❌ Not Implemented |
| Duplication Potion | Uncommon | All | ❌ Not Implemented |
| Essence of Steel | Uncommon | All | ✅ Implemented |
| Gambler's Brew | Uncommon | All | ❌ Not Implemented |
| Liquid Bronze | Uncommon | All | ❌ Not Implemented |
| Liquid Memories | Uncommon | All | ❌ Not Implemented |
| Regen Potion | Uncommon | All | ✅ Implemented |
| Smoke Bomb | Uncommon | All | ❌ Not Implemented |
| Snecko Oil | Uncommon | All | ❌ Not Implemented |
| **Character-Specific Uncommon** |
| Elixir | Uncommon | Ironclad | ❌ Not Implemented |
| Cunning Potion | Uncommon | Silent | ❌ Not Implemented |
| Potion of Capacity | Uncommon | Defect | ❌ Not Implemented |
| Stance Potion | Uncommon | Watcher | ❌ Not Implemented |
| **Rare Potions** |
| Cultist Potion | Rare | All | ❌ Not Implemented |
| Entropic Brew | Rare | All | ❌ Not Implemented |
| Fairy in a Bottle | Rare | All | ❌ Not Implemented |
| Fruit Juice | Rare | All | ❌ Not Implemented |
| **Character-Specific Rare** |
| Heart of Iron | Rare | Ironclad | ❌ Not Implemented |
| Ghost in a Jar | Rare | Silent | ❌ Not Implemented |
| Essence of Darkness | Rare | Defect | ❌ Not Implemented |
| Ambrosia | Rare | Watcher | ❌ Not Implemented |

**Summary**: 14 / 45 potions implemented (31.1%)

---

## Potion Mechanics

- **Potion Slots**: Players start with 3 potion slots (can be increased with certain relics)
- **Usage**: Potions can only be used during combat (except Smoke Bomb)
- **Acquisition**: Potions can be found after combat, purchased from shops, or obtained from events
- **Discarding**: Potions can be discarded to make room for new ones

---

## Common Potions (Available to All Characters)

### Attack Potion
- **Effect**: Choose 1 of 3 random Attack cards to add to your hand. It costs 0 this turn.
- **Target**: Player
- **Usage**: Combat only

### Blessing of the Forge
- **Effect**: Upgrades all cards in your hand for the rest of combat
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Upgrade persists even if cards leave and return to hand

### Block Potion
- **Effect**: Gain 12 Block
- **Target**: Player
- **Usage**: Combat only

### Colorless Potion
- **Effect**: Choose 1 of 3 random Colorless cards to add to your hand. It costs 0 this turn.
- **Target**: Player
- **Usage**: Combat only

### Dexterity Potion
- **Effect**: Gain 2 Dexterity
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Permanent for the combat (does not wear off)

### Energy Potion
- **Effect**: Gain 2 Energy
- **Target**: Player
- **Usage**: Combat only

### Explosive Potion
- **Effect**: Deal 10 damage to ALL enemies
- **Target**: All enemies
- **Usage**: Combat only

### Fear Potion
- **Effect**: Apply 3 Vulnerable
- **Target**: Single enemy
- **Usage**: Combat only

### Fire Potion
- **Effect**: Deal 20 damage
- **Target**: Single enemy
- **Usage**: Combat only

### Flex Potion
- **Effect**: Gain 5 Strength. At the end of your turn, lose 5 Strength.
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Temporary strength buff for one turn

### Power Potion
- **Effect**: Choose 1 of 3 random Power cards to add to your hand. It costs 0 this turn.
- **Target**: Player
- **Usage**: Combat only

### Skill Potion
- **Effect**: Choose 1 of 3 random Skill cards to add to your hand. It costs 0 this turn.
- **Target**: Player
- **Usage**: Combat only

### Speed Potion
- **Effect**: Gain 5 Dexterity. At the end of your turn, lose 5 Dexterity.
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Temporary dexterity buff for one turn

### Strength Potion
- **Effect**: Gain 2 Strength
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Permanent for the combat (does not wear off)

### Swift Potion
- **Effect**: Draw 3 cards
- **Target**: Player
- **Usage**: Combat only

### Weak Potion
- **Effect**: Apply 3 Weak
- **Target**: Single enemy
- **Usage**: Combat only

---

## Character-Specific Common Potions

### Blood Potion (Ironclad)
- **Effect**: Heal for 20% of your Max HP
- **Target**: Player
- **Usage**: Combat only

### Poison Potion (Silent)
- **Effect**: Apply 6 Poison
- **Target**: Single enemy
- **Usage**: Combat only

### Focus Potion (Defect)
- **Effect**: Gain 2 Focus
- **Target**: Player
- **Usage**: Combat only

### Bottled Miracle (Watcher)
- **Effect**: Add 2 Miracle cards to your hand
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Miracles are 0-cost cards that grant 1 Energy and Retain

---

## Uncommon Potions (Available to All Characters)

### Ancient Potion
- **Effect**: Gain 1 Artifact
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Artifact prevents the next debuff applied to you

### Distilled Chaos
- **Effect**: Play the top 3 cards of your draw pile
- **Target**: Player/Varies
- **Usage**: Combat only
- **Notes**: Cards are played in order; targeting is random for attack cards

### Duplication Potion
- **Effect**: This turn, your next card is played twice
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Does not consume the card; works with all card types

### Essence of Steel
- **Effect**: Gain 4 Plated Armor
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Plated Armor reduces damage at the end of each turn (permanent armor)

### Gambler's Brew
- **Effect**: Discard your hand, then draw that many cards
- **Target**: Player
- **Usage**: Combat only

### Liquid Bronze
- **Effect**: Gain 3 Thorns
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Thorns reflect damage back to attackers

### Liquid Memories
- **Effect**: Choose a card in your discard pile and add it to your hand. It costs 0 this turn.
- **Target**: Player
- **Usage**: Combat only

### Regen Potion
- **Effect**: Gain 5 Regeneration
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Heals 5 HP at the end of each turn, decreases by 1 each turn

### Smoke Bomb
- **Effect**: Escape from a non-boss combat. Receive no rewards.
- **Target**: Player
- **Usage**: Combat only (non-boss encounters)
- **Notes**: Immediately ends combat with no rewards

### Snecko Oil
- **Effect**: Draw 5 cards. Randomize the costs of all cards in your hand.
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Card costs become random values between 0-3

---

## Character-Specific Uncommon Potions

### Elixir (Ironclad)
- **Effect**: Exhaust any number of cards in your hand
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Useful for removing Status/Curse cards or triggering exhaust synergies

### Cunning Potion (Silent)
- **Effect**: Add 3 Shivs to your hand
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Shivs are 0-cost Attack cards that deal 4 damage and Exhaust

### Potion of Capacity (Defect)
- **Effect**: Gain 2 Orb slots
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Permanent for the combat

### Stance Potion (Watcher)
- **Effect**: Enter Calm or Wrath (your choice)
- **Target**: Player
- **Usage**: Combat only

---

## Rare Potions (Available to All Characters)

### Cultist Potion
- **Effect**: Gain 1 Ritual
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Ritual grants 1 Strength at the end of each turn

### Entropic Brew
- **Effect**: Fill all empty potion slots with random potions
- **Target**: Player
- **Usage**: Any time
- **Notes**: Can be used outside combat; potions generated are random

### Fairy in a Bottle
- **Effect**: When you would die, heal to 30% of your Max HP instead and discard this potion
- **Target**: Player (passive effect)
- **Usage**: Automatic trigger
- **Notes**: Does not need to be manually used; activates when fatal damage is taken

### Fruit Juice
- **Effect**: Gain 5 Max HP
- **Target**: Player
- **Usage**: Any time
- **Notes**: Permanent increase to maximum HP; can be used outside combat

---

## Character-Specific Rare Potions

### Heart of Iron (Ironclad)
- **Effect**: Gain 6 Metallicize
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Metallicize grants Block at the end of each turn

### Ghost in a Jar (Silent)
- **Effect**: Gain 1 Intangible
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Intangible reduces all damage taken to 1

### Essence of Darkness (Defect)
- **Effect**: Channel 1 Dark orb for each orb slot
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Dark orbs deal 6 damage when evoked and increase by 6 each turn

### Ambrosia (Watcher)
- **Effect**: Enter Divinity stance
- **Target**: Player
- **Usage**: Combat only
- **Notes**: Divinity triples all damage dealt; exits after playing 3 attacks

---

## Potion Drop Rates

- **Common Potions**: ~75% drop rate
- **Uncommon Potions**: ~20% drop rate
- **Rare Potions**: ~5% drop rate

## Implementation Notes

### Targeting Requirements
- **No Target Required**: Block Potion, Strength Potion, Dexterity Potion, Energy Potion, Swift Potion, Blood Potion, Focus Potion, etc. (most buff potions)
- **Enemy Target Required**: Fire Potion, Fear Potion, Weak Potion, Poison Potion
- **All Enemies**: Explosive Potion
- **Card Selection**: Attack Potion, Power Potion, Skill Potion, Colorless Potion, Liquid Memories
- **Automatic/Passive**: Fairy in a Bottle

### Special Mechanics
1. **Card Addition Potions**: Attack/Power/Skill/Colorless Potions show 3 options to choose from
2. **Temporary Buffs**: Flex Potion and Speed Potion buffs expire at end of turn
3. **Passive Triggers**: Fairy in a Bottle automatically activates on fatal damage
4. **Outside Combat**: Entropic Brew and Fruit Juice can be used outside combat
5. **Combat Exit**: Smoke Bomb immediately ends non-boss combat
