# Slay the Spire (Rust Implementation)

A Rust implementation of a Slay the Spire-like card game with both full game experience and battle-only modes.

## Features

- **Complete Game Experience**: Navigate a map with different encounter types, manage your deck, and progress through floors
- **Battle System**: Turn-based combat with cards, enemies, and status effects
- **Real Damage Display**: Enemy move previews show actual calculated damage including strength, vulnerable, and block modifiers
- **Multiple Enemy Types**: Red Louse, Green Louse, Jaw Worm, Cultist, Slimes, and Gremlin Nob
- **Ascension Support**: Configurable difficulty scaling


## Gameplay

### Full Game
- Navigate through a branching map
- Choose your path: Combat, Elite, Rest Sites, Events, Shops
- Manage your HP and deck between battles
- Progress through multiple floors

### Battle Mode
- Choose from various enemy encounters
- Play cards to attack and defend
- Use status effects strategically
- See real calculated damage values in enemy move previews

## Architecture

- **Game Logic**: Core game state management, map navigation (`src/game/`)
- **Battle System**: Turn-based combat with effect processing (`src/battle/`)
- **Card System**: Factory pattern for card creation (`src/cards/`)
- **Enemy AI**: Trait-based enemy behaviors with move patterns (`src/enemies/`)
- **CLI Interfaces**: 
  - **GameCli** (`src/game_cli.rs`): Full game experience with map navigation
  - **BattleCli** (`src/battle_cli.rs`): Focused battle simulator

## Building

```bash
# Build both binaries
cargo build --release

# Run tests
cargo test
```

## Technical Highlights

- **Clean Architecture**: Separation between game logic, battle system, and UI
- **Damage Calculation**: Centralized damage computation with proper modifier application
- **Status Effects**: Comprehensive implementation of vulnerable, weak, strength, etc.
- **Enemy Behavior**: Accurate recreation of original game enemy patterns
- **Binary Separation**: Modular design allows for focused gameplay modes