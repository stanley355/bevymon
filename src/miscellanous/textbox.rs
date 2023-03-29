use crate::startup::StartupState;
use bevy::prelude::*;

#[derive(Debug)]
pub struct TextboxPlugin;

impl Plugin for TextboxPlugin {
    fn build(&self, app: &mut App) {
        let textbox = TextBox::new(true, vec!["".to_string()]);

        app.insert_resource(textbox)
            .add_startup_system(TextBox::spawn);
        // TODO: Activate this code once functionality done
        //     .add_system(TextBox::spawn.in_schedule(OnEnter(StartupState::InGame)));
    }
}

const TEXTBOX_TILE_SIZE: Vec2 = Vec2::new(255., 48.);

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
        let transform = Transform {
            translation: Vec3::new(0., window.height() / -3., 5.),
            scale: Vec3::new(3., 3., 0.),
            ..Default::default()
        };

        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(2),
            transform,
            ..default()
        }
    }

    pub fn spawn(
        mut textbox_res: ResMut<TextBox>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas_res: ResMut<Assets<TextureAtlas>>,
        window_query: Query<&Window>,
    ) {
        if textbox_res.spawn {
            let window = window_query.single();

            let textbox_name = Name::new("Textbox");
            let textbox = TextBox::bundle(asset_server, texture_atlas_res, window);
            commands.spawn((textbox_name, textbox));
        }
    }
}
