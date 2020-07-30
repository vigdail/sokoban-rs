use amethyst::ecs::{Component, DenseVecStorage, NullStorage};

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Wall;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Box;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct BoxSpot;

#[derive(Component)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;
