# Rust CLI Tools

This repository contains small command-line tools I built while learning Rust.  
Each tool focuses on a specific concept such as input handling, control flow, randomness, or basic game logic.

---

## Included Tools

### Coin Flipper 🪙
A simple random number based tool that simulates a coin toss (Heads / Tails).

- Binary: `coin_flip`  
- Run with: `cargo run --bin coin_flip`

---

### RPG Dice Roller (d20) 🎲
A 20-sided dice simulator inspired by tabletop RPG games like Dungeons & Dragons.  
Includes logic for critical hits (20) and critical misses (1).

- Binary: `dnd_dice`  
- Run with: `cargo run --bin dnd_dice`

---

### Random PIN Generator 🔐
Generates a random 6-digit numeric PIN.  
Built to practice randomness, formatting, and basic output control.

- Binary: `pin_gen`  
- Run with: `cargo run --bin pin_gen`

---

### Russian Roulette (Sequential Logic) 🔫
A simulation where the bullet stays in a fixed chamber and the cylinder advances after each trigger pull.  
The risk increases mathematically with every turn until the cylinder resets.

- Focus: hidden state, circular logic  
- Binary: `russian_roulette`  
- Run with: `cargo run --bin russian_roulette`

---

### BMI Calculator 🏥
Calculates Body Mass Index based on user input and outputs a health category.

- Focus: input parsing, float math  
- Binary: `bmi_calc`  
- Run with: `cargo run --bin bmi_calc`

---

### Tic Tac Toe ❌
A two-player command-line Tic Tac Toe game played in the terminal.  
Built to practice vector usage, indexing, and basic game state management.

- Binary: `tic_tac_toe`  
- Run with: `cargo run --bin tic_tac_toe`

---

### Hangman 🔤
Simple CLI Hangman game built while learning Rust fundamentals.
Focused on vectors, chars, loops, input handling and basic game logic.

- Binary: `hangman`
- Run with: `cargo run --bin hangman`

---

## How to Run

Clone the repository, then run any tool using Cargo:

```bash
cargo run --bin tool_name
```
Each tool is implemented as a separate binary.

---

Created by Emre Esim
