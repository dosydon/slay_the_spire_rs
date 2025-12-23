# Slay the Spire - Curse Cards

Curses are negative cards that, unlike Status cards, stay in your deck outside of combat. Under normal circumstances, all curses are **Unplayable**, and many impose an additional negative effect.

## Implementation Status

- ✅ **5 curse cards implemented** (36% of curses)
- ❌ **9 curse cards not yet implemented**
- 🎯 **Implementation Progress: ~36%** of all curses
- 📊 **Total Curses: 14** (11 Removable + 3 Non-Removable + 1 Playable)

## Curse Card Categories

### Randomly Generated Curses (Removable)
When an effect (such as Cursed Key or Transform) generates a random curse, it can only become one of these 10 curses:
- Clumsy
- Decay
- Doubt
- Injury
- Normality
- Pain
- Parasite
- Regret
- Shame
- Writhe

### Special Curse (Removable)
- **Pride** - The only removable curse that can be played (obtained from Grotesque Trophy in Endless runs)

### Non-Removable Curses
These curses cannot be removed from your deck at shops or with Remove Card events:
- Ascender's Bane
- Curse of the Bell
- Necronomicurse

---

## Implementation Status Table

| Curse Name | Type | Implemented | File Location | Notes |
|------------|------|-------------|---------------|-------|
| ✅ **Ascender's Bane** | Non-Removable | Yes | `src/cards/curse/mod.rs` | Ethereal, unplayable |
| ✅ **Clumsy** | Removable | Yes | `src/cards/curse/mod.rs` | Ethereal, exhausts at end of turn |
| ❌ **Curse of the Bell** | Non-Removable | No | - | Cannot be removed or exhausted |
| ❌ **Decay** | Removable | No | - | Deals 2 damage at end of turn |
| ❌ **Doubt** | Removable | No | - | Gain 1 Weak at end of turn |
| ✅ **Injury** | Removable | Yes | `src/cards/curse/mod.rs` | No effect, unplayable |
| ❌ **Necronomicurse** | Non-Removable | No | - | Cannot be removed or exhausted (creates copy) |
| ❌ **Normality** | Removable | No | - | Limits cards played to 3 per turn |
| ❌ **Pain** | Removable | No | - | Lose 1 HP when other cards played |
| ❌ **Parasite** | Removable | No | - | Lose 3 Max HP if transformed/removed |
| ✅ **Regret** | Removable | Yes | `src/cards/curse/mod.rs` | Lose 1 HP per card in hand at end of turn |
| ❌ **Shame** | Removable | No | - | Gain 1 Frail at end of turn |
| ✅ **Writhe** | Removable | Yes | `src/cards/curse/mod.rs` | Innate, starts in every hand |

### Implementation Notes

#### ✅ Fully Implemented Curses (5/14)

1. **Ascender's Bane** (`ascenders_curse()`)
   - Properties: Ethereal, Unplayable, Non-removable
   - Effect: None (negative space in deck)
   - Test Coverage: ✅ Complete

2. **Clumsy** (`clumsy()`)
   - Properties: Ethereal, Unplayable, Removable
   - Effect: None (exhausts at end of turn via Ethereal)
   - Test Coverage: ✅ Complete

3. **Injury** (`injury()`)
   - Properties: Unplayable, Removable
   - Effect: None (pure negative space in deck)
   - Test Coverage: ✅ Complete

4. **Regret** (`regret()`)
   - Properties: Ethereal, Unplayable, Removable
   - Effect: `LoseHpPerCardInHand { damage_per_card: 1 }` at end of turn
   - Test Coverage: ✅ Complete

5. **Writhe** (`writhe()`)
   - Properties: Innate, Unplayable, Removable
   - Effect: Always starts in hand (takes up a card slot)
   - Test Coverage: ✅ Complete

#### ❌ Not Yet Implemented Curses (9/14)

**High Priority** (Simple mechanics):
- **Decay** - EndOfTurn damage effect (similar to Regret)
- **Doubt** - Apply Weak at end of turn
- **Shame** - Apply Frail at end of turn

**Medium Priority** (Unique mechanics):
- **Normality** - Card play limit per turn
- **Pain** - Trigger on card play (during turn)

**Low Priority** (Complex systems):
- **Curse of the Bell** - Special non-removable property
- **Necronomicurse** - Copies itself when exhausted
- **Parasite** - Max HP loss on removal/transformation

---

## Complete Curse List

### Ascender's Bane
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable. Ethereal.
- **Properties**:
  - Cannot be removed from deck
  - Can be exhausted (via Ethereal)
- **Obtained**: Starting card from Ascension level 10+

---

### Clumsy
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable. Ethereal.
- **Properties**:
  - Can be removed from deck
  - Exhausts at end of turn (via Ethereal)
- **Obtained**: Random curse generation, various events and relics

---

### Curse of the Bell
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable.
- **Properties**:
  - Cannot be removed from deck via shops or events
  - Can be exhausted using cards like Blue Candle or Purity
- **Obtained**: Calling Bell relic event

---

### Decay
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable. At the end of your turn, take 2 damage.
- **Properties**: Can be removed from deck
- **Obtained**: Random curse generation, various events and relics

---

### Doubt
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable. At the end of your turn, gain 1 Weak.
- **Properties**: Can be removed from deck
- **Obtained**: Random curse generation, various events and relics

---

### Injury
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable.
- **Properties**: Can be removed from deck
- **Obtained**: Random curse generation, various events and relics

---

### Necronomicurse
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable.
- **Properties**:
  - Cannot be removed from deck
  - Cannot be transformed
  - Cannot be exhausted (creates a new copy if exhausted)
  - Most difficult curse to deal with
- **Obtained**: Necronomicon event, Necronomicon relic curse

---

### Normality
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable. You cannot play more than 3 cards this turn.
- **Properties**: Can be removed from deck
- **Obtained**: Random curse generation, various events and relics

---

### Pain
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable. While in hand, lose 1 HP when other cards are played.
- **Properties**: Can be removed from deck
- **Obtained**: Random curse generation, various events and relics

---

### Parasite
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable. If transformed or removed from your deck, lose 3 Max HP.
- **Properties**:
  - Can be removed from deck (with penalty)
  - Removing inflicts permanent damage
- **Obtained**: Random curse generation, various events and relics

---

### Pride
- **Cost**: 1
- **Type**: Curse (Special rarity)
- **Effect**: Innate. Exhaust. At the end of your turn, put a card from your hand on top of your draw pile.
- **Properties**:
  - **Only curse that can be played**
  - Can be removed from deck
  - Has special rarity
- **Obtained**: Grotesque Trophy blight (Endless mode only)

---

### Regret
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable. At the end of your turn, lose 1 HP for each card in your hand.
- **Properties**: Can be removed from deck
- **Obtained**: Random curse generation, various events and relics

---

### Shame
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable. At the end of your turn, gain 1 Frail.
- **Properties**: Can be removed from deck
- **Obtained**: Random curse generation, various events and relics

---

### Writhe
- **Cost**: Unplayable
- **Type**: Curse
- **Effect**: Unplayable. Innate.
- **Properties**:
  - Can be removed from deck
  - Innate (always starts in hand)
- **Obtained**: Random curse generation, various events and relics

---

## Notes

- **Total curse cards**: 14
- **Implemented**: 5 (Ascender's Bane, Clumsy, Injury, Regret, Writhe)
- **Not implemented**: 9 (Decay, Doubt, Shame, Normality, Pain, Parasite, Pride, Curse of the Bell, Necronomicurse)
- **Removable curses**: 11 (Clumsy, Decay, Doubt, Injury, Normality, Pain, Parasite, Pride, Regret, Shame, Writhe)
- **Non-removable curses**: 3 (Ascender's Bane, Curse of the Bell, Necronomicurse)
- **Playable curse**: 1 (Pride) - Not yet implemented
- **Random curse pool**: 10 (excludes Ascender's Bane, Curse of the Bell, Necronomicurse, and Pride)

### Implementation Features

#### ✅ Supported Features
- **Unplayable** property - Cards cannot be played
- **Ethereal** property - Cards exhaust at end of turn
- **Innate** property - Cards start in every hand
- **End of turn effects** - Regret's damage effect
- **Removable** property - Cards can be removed from deck
- **Non-removable** property - Cards cannot be removed

#### ❌ Required Features for Remaining Curses
- **Card play limit** - Normality (max 3 cards per turn)
- **Card play trigger** - Pain (lose HP when other cards played)
- **Removal transformation hook** - Parasite (lose Max HP on removal)
- **Exhaust prevention** - Necronomicurse (copy on exhaust)
- **Playable curses** - Pride (only playable curse)
- **Status effect application** - Doubt/Shame (Weak/Frail at end of turn)
- **Direct damage** - Decay (fixed damage at end of turn)

### Framework Requirements

Most missing curses can be implemented once these systems are in place:

1. **End of Turn Effect Processing** ✅ Already exists (Regret uses it)
   - Decay: Apply damage
   - Doubt: Apply Weak
   - Shame: Apply Frail

2. **Card Play Tracking** ❌ Not yet implemented
   - Normality: Count cards played per turn
   - Pain: Trigger when other cards played

3. **Transformation/Removal Hooks** ❌ Not yet implemented
   - Parasite: React to deck transformation/removal

4. **Exhaust Interception** ❌ Not yet implemented
   - Necronomicurse: Copy itself on exhaust attempt

5. **Card Play Limits** ❌ Not yet implemented
   - Normality: Prevent playing >3 cards per turn

## Sources
- Slay the Spire Wiki (slaythespire.wiki.gg)
- Slay the Spire Fandom Wiki
