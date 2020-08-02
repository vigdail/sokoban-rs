use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    ecs::{World, WorldExt},
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{FontAsset, TtfFormat},
};
use std::collections::HashMap;

pub enum AssetType {
    Sprite,
    Font,
}

pub struct AssetManager {
    pub progress: ProgressCounter,
    pub sprites: HashMap<String, Handle<SpriteSheet>>,
    pub font: Option<Handle<FontAsset>>,
}

impl AssetManager {
    pub fn insert(&mut self, world: &mut World, name: &str, asset_type: AssetType) {
        match asset_type {
            AssetType::Sprite => {
                let handle = self.load_sprite_sheet(world, name);
                self.sprites.insert(name.to_string(), handle);
            }
            AssetType::Font => {
                let handle = self.load_font(world, name);
                self.font = Some(handle);
            }
        }
    }

    fn load_sprite_sheet(&mut self, world: &World, name: &str) -> Handle<SpriteSheet> {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                format!("sprites/{}.png", name),
                ImageFormat::default(),
                &mut self.progress,
                &texture_storage,
            )
        };

        let sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                format!("sprites/{}.ron", name),
                SpriteSheetFormat(texture_handle),
                &mut self.progress,
                &sheet_storage,
            )
        };

        sheet_handle
    }

    fn load_font(&mut self, world: &World, name: &str) -> Handle<FontAsset> {
        world.read_resource::<Loader>().load(
            format!("fonts/{}.ttf", name),
            TtfFormat,
            &mut self.progress,
            &world.read_resource(),
        )
    }
}
