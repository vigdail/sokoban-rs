use amethyst::{
    ecs::{Read, System, WriteExpect},
    input::{InputHandler, StringBindings},
};

use crate::resources::{InputQueue, MoveCommand};

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
