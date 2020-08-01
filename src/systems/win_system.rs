use amethyst::ecs::{Join, ReadStorage, System, WriteExpect};
use std::collections::HashMap;

use crate::{
    components::{Box, BoxSpot, TilePosition},
    resources::{GameState, Gameplay},
};

pub struct WinSystem;

impl<'a> System<'a> for WinSystem {
    type SystemData = (
        ReadStorage<'a, TilePosition>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        WriteExpect<'a, Gameplay>,
    );

    fn run(&mut self, (positions, boxes, spots, mut state): Self::SystemData) {
        let box_map: HashMap<(i32, i32), &Box> = (&positions, &boxes)
            .join()
            .map(|(p, b)| ((p.x, p.y), b))
            .collect();

        for (_, position) in (&spots, &positions).join() {
            if !box_map.contains_key(&(position.x, position.y)) {
                state.state = GameState::Playing;
                return;
            }
        }

        state.state = GameState::Win;
    }
}
