use bevy::prelude::*;
use bevy_kira_audio::prelude::{*, Audio};

const CHATBOX_SPRITE_TILE_SIZE: Vec2 = Vec2::new(255., 48.);

#[derive(Debug, Component)]
pub struct ChatBoxSprite;

impl ChatBoxSprite {
    pub fn new(
        asset_server: &Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
        window: &Window,
    ) -> SpriteSheetBundle {
        let textbox_sprites: Handle<Image> = asset_server.load("images/chat_boxes.png");

        let texture_atlas =
            TextureAtlas::from_grid(textbox_sprites, CHATBOX_SPRITE_TILE_SIZE, 2, 10, None, None);
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
}

#[derive(Debug, Component)]
pub struct ChatBoxText;

impl ChatBoxText {
    pub fn new(asset_server: &Res<AssetServer>, window: &Window, text_value: &str) -> TextBundle {
        let font: Handle<Font> = asset_server.load("fonts/poke-text.ttf");

        let style = Style {
            margin: UiRect {
                top: Val::Px(window.height() * 0.75),
                left: Val::Px(window.width() / 5.),
                ..Default::default()
            },
            max_size: Size::width(Val::Px(window.width() / 1.75)),
            ..Default::default()
        };
        TextBundle::from_section(
            text_value,
            TextStyle {
                font: font,
                font_size: 36.0,
                color: Color::BLACK,
            },
        )
        .with_style(style)
    }

    pub fn play_btn_audio(asset_server: &Res<AssetServer>, audio: Res<Audio>) {
        audio.play(asset_server.load("audio/button_click.ogg"));
    }
}
