use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    window::ScreenDimensions,
};

use crate::{components::TilePosition, resources::Map, BLOCK_SIZE};

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
            transform.set_translation_x((position.x * BLOCK_SIZE + BLOCK_SIZE / 2) as f32);
            transform
                .set_translation_y(-((position.y * BLOCK_SIZE) as f32 - BLOCK_SIZE as f32 / 2.0));
            transform.set_translation_z(position.z as f32);
        }
    }
}
