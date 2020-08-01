use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use log::info;

use crate::{
    components,
    resources::{GameUI, Gameplay, InputQueue, SpriteAtlases},
    states::GameplayState,
};

pub struct LoadingState {
    progress: Option<ProgressCounter>,
}

impl Default for LoadingState {
    fn default() -> Self {
        Self { progress: None }
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("Loading...");
        let world = data.world;

        register_components(world);
        insert_resources(world);

        let mut progress_counter = ProgressCounter::new();
        progress_counter = load_sprites(world, progress_counter);

        create_ui(world, &mut progress_counter);

        self.progress = Some(progress_counter);
    }

    fn update(&mut self, _: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(ref counter) = self.progress.as_ref() {
            if counter.is_complete() {
                return Trans::Switch(Box::new(GameplayState));
            }
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
}

fn load_sprites(world: &mut World, mut progress_counter: ProgressCounter) -> ProgressCounter {
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
            &mut progress_counter,
            &sheet_storage,
        )
    };

    world.insert(SpriteAtlases { all: sheet_handle });

    progress_counter
}

fn create_ui(world: &mut World, progress_counter: &mut ProgressCounter) {
    let font = world.read_resource::<Loader>().load(
        "fonts/square.ttf",
        TtfFormat,
        progress_counter,
        &world.read_resource(),
    );

    let state_text_transform = UiTransform::new(
        "state_text".to_string(),
        Anchor::TopRight,
        Anchor::TopRight,
        0.0,
        -60.0,
        1.,
        170.,
        50.,
    );

    let state_text = world
        .create_entity()
        .with(state_text_transform)
        .with(UiText::new(
            font.clone(),
            "Play".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    let steps_text_transform = UiTransform::new(
        "steps_text".to_string(),
        Anchor::TopRight,
        Anchor::TopRight,
        0.0,
        0.0,
        1.,
        170.,
        50.,
    );

    let steps_text = world
        .create_entity()
        .with(steps_text_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    world.insert(GameUI {
        state_text,
        steps_text,
    });
}
