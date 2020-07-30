use amethyst::{
    assets::{AssetStorage, Loader},
    core::{math::Vector2, transform::Transform},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

use log::info;

use crate::{
    components::*,
    entities::create_box,
    resources::{Map, SpriteAtlases},
};

pub struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        register_components(world);
        init_camera(world, &dimensions);

        load_sprites(world);

        create_map(world, Vector2::new(0.0, dimensions.height()));
        create_box(world, Vector2::new(5, 1));
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }
        }

        Trans::None
    }
}

fn register_components(world: &mut World) {
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<TilePosition>();
    world.register::<Box>();
    world.register::<BoxSpot>();
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, -dimensions.height() * 0.5, 10.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World) {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/atlas.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/atlas.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    world.insert(SpriteAtlases { all: sheet_handle });
}

fn create_map(world: &mut World, position: Vector2<f32>) {
    let s = "
    ##########
    #........#
    #........#
    #........#
    #........#
    ##########
    ";
    let map = Map::from_str(position, s);
    map.build(world);
    world.insert(map);
}
