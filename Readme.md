# Cyber Path – Terminal Games in Rust

A collection of retro-style terminal-based games written in Rust, featuring cyberpunk aesthetics and increasing difficulty.

## Game Modes

### 1. Path Mode (Memory Challenge)

Watch a path, then recreate it from memory. Paths get longer each round!

```
                              CYBER PATH
┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
└────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘

                     Round 1  |  3 moves  |  Q to quit
                            Watch the path...
```

### 2. Chase Mode (Evasion)

Flee from a pursuing bot that gets faster and smarter each round!

```
                              CYBER CHASE
┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
└────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘

              Flee from the bot! Survive 15 moves | WASD / Arrow keys
                Round 1  |  0/15  survived  |  Bot speed: 1  |  Q to quit
```

### 3. Snake Mode (Classic)

Collect coins, grow your snake, and avoid crashing!

```
                              CYBER SNAKE
┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
└────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘

                       Score: 0  |  Level: 1  |  Q to quit
                         Collect coins! WASD / Arrow keys
```

### 4. Flappy Mode (Reaction)

Navigate through moving pipes by jumping at the right time!

```
                              CYBER FLAPPY
┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
└────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘

                           Score: 0  |  Q to quit
                           Press SPACE to jump!
```

---

## Features

- Grid-based board rendered with Unicode box-drawing characters
- Four game modes with different gameplay mechanics
- Progressive difficulty scaling
- Sound effects (movement, winning, losing)
- Terminal rendering using alternate screen buffer

---

## Controls

### Path, Chase & Snake Modes

| Key              | Action     |
| ---------------- | ---------- |
| W / ↑            | Move up    |
| A / ←            | Move left  |
| S / ↓            | Move down  |
| D / →            | Move right |
| Q / Esc          | Quit game  |
| R (after defeat) | Restart    |

### Flappy Mode

| Key              | Action    |
| ---------------- | --------- |
| Space            | Jump      |
| Q / Esc          | Quit game |
| R (after defeat) | Restart   |

---

## Installation

### Prerequisites

- Rust (stable toolchain recommended)
- Install via: [https://rustup.rs](https://rustup.rs)

### Build & Run

```bash
# Clone the repository
git clone <repository-url>
cd cyber-path

# Build release version
cargo build --release

# Run with default mode (Path)
cargo run --release

# Run specific game mode
cargo run --release -- --mode path
cargo run --release -- --mode chase
cargo run --release -- --mode snake
cargo run --release -- --mode flappy

# View help
cargo run -- --help
```

---

## Project Structure

```
src/
 ├── main.rs        # Entry point and game loop management
 ├── game.rs        # Path mode: memory challenge logic
 ├── chase.rs       # Chase mode: evasion gameplay
 ├── snake.rs       # Snake mode: classic snake implementation
 ├── flappy.rs      # Flappy mode: obstacle avoidance
 ├── models.rs      # Core data structures (Position, Direction)
 ├── ui.rs          # Terminal rendering and UI components
 └── input.rs       # Keyboard input handling
```

---

## Gameplay Details

### Path Mode

1. A random path is generated and displayed
2. Path is shown for a preview duration (shorter each round)
3. Player must recreate the exact path from memory
4. Each successful round adds more moves to memorize

### Chase Mode

1. Player and bot spawn at opposite corners
2. Player must survive a set number of moves
3. Bot pursues using pathfinding (with 25% random moves)
4. Each round increases required survival moves and bot speed

### Snake Mode

1. Control a snake that grows by collecting coins
2. Avoid walls and your own body
3. Speed increases with score
4. Level up every 5 coins collected

### Flappy Mode

1. Player auto-falls due to gravity
2. Press space to jump upward
3. Navigate through gaps in moving pipes
4. Pipes spawn continuously and move left
5. Score increases for each pipe passed

---

## Testing

Run the full test suite:

```bash
cargo test
```

All game modes include comprehensive unit tests covering:

- Game state transitions
- Movement validation
- Collision detection
- Score/round progression
- Boundary conditions

---

## Dependencies

```toml
anyhow = "1.0.102"
clap = { version = "4.5.60", features = ["derive"] }
crossterm = "0.29.0"
rand = "0.10.0"
kira = "0.12.0"
```

---

## Configuration

Game parameters are tuned for balanced gameplay but can be adjusted in the source:

- **Board size**: Automatically calculated based on terminal dimensions (5-16 cols, 4-10 rows)
- **Path mode**: Preview duration and step delay decrease with rounds
- **Chase mode**: Bot speed formula: `1 + (round-1) / 3`
- **Snake mode**: Tick speed formula: `300ms - (score * 10ms)`, min 100ms
- **Flappy mode**: Tick speed formula: `150ms - (score * 5ms)`, min 80ms

---

## License

MIT License
