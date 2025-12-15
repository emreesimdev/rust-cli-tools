# Rust CLI Tools

A collection of useful command-line interface (CLI) tools and utilities developed while learning Rust.

## Tools List

### 1. Coin Flipper 🪙

A simple RNG-based tool to simulate a coin toss (Heads/Tails).

- **Command:** `cargo run --bin coin_flip`

### 2. RPG Dice Roller (d20) 🎲
A simulation of a 20-sided die roll used in tabletop RPG games like Dungeons & Dragons. Includes logic for critical hits (20) and critical misses (1).

- **Command:** `cargo run --bin dnd_dice`

### 3. Random PIN Generator 🔐
A security tool that generates a random 6-digit numeric PIN code for testing or personal use.

- **Command:** `cargo run --bin pin_gen`

### 4. Russian Roulette (Realistic Mode) 🔫
A high-stakes simulation where the bullet remains in a fixed chamber. The cylinder rotates sequentially after every empty pull, mathematically increasing the risk each turn.

- **Mechanic:** Hidden State & Circular Logic (Cylinder resets after 6).
- **Command:** `cargo run --bin russian_roulette`

### 5. BMI Calculator 🏥
A health utility tool that calculates Body Mass Index based on weight and height inputs, providing health status categories.

- **Command:** `cargo run --bin bmi_calc`

### 6. Tic Tac Toe ❌
A classic command-line version of the popular strategy game where two players compete on a 3x3 grid to align three marks.

- **Command:** `cargo run --bin tic_tac_toe`

## Hangman (CLI)

This project is a work-in-progress Hangman game written in Rust.

### Current state
- Random word selection
- Letter-by-letter guessing logic (draft)
- CLI input handling

### Planned
- Win/lose conditions
- Guess tracking
- Cleaner output formatting


## How to Run

1. Clone the repository.
2. Navigate to the project folder.
3. Run the specific tool using `cargo run --bin tool_name`.

---

Created by **Emre Esim**
