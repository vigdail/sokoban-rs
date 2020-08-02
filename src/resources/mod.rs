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

#[derive(PartialEq)]
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
    pub current_level_index: usize,
    pub state: GameState,
}

impl Gameplay {
    pub fn next_level(&mut self) {
        if self.current_level_index == 1 {
            self.current_level_index = 0;
        } else {
            self.current_level_index = 1;
        }
    }
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
