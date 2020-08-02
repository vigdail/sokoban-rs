use amethyst::ecs::Entity;
use std::collections::HashMap;
use std::fmt::Display;

pub mod asset_manager;
pub mod map;

pub use asset_manager::{AssetManager, AssetType};
pub use map::Map;

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

#[derive(Default)]
pub struct GameUI {
    pub texts: HashMap<String, Entity>,
}

impl GameUI {
    pub fn insert_text(&mut self, name: &str, text: Entity) {
        self.texts.insert(name.to_string(), text);
    }
}
