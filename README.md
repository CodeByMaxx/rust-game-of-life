# Conway's Game of Life in Rust + Bevy

A real-time implementation of **Conway's Game of Life** written in **Rust** using the **Bevy** game engine.

The simulation uses a toroidal world, random initial states and a simple double-buffered update approach.

![Game of Life](Screenshot-1.png)

---

## Features

* Conway's Game of Life simulation
* Rust + Bevy
* Random initial world
* Real-time rendering
* Toroidal world with wrapped edges
* Pause / Resume
* Generate a new random world
* Simple and lightweight implementation

---

## Requirements

You need:

* [Rust](https://www.rust-lang.org/tools/install)
* Cargo

Verify the installation:

```bash
rustc --version
cargo --version
```

---

## Run the Application

Clone the repository:

```bash
git clone https://github.com/CodeByMaxx/rust-game-of-life.git
cd rust-game-of-life
```

Start the application:

```bash
cargo run
```

---

## Controls

| Key     | Action                      |
| ------- | --------------------------- |
| `Space` | Pause / Resume simulation   |
| `R`     | Generate a new random world |

---

## How It Works

The simulation maintains two world states:

```text
Current World
      │
      ▼
Calculate next generation
      │
      ▼
Next World
      │
      ▼
Swap worlds
      │
      └──────► repeat
```

The current generation is read without modifying it directly. The next generation is calculated from the current state and then the two worlds are swapped.

This avoids modifying cells while their neighbours are still being evaluated.

---

## Conway's Game of Life

Every cell checks its eight neighbouring cells.

### Living Cell

A living cell survives with:

* 2 neighbours
* 3 neighbours

A living cell dies with:

* fewer than 2 neighbours — underpopulation
* more than 3 neighbours — overpopulation

### Dead Cell

A dead cell becomes alive with exactly:

* 3 neighbours

These simple rules produce the characteristic emergent behaviour of Conway's Game of Life.

---

## Technical Details

### World Size

The current simulation uses:

```text
150 × 100 cells
```

Each cell is rendered at:

```text
6 × 6 pixels
```

---

### Data Storage

The world is stored as a one-dimensional Rust vector:

```rust
Vec<bool>
```

A two-dimensional coordinate is converted into an array index:

```text
index = y * WIDTH + x
```

For example:

```text
false false true
false true  true
false false false
```

represents:

```text
. . #
. # #
. . .
```

---

## Toroidal World

The simulation has no hard boundaries.

The edges wrap around:

```text
┌─────────────────┐
│                 │
│                 │
│                 │
└─────────────────┘
       ↕
   connected
```

Moving beyond the left edge places the cell on the right side.

Moving beyond the top places the cell at the bottom.

The implementation uses `rem_euclid()` to handle the wrapped coordinates.

This creates a continuous, looping world without borders.

---

## Main Systems

The application separates the main responsibilities into systems.

### Simulation

`update_game()`

Responsible for:

* counting neighbours
* applying the Game of Life rules
* calculating the next generation
* updating the world state

### Rendering

`draw_game()`

Responsible for:

* checking living cells
* calculating screen positions
* drawing the cells

### Input

`keyboard()`

Responsible for:

* pausing and resuming the simulation
* generating a new random world

---

## Project Structure

```text
rust-game-of-life/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
├── Screenshot-1.png
│
└── src/
    └── main.rs
```

---

## Dependencies

The project uses:

### Bevy

The game engine used for rendering, application management and input handling.

https://bevyengine.org/

### Rand

Used for random world generation.

https://crates.io/crates/rand

---

## Future Improvements

Possible extensions include:

* mouse interaction
* manually adding/removing cells
* camera movement
* zoom functionality
* predefined patterns
* Gosper Glider Gun
* infinite-world simulation
* texture-based rendering
* GPU acceleration

---

## License

MIT License

---

## Author

**Markus**

