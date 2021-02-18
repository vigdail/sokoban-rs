use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use crate::systems::{CoordSystem, InputSystem, MoveSystem, UpdateUISystem, WinSystem};

mod components;
mod entities;
mod resources;
mod states;
mod systems;

pub const BLOCK_SIZE: u32 = 64;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(resources.join("input.ron"))?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with(InputSystem, "input_mapping_system", &[])
        .with(MoveSystem, "move_system", &[])
        .with(CoordSystem, "coord_system", &[])
        .with(WinSystem, "win_system", &[])
        .with(UpdateUISystem, "update_ui", &["win_system"])
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::new(resources, states::LoadingState, game_data)?;
    game.run();

    Ok(())
}
