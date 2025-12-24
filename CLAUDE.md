# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust implementation of a Slay the Spire-like card game. The codebase is structured as a turn-based card battler where players fight enemies using cards from their deck.

## Build Commands

Standard Rust project managed with Cargo:
- `cargo build` - Build the project
- `cargo run` - Run the main executable
- `cargo test` - Run all tests
- `cargo check` - Check for compilation errors without building

Note: Rust toolchain may not be installed in all environments - check with `cargo --version` first.

## Architecture

### Core Game Systems

The game is organized into three main modules:

**`src/game/`** - Core game mechanics
- `battle.rs` - Main battle orchestration, turn management, and game state
- `card.rs` - Card structure with name, cost, type, and effects
- `effect.rs` - Game effects system (damage, defense, status effects)
- `enemy.rs` - Enemy trait definitions and behavior
- `deck.rs` - Deck management and card drawing
- `action.rs` - Player action enumeration
- `game_event.rs` - Game-wide event system (combat victory, etc.)
- `map.rs` - Map navigation and node management
- `card_reward.rs` - Card reward selection system

**`src/cards/`** - Card implementations
- `ironclad/` - Character-specific cards (Strike, Defend, Bash, etc.)
- `colorless/` - Neutral cards available to all characters
- `status/` - Status effect cards (Wound, etc.)
- `curse/` - Curse cards (Ascender's Curse, Injury, etc.)
- Cards are implemented as factory functions returning `Card` instances

**`src/enemies/`** - Enemy implementations
- Each enemy implements `EnemyTrait` with HP bounds, moves, and names
- `red_louse.rs` - Basic enemy implementation
- `jaw_worm.rs` - Enemy with varied move patterns
- `green_louse.rs` - Variant of louse enemy
- `acid_slime_m.rs` - Medium acid slime enemy
- `cultist.rs` - Enemy with power mechanics
- `gremlin_nob.rs` - Elite enemy with multi-phase combat
- `spike_slime_m.rs` - Medium spike slime enemy

**`src/relics/`** - Relic system
- `burning_blood.rs` - Heals 6 HP at combat victory (GameEventListener)
- `anchor.rs` - Starts combat with 10 block (BattleEventListener)
- `blood_vial.rs` - Heals 2 HP at combat start (BattleEventListener)
- `lantern.rs` - Gain 1 Energy on first turn of each combat (BattleEventListener)
- Relics use event-driven architecture with separate Game and Battle event systems

### Key Design Patterns

1. **Effect System**: Cards and enemies use a unified `Effect` enum for all game actions (damage, defense, status effects)

2. **Entity-Component Pattern**:
   - `Player` struct contains HP, block, and energy
   - `Card` struct contains name, cost, type, and effects vector
   - `Battle` orchestrates all game entities

3. **Factory Pattern**: Cards are created via functions (e.g., `strike()`) rather than direct construction

4. **Trait-Based Enemies**: All enemies implement `EnemyTrait` for consistent behavior

5. **Event-Driven Relics**: Relics implement either `GameEventListener` or `BattleEventListener` traits to respond to specific game events
   - Game events: combat victory, map navigation, etc.
   - Battle events: combat start, end of turn, etc.

6. **Modular Relic System**: `Relic` enum provides `to_game_event_listener()` and `to_battle_event_listener()` methods for automatic conversion to appropriate listeners

### Current State

The codebase has evolved significantly with major systems implemented:

**✅ Implemented Systems:**
- **Complete Relic System**: Event-driven relics with both Game and Battle event listeners
- **Map Navigation**: Visual map system with node connections and path choices
- **Battle System**: Functional combat with card playing, enemy AI, and turn management
- **CLI Interface**: Full game flow through `GameCli` with map navigation, battles, and card rewards
- **Card Effects**: Most card effects are properly implemented and processed
- **Enemy AI**: Multiple enemy types with unique moves and behaviors
- **Event System**: Separate Game and Battle event systems for different scopes
- **Potion System**: 13 potions implemented with rarity-based distribution (Common/Uncommon)
- **Colorless Cards**: 14 colorless cards implemented (Dramatic Entrance and 13 others)
- **Regen Mechanic**: Regeneration system with end-of-turn healing that decreases by 1 each turn
- **Relic Rewards**: Elite combats provide guaranteed relic rewards (85% Uncommon, 15% Rare)

**⚠️ Partially Implemented:**
- **Effect Queue**: Basic structure exists but some complex effects may need refinement
- **Status Effects**: Some status effects implemented but may need expansion

**🎯 How to Add New Relics:**
1. Create relic struct implementing `GameEventListener` or `BattleEventListener`
2. Add variant to `Relic` enum in `src/relics/mod.rs`
3. Update `to_game_event_listener()` or `to_battle_event_listener()` methods
4. Add relic to game using `game.add_relic(Relic::YourRelic)`
5. **IMPORTANT**: Update `RELICS.md` to mark the relic as implemented with proper cost and effects

**🎯 How to Add New Cards:**
1. Create factory function in appropriate character module (e.g., `src/cards/ironclad/` or `src/cards/colorless/`)
2. Create both base and upgraded versions (e.g., `card_name()` and `card_name_upgraded()`)
3. Define effects using the `BattleEffect` enum
4. Add card variant to `CardEnum` in `src/game/card_enum.rs`
5. Add name mapping in `CardEnum::name()` method
6. Add `to_card()` implementation in `CardEnum`
7. Add upgrade() match arm in `src/game/card.rs`
8. Add card to `src/game/card_reward.rs` for reward pools
9. **IMPORTANT**: Update the appropriate documentation file:
   - For Ironclad cards: `IRONCLAD_CARDS.md`
   - For Colorless cards: `COLORLESS_CARDS.md`
   - Include: Base cost, upgraded cost, base effects, upgraded effects, file location

When implementing new features, follow the existing patterns of effects-based actions, trait-based entities, and event-driven architecture.

## Documentation Maintenance

**CRITICAL: Always keep documentation synchronized with code changes.**

When you implement a new card or relic:
- ✅ **DO**: Update the corresponding markdown file immediately after implementation:
  - `IRONCLAD_CARDS.md` for Ironclad cards
  - `COLORLESS_CARDS.md` for colorless cards
  - `RELICS.md` for relics
  - `POTIONS.md` for potions
- ✅ **DO**: Include all required details: cost, upgraded cost, base effects, upgraded effects, file location
- ✅ **DO**: Update the summary statistics (total implemented count, percentage progress)
- ✅ **DO**: Add to "Recently Implemented" section if it introduces new mechanics
- ❌ **DON'T**: Leave documentation outdated - it causes confusion and duplicate work
- ❌ **DON'T**: Assume someone else will update the docs later

**Documentation Format:**
- Cards require: `Cost`, `Cost+`, `Base Effects`, `Upgraded Effects`, `File Location`
- Relics require: Implementation status, tier, effects, event listener type
- Both require: Clear description of what the feature does

## Reference Documents

**Use the following markdown files as reference when working on this project:**

- **`IRONCLAD_CARDS.md`** - Comprehensive documentation of Ironclad card implementation status
  - Lists all implemented and missing Ironclad cards with their effects
  - Includes implementation notes and technical debt tracking
  - Shows next priority cards for implementation
  - Documents recently implemented cards and their mechanics

- **`COLORLESS_CARDS.md`** - Complete documentation of colorless card implementation status
  - Lists all 57 colorless cards across Uncommon, Rare, and Special categories
  - Currently 14 cards implemented (24.6% completion rate)
  - Documents required framework features (Innate, Retain, Ethereal, etc.)
  - Includes recommended implementation order and technical considerations
  - Shows recently implemented cards with detailed mechanics

- **`RELICS.md`** - Complete documentation of relic implementation status
  - Lists all implemented and missing relics across all categories
  - Explains the event-driven relic architecture
  - Documents both GameEventListener and BattleEventListener patterns
  - Shows next priority relics for implementation
  - Includes implementation notes and required framework features

- **`POTIONS.md`** - Comprehensive documentation of potion implementation status
  - Lists all 45 potions with rarity and character availability
  - Currently 13 potions implemented (28.9% completion rate)
  - Documents potion mechanics and drop rates
  - Includes targeting requirements and special mechanics

- **`ENEMIES.md`** - Complete documentation of enemy implementation status
  - Lists all implemented enemies with their behaviors and patterns
  - Documents encounter types and spawn mechanics

- **`EVENTS.md`** - Documentation of map events implementation status

**How to Use These References:**
1. **Check Implementation Status**: Before implementing a new feature, check the appropriate file to see if it's already implemented
2. **Follow Patterns**: Use the documented implementation patterns for consistency
3. **Avoid Duplication**: Prevent implementing already-completed features
4. **Prioritize Work**: Use the "Next Priority" sections to focus on impactful features
5. **Understand Architecture**: Read the technical notes to understand the current system capabilities
6. **Keep Documentation Updated**: After implementing any feature, immediately update the corresponding markdown file with all details (costs, effects, file location)