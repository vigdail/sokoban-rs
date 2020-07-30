use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::{Builder, World, WorldExt},
    renderer::SpriteRender,
};

use crate::{
    components::{Box, TilePosition, Wall},
    resources::SpriteAtlases,
};

pub fn create_wall(world: &mut World, position: Vector2<u32>) {
    let handle = {
        let atlases = world.read_resource::<SpriteAtlases>();
        atlases.all.clone()
    };

    world
        .create_entity()
        .with(Wall)
        .with(Transform::default())
        .with(TilePosition {
            x: position.x,
            y: position.y,
        })
        .with(SpriteRender {
            sprite_sheet: handle,
            sprite_number: 1,
        })
        .build();
}

pub fn create_box(world: &mut World, position: Vector2<u32>) {
    let handle = {
        let atlases = world.read_resource::<SpriteAtlases>();
        atlases.all.clone()
    };

    world
        .create_entity()
        .with(Box)
        .with(Transform::default())
        .with(TilePosition {
            x: position.x,
            y: position.y,
        })
        .with(SpriteRender {
            sprite_sheet: handle,
            sprite_number: 2,
        })
        .build();
}

pub fn create_floor(world: &mut World, position: Vector2<u32>) {
    let handle = {
        let atlases = world.read_resource::<SpriteAtlases>();
        atlases.all.clone()
    };

    world
        .create_entity()
        .with(Box)
        .with(Transform::default())
        .with(TilePosition {
            x: position.x,
            y: position.y,
        })
        .with(SpriteRender {
            sprite_sheet: handle,
            sprite_number: 0,
        })
        .build();
}
