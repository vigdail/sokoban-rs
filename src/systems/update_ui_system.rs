use amethyst::{
    ecs::{ReadExpect, System, WriteStorage},
    ui::UiText,
};

use crate::resources::{GameUI, Gameplay};

pub struct UpdateUISystem;

impl<'a> System<'a> for UpdateUISystem {
    type SystemData = (
        ReadExpect<'a, Gameplay>,
        ReadExpect<'a, GameUI>,
        WriteStorage<'a, UiText>,
    );

    fn run(&mut self, (gameplay, ui, mut texts): Self::SystemData) {
        if let Some(entity) = ui.texts.get("state") {
            if let Some(text) = texts.get_mut(*entity) {
                text.text = gameplay.state.to_string();
            }
        }
        if let Some(entity) = ui.texts.get("steps") {
            if let Some(text) = texts.get_mut(*entity) {
                text.text = gameplay.steps.to_string();
            }
        }
    }
}
