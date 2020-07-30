use amethyst::{assets::Handle, core::math::Vector2, ecs::World, renderer::SpriteSheet};

use crate::entities::*;

pub struct SpriteAtlases {
    pub all: Handle<SpriteSheet>,
}

#[derive(Clone)]
pub enum Tile {
    Floor,
    Wall,
    Box,
    BoxSpot,
    Player,
}

pub struct Map {
    pub position: Vector2<f32>,
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn from_str(position: Vector2<f32>, s: &str) -> Self {
        let tiles = s
            .split("\n")
            .map(|line| {
                line.trim()
                    .chars()
                    .filter_map(|c| match c {
                        '#' => Some(Tile::Wall),
                        '.' => Some(Tile::Floor),
                        '*' => Some(Tile::BoxSpot),
                        'B' => Some(Tile::Box),
                        '@' => Some(Tile::Player),
                        _ => None,
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Tile>>>();
        Self { position, tiles }
    }

    pub fn build(&self, world: &mut World) {
        for (y, line) in self.tiles.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let position = Vector2::new(x as i32, y as i32);
                match tile {
                    Tile::Floor => create_floor(world, position),
                    Tile::Wall => create_wall(world, position),
                    Tile::Box => {
                        create_floor(world, position);
                        create_box(world, position);
                    }
                    Tile::BoxSpot => {
                        create_floor(world, position);
                        create_box_spot(world, position);
                    }
                    Tile::Player => {
                        create_floor(world, position);
                        create_player(world, position);
                    }
                }
            }
        }
    }
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
