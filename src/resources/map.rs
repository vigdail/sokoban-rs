use amethyst::{core::math::Vector2, ecs::World};

use crate::entities::*;

#[derive(Clone)]
enum Tile {
    Floor,
    Wall,
    Box,
    BoxSpot,
    Player,
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn from_str(s: &str) -> Self {
        let tiles = s
            .trim()
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
        Self { tiles }
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

    pub fn width(&self) -> usize {
        self.tiles.get(0).unwrap_or(&vec![]).len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }
}
