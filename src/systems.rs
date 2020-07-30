use amethyst::{
    core::transform::Transform,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

use crate::{
    components::{Player, TilePosition},
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
                -((position.y * BLOCK_SIZE as i32) as f32 - BLOCK_SIZE as f32 / 2.0),
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
        WriteExpect<'a, InputQueue>,
    );
    fn run(&mut self, (players, mut positions, mut input_queue): Self::SystemData) {
        for (_player, position) in (&players, &mut positions).join() {
            if let Some(command) = input_queue.commands.pop() {
                match command {
                    MoveCommand::Up => position.y -= 1,
                    MoveCommand::Down => position.y += 1,
                    MoveCommand::Left => position.x -= 1,
                    MoveCommand::Right => position.x += 1,
                }
            }
        }
    }
}
