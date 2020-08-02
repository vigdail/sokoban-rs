use amethyst::{assets::Handle, ecs::Entity, renderer::SpriteSheet};
use std::fmt::Display;

pub mod map;

pub use map::Map;

pub struct SpriteAtlases {
    pub all: Handle<SpriteSheet>,
}

#[derive(PartialEq, Clone)]
pub enum MoveCommand {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct InputQueue {
    pub commands: Vec<MoveCommand>,
    pub last_key: Option<MoveCommand>,
}

pub enum GameState {
    Playing,
    Win,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            GameState::Playing => "Play",
            GameState::Win => "Win",
        })
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub steps: u32,
    pub state: GameState,
}

pub struct GameUI {
    pub state_text: Entity,
    pub steps_text: Entity,
}
