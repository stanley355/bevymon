use bevy::prelude::*;

#[derive(Debug)]
pub struct TextboxPlugin;

impl Plugin for TextboxPlugin {
    fn build(&self, app: &mut App) {
        let textbox = TextBox::new(false, vec!["".to_string()]);
        app.insert_resource(textbox);
    }
}

const TEXTBOX_TILE_SIZE: Vec2 = Vec2::new(255., 50.);

#[derive(Debug, Resource)]
pub struct TextBox {
    spawn: bool,
    texts: Vec<String>,
}

impl TextBox {
    pub fn new(spawn: bool, texts: Vec<String>) -> TextBox {
        TextBox { spawn, texts }
    }

    pub fn bundle(
        asset_server: Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
        window: &Window,
    ) -> SpriteSheetBundle {
        let textbox_sprites: Handle<Image> = asset_server.load("images/text_boxes.png");

        let texture_atlas =
            TextureAtlas::from_grid(textbox_sprites, TEXTBOX_TILE_SIZE, 2, 10, None, None);
        let texture_atlas_handle = texture_atlas_res.add(texture_atlas);

        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            ..default()
        }
    }
}
