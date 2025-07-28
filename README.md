# Snake Game 🐍

A classic Snake game built in Rust using the ggez game engine.

## Features

- **Classic Snake gameplay** - Eat food, grow longer, avoid walls and yourself
- **Visual grid** - Clear grid lines and borders show the play area
- **Score tracking** - See your current score in real-time
- **Sound effects** - Audio feedback for eating food and game over (optional)
- **Restart functionality** - Press 'R' to restart after game over
- **Smart food spawning** - Food never spawns on the snake's body

## Controls

- **Arrow Keys** - Move the snake (Up, Down, Left, Right)
- **R** - Restart game after game over

## Installation & Running

1. Make sure you have [Rust installed](https://rustup.rs/)
2. Clone this repository:
   ```bash
   git clone <your-repo-url>
   cd snake
   ```
3. Run the game:
   ```bash
   cargo run
   ```

## Optional: Sound Effects

To enable sound effects, create an `assets/` folder in the project root and add:
- `eat.wav` - Sound when eating food
- `die.wav` - Sound when game ends

The game works perfectly without these files - you'll just see harmless error messages in the console.

## Technical Details

- **Language**: Rust
- **Game Engine**: ggez 0.9.3
- **Audio**: rodio 0.17
- **Grid Size**: 20x20 cells
- **Window Size**: 400x400 pixels

## Project Structure

```
src/
├── main.rs      - Entry point and window setup
├── game.rs      - Main game logic and event handling
├── snake.rs     - Snake entity (movement, collision, rendering)
├── food.rs      - Food entity (spawning, rendering)
└── audio.rs     - Sound effect system
```

## Gameplay Rules

1. Use arrow keys to control the snake
2. Eat red food to grow and increase your score
3. Avoid hitting the walls (gray borders)
4. Avoid hitting yourself
5. Game gets progressively harder as the snake grows longer
6. Press 'R' to restart when game is over

## Development

Built with debugging and improvements including:
- Fixed audio system threading issues
- Added wall collision detection
- Prevented food from spawning on snake body
- Added visual grid for better gameplay
- Improved error handling throughout

---

*Created by Michael Frazier*
