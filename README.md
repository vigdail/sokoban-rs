# Sokoban-RS

[![Build Status](https://travis-ci.com/vigdail/sokoban-rs.svg?branch=master)](https://travis-ci.com/vigdail/sokoban-rs)

This is a sokoban-like game written in rust with [Amethyst](https://github.com/amethyst/amethyst) game engine.

The project was inspired by this [tutorial](https://sokoban.iolivia.me/c02-05-gameplay.html), it uses [ggez](https://github.com/ggez/ggez) engine, but also utilize [spesc](https://github.com/amethyst/specs) ecs library as Amethyst does.

## Configure Cargo.toml

The default render target feature is set to `vulkan` for Linux/Windows users.

```toml
# Cargo.toml

[features]
default = ["vulkan"]
metal = ["amethyst/metal"]
vulkan = ["amethyst/vulkan"]
```

If you are on MacOs, you will have to set that default to `metal`:

```toml
# Cargo.toml

[features]
default = ["metal"]
# ...
```

## Running the game

```bash
# Clone the repo
git clone https://github.com/vigdail/sokoban-rs.git
cd sokoban-rs

# Run
cargo run —-release
```