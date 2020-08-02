use amethyst::{
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
    ui::{Anchor, UiText, UiTransform},
    window::ScreenDimensions,
};
use log::info;

use crate::{
    resources::{AssetManager, GameState, GameUI, Gameplay, Map},
    states::WinState,
};

pub struct GameplayState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("Gameplay state start");
        reset_game(data.world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            if is_key_down(&event, VirtualKeyCode::R) {
                return Trans::Switch(Box::new(GameplayState));
            }

            let game = data.world.read_resource::<Gameplay>();
            if game.state == GameState::Win {
                return Trans::Switch(Box::new(WinState::default()));
            }
        }

        Trans::None
    }
}

fn reset_game(world: &mut World) {
    {
        let mut game_state = world.write_resource::<Gameplay>();
        game_state.steps = 0;
        game_state.state = GameState::Playing;
    }
    world.delete_all();
    init_camera(world);
    create_ui(world);
    load_map(world);
}

fn init_camera(world: &mut World) {
    let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, -dimensions.height() * 0.5, 10.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_map(world: &mut World) {
    let level_index = world.read_resource::<Gameplay>().current_level_index + 1;

    // @TODO: Custom format for Map loading
    let s = std::fs::read_to_string(format!("./resources/maps/level{}.txt", level_index))
        .unwrap_or_else(|_| panic!("Map level{} exist", level_index));
    let map = Map::from_str(&s);
    map.build(world);
    world.insert(map);
}

fn create_ui(world: &mut World) {
    let font = {
        let manager = world.read_resource::<AssetManager>();
        manager.font.clone().unwrap()
    };
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
        .with(UiText::new(font, "0".to_string(), [1., 1., 1., 1.], 50.))
        .build();

    let mut game_ui = GameUI::default();
    game_ui.insert_text("state", state_text);
    game_ui.insert_text("steps", steps_text);
    world.insert(game_ui);
}
