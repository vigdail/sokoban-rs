use amethyst::{
    core::transform::Transform,
    ecs::{
        world::Index, Entities, Join, Read, ReadExpect, ReadStorage, System, WriteExpect,
        WriteStorage,
    },
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};
use std::collections::HashMap;

use crate::{
    components::{Immovable, Movable, Player, TilePosition},
    resources::{InputQueue, Map, MoveCommand},
    BLOCK_SIZE,
};

pub struct CoordSystem;

impl<'a> System<'a> for CoordSystem {
    type SystemData = (
        ReadStorage<'a, TilePosition>,
        WriteStorage<'a, Transform>,
        ReadExpect<'a, Map>,
        ReadExpect<'a, ScreenDimensions>,
    );
    fn run(&mut self, (positions, mut transforms, _map, _dimensions): Self::SystemData) {
        for (position, transform) in (&positions, &mut transforms).join() {
            transform
                .set_translation_x((position.x * BLOCK_SIZE as i32 + BLOCK_SIZE as i32 / 2) as f32);
            transform.set_translation_y(
                -((position.y * BLOCK_SIZE as i32) as f32 + BLOCK_SIZE as f32 / 2.0),
            );
            transform.set_translation_z(position.z as f32);
        }
    }
}

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Read<'a, InputHandler<StringBindings>>,
        WriteExpect<'a, InputQueue>,
    );
    fn run(&mut self, (input, mut queue): Self::SystemData) {
        // @TODO: Refactor this in some way
        let command = if Some(true) == input.action_is_down("up") {
            Some(MoveCommand::Up)
        } else if Some(true) == input.action_is_down("down") {
            Some(MoveCommand::Down)
        } else if Some(true) == input.action_is_down("left") {
            Some(MoveCommand::Left)
        } else if Some(true) == input.action_is_down("right") {
            Some(MoveCommand::Right)
        } else {
            None
        };
        if command != queue.last_key && command.is_some() {
            queue.commands.push(command.clone().unwrap());
        }
        queue.last_key = command.clone();
    }
}

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, TilePosition>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
        WriteExpect<'a, InputQueue>,
        ReadExpect<'a, Map>,
        Entities<'a>,
    );
    fn run(
        &mut self,
        (players, mut positions, movables, immovables, mut input_queue, map, entities): Self::SystemData,
    ) {
        let mut to_move = Vec::new();
        if let Some(command) = input_queue.commands.pop() {
            for (_player, position) in (&players, &positions).join() {
                let mov: HashMap<(i32, i32), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|(e, _, p)| ((p.x, p.y), e.id()))
                    .collect();
                let immov: HashMap<(i32, i32), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|(e, _, p)| ((p.x, p.y), e.id()))
                    .collect();

                let (start, end, is_x) = match command {
                    MoveCommand::Up => (position.y, 0, false),
                    MoveCommand::Down => (position.y, map.height() as i32, false),
                    MoveCommand::Left => (position.x, 0, true),
                    MoveCommand::Right => (position.x, map.width() as i32, true),
                };

                let range = if start < end {
                    (start..end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for p in range {
                    let pos = if is_x {
                        (p, position.y)
                    } else {
                        (position.x, p)
                    };

                    match mov.get(&pos) {
                        Some(id) => to_move.push((command.clone(), id.clone())),
                        None => match immov.get(&pos) {
                            Some(_) => to_move.clear(),
                            None => break,
                        },
                    }
                }
            }

            for (key, id) in to_move {
                let position = positions.get_mut(entities.entity(id));
                if let Some(position) = position {
                    match key {
                        MoveCommand::Up => position.y -= 1,
                        MoveCommand::Down => position.y += 1,
                        MoveCommand::Left => position.x -= 1,
                        MoveCommand::Right => position.x += 1,
                    }
                }
            }
        }
    }
}
