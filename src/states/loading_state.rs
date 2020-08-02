use amethyst::prelude::*;

use log::info;

use crate::{
    components,
    resources::{AssetManager, AssetType, GameUI, Gameplay, InputQueue},
    states::GameplayState,
};

pub struct LoadingState;

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("Loading...");
        let world = data.world;

        register_components(world);
        insert_resources(world);

        load_sprites(world);

        load_font(world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        let asset_manager = data.world.read_resource::<AssetManager>();
        let counter = &asset_manager.progress;
        if counter.is_complete() {
            return Trans::Switch(Box::new(GameplayState));
        }

        Trans::None
    }
}

fn register_components(world: &mut World) {
    world.register::<components::Wall>();
    world.register::<components::Player>();
    world.register::<components::TilePosition>();
    world.register::<components::Box>();
    world.register::<components::BoxSpot>();
    world.register::<components::Movable>();
    world.register::<components::Immovable>();
}

fn insert_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(GameUI::default());
    world.insert(AssetManager::default());
}

fn load_sprites(world: &mut World) {
    let mut manager = world.write_resource::<AssetManager>();
    manager.insert(world, "atlas", AssetType::Sprite);
}

fn load_font(world: &mut World) {
    let mut manager = world.write_resource::<AssetManager>();
    manager.insert(world, "square", AssetType::Font);
}
