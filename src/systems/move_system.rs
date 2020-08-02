use amethyst::ecs::{
    world::Index, Entities, Join, Read, ReadStorage, System, WriteExpect, WriteStorage,
};
use std::collections::HashMap;

use crate::{
    components::{Immovable, Movable, Player, TilePosition},
    resources::{Gameplay, InputQueue, Map, MoveCommand},
};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, TilePosition>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
        WriteExpect<'a, InputQueue>,
        Option<Read<'a, Map>>,
        WriteExpect<'a, Gameplay>,
        Entities<'a>,
    );
    fn run(
        &mut self,
        (
            players,
            mut positions,
            movables,
            immovables,
            mut input_queue,
            map,
            mut gameplay,
            entities,
        ): Self::SystemData,
    ) {
        if map.is_none() {
            return;
        }
        let map = map.unwrap();

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

            if to_move.len() > 0 {
                gameplay.steps += 1;
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
