use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::{Builder, World, WorldExt},
    renderer::SpriteRender,
};

use crate::{
    components::{Box, BoxSpot, Immovable, Movable, Player, TilePosition, Wall},
    resources::AssetManager,
};

pub fn create_wall(world: &mut World, position: Vector2<i32>) {
    let handle = {
        let atlases = world.read_resource::<AssetManager>();
        atlases.sprites.get("atlas").unwrap().clone()
    };

    world
        .create_entity()
        .with(Wall)
        .with(Transform::default())
        .with(TilePosition {
            x: position.x,
            y: position.y,
            z: 1,
        })
        .with(Immovable)
        .with(SpriteRender {
            sprite_sheet: handle,
            sprite_number: 8,
        })
        .build();
}

pub fn create_box(world: &mut World, position: Vector2<i32>) {
    let handle = {
        let atlases = world.read_resource::<AssetManager>();
        atlases.sprites.get("atlas").unwrap().clone()
    };

    world
        .create_entity()
        .with(Box)
        .with(Transform::default())
        .with(TilePosition {
            x: position.x,
            y: position.y,
            z: 2,
        })
        .with(Movable)
        .with(SpriteRender {
            sprite_sheet: handle,
            sprite_number: 0,
        })
        .build();
}

pub fn create_box_spot(world: &mut World, position: Vector2<i32>) {
    let handle = {
        let atlases = world.read_resource::<AssetManager>();
        atlases.sprites.get("atlas").unwrap().clone()
    };

    world
        .create_entity()
        .with(BoxSpot)
        .with(Transform::default())
        .with(TilePosition {
            x: position.x,
            y: position.y,
            z: 1,
        })
        .with(SpriteRender {
            sprite_sheet: handle,
            sprite_number: 6,
        })
        .build();
}

pub fn create_floor(world: &mut World, position: Vector2<i32>) {
    let handle = {
        let atlases = world.read_resource::<AssetManager>();
        atlases.sprites.get("atlas").unwrap().clone()
    };

    world
        .create_entity()
        .with(Transform::default())
        .with(TilePosition {
            x: position.x,
            y: position.y,
            z: 0,
        })
        .with(SpriteRender {
            sprite_sheet: handle,
            sprite_number: 1,
        })
        .build();
}

pub fn create_player(world: &mut World, position: Vector2<i32>) {
    let handle = {
        let atlases = world.read_resource::<AssetManager>();
        atlases.sprites.get("atlas").unwrap().clone()
    };

    world
        .create_entity()
        .with(Player)
        .with(Transform::default())
        .with(TilePosition {
            x: position.x,
            y: position.y,
            z: 5,
        })
        .with(Movable)
        .with(SpriteRender {
            sprite_sheet: handle,
            sprite_number: 2,
        })
        .build();
}
