use amethyst::{
    core::Time,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
};
use log::info;

use crate::{resources::Gameplay, states::GameplayState};

pub struct WinState {
    timer: f32,
}

impl Default for WinState {
    fn default() -> Self {
        WinState { timer: 3.0 }
    }
}

impl SimpleState for WinState {
    fn on_start(&mut self, _: StateData<GameData>) {
        info!("Win state");
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        let time = data.world.read_resource::<Time>();
        self.timer -= time.delta_seconds();
        if self.timer <= 0.0 {
            let mut game = data.world.write_resource::<Gameplay>();
            game.next_level();
            return Trans::Switch(Box::new(GameplayState));
        }

        Trans::None
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        Trans::None
    }
}
