use amethyst::{assets::Handle, core::math::Vector2, ecs::World, renderer::SpriteSheet};

use crate::entities::{create_floor, create_wall};

pub struct SpriteAtlases {
    pub all: Handle<SpriteSheet>,
}

#[derive(Clone)]
pub enum Tile {
    Floor,
    Wall,
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
                line.chars()
                    .filter_map(|c| match c {
                        '#' => Some(Tile::Wall),
                        '.' => Some(Tile::Floor),
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
                let position = Vector2::new(x as u32, y as u32);
                match tile {
                    Tile::Floor => create_floor(world, position),
                    Tile::Wall => create_wall(world, position),
                }
            }
        }
    }
}
