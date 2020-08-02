use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::{components::TilePosition, BLOCK_SIZE};

pub struct CoordSystem;

impl<'a> System<'a> for CoordSystem {
    type SystemData = (ReadStorage<'a, TilePosition>, WriteStorage<'a, Transform>);
    fn run(&mut self, (positions, mut transforms): Self::SystemData) {
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
