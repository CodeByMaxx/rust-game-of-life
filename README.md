# Conway's Game of Life in Rust + Bevy

A simple implementation of **Conway's Game of Life** written in **Rust** using the **Bevy game engine**.

The project simulates a cellular automaton where cells live, die, and reproduce according to Conway's rules.

![2d Game of Life](Screenshot-1.png)

---

## Features

- ✅ Conway's Game of Life simulation
- ✅ Built with Rust and Bevy
- ✅ Random initial world generation
- ✅ Real-time simulation
- ✅ Toroidal world (edges wrap around)
- ✅ Pause / Resume simulation
- ✅ Generate a new random world

---

## Requirements

You need:

- Rust
- Cargo

Install Rust:

https://www.rust-lang.org/tools/install

Check your installation:

```bash
rustc --version
cargo --version
```

---

## Installation

Clone the repository:

```bash
https://github.com/CodeByMaxx/rust-game-of-life
```

Enter the project directory:

```bash
cd rust-game-of-life
```

Run the application:

```bash
cargo run
```

---

## Controls

| Key | Action |
|---|---|
| Space | Pause / Resume simulation |
| R | Generate a new random world |

---

# How It Works

The simulation uses two separate worlds:

```
current
   |
   | calculate next generation
   v
next
```

The current generation is read, and the next generation is calculated based on the Game of Life rules.

After the calculation, the worlds are swapped.

---

# Conway's Game of Life Rules

Each cell checks its **8 neighboring cells**.

## Living Cell

A living cell survives if it has:

- 2 neighbors
- 3 neighbors

A living cell dies if it has:

- fewer than 2 neighbors (underpopulation)
- more than 3 neighbors (overpopulation)

---

## Dead Cell

A dead cell becomes alive if it has:

- exactly 3 neighbors

---

# Technical Details

## World Size

Current simulation size:

```
150 x 100 cells
```

Each cell has a size of:

```
6 x 6 pixels
```

---

## Data Storage

The world is stored as a one-dimensional array:

```rust
Vec<bool>
```

Example:

```
false false true
false true  true
false false false
```

represents:

```
. . #
. # #
. . .
```

The 2D position is converted into an array index:

```rust
index = y * WIDTH + x
```

---

# Toroidal World

The world has no real borders.

The edges are connected using:

```rust
rem_euclid()
```

Example:

```
0 1 2 3 4

^         |
|_________|
```

Moving outside the left side brings you back on the right side.

Moving outside the top brings you back at the bottom.

This creates a continuous looping world.

---

# Project Structure

```
rust-game-of-life
│
├── Cargo.toml
├── Cargo.lock
│
└── src
    └── main.rs
```

---

# Main Systems

The program is separated into different systems:

## Simulation

Responsible for calculating the next generation:

```rust
update_game()
```

Tasks:

- Count neighbors
- Apply Game of Life rules
- Generate the next state

---

## Rendering

Responsible for drawing the cells:

```rust
draw_game()
```

Tasks:

- Check living cells
- Calculate screen position
- Draw rectangles

---

## Input

Handles user interaction:

```rust
keyboard()
```

Tasks:

- Pause simulation
- Reset world

---

# Used Libraries

## Bevy

Game engine:

https://bevyengine.org/

## Rand

Random number generation:

https://crates.io/crates/rand

---

# Future Improvements

Planned features:

- [ ] Mouse interaction
- [ ] Add/remove cells manually
- [ ] Camera movement
- [ ] Zoom functionality
- [ ] Load predefined patterns
- [ ] Gosper Glider Gun
- [ ] Infinite world simulation
- [ ] Texture-based rendering
- [ ] GPU acceleration

---

# License

MIT License
