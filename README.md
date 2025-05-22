# Vibegame

## Overview
Vibegame is a simple 2D platformer game inspired by classic titles like Super Mario. The game features a colorful character represented by a rectangle, and players can navigate through various levels while avoiding obstacles and collecting items.

## Project Structure
```
vibegame
├── src
│   ├── main.rs          # Entry point of the application
│   ├── game.rs          # Main game logic and state management
│   ├── player.rs        # Player character definition and behavior
│   ├── camera.rs        # Camera management for game view
│   ├── resources.rs     # Resource loading and management
│   └── scenes
│       ├── mod.rs       # Module for different game scenes
│       └── level.rs     # Level definition and management
├── assets
│   ├── textures
│   │   └── player.rs    # Texture data for the player character
│   └── sounds
│       └── jump.rs      # Sound data for jump action
├── Cargo.toml           # Rust project configuration and dependencies
└── README.md            # Project documentation
```

## Setup Instructions
1. Ensure you have Rust and Cargo installed on your machine.
2. Clone the repository:
   ```
   git clone <repository-url>
   ```
3. Navigate to the project directory:
   ```
   cd vibegame
   ```
4. Install the required dependencies:
   ```
   cargo build
   ```
5. Run the game:
   ```
   cargo run
   ```

## Gameplay
- Control the character using the arrow keys or WASD.
- Jump over obstacles and collect items to score points.
- Navigate through different levels, each with unique challenges.

## Dependencies
- raylib: A simple and easy-to-use library for 2D game development in Rust.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.