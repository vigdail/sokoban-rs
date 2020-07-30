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
    pub x: u32,
    pub y: u32,
    pub z: i32,
}
