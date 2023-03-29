use crate::startup::StartupState;
use bevy::prelude::*;

#[derive(Debug)]
pub struct TextboxPlugin;

impl Plugin for TextboxPlugin {
    fn build(&self, app: &mut App) {
        let dummy = vec![
            "At the start there's creation".to_string(),
            "But where does the creation comes from?".to_string(),
        ];
        let textbox = TextBox::new(true, dummy);

        app.insert_resource(textbox)
            .add_startup_system(TextBox::spawn)
            .add_system(TextBox::detect_change);
        // TODO: Activate this code once functionality done
        // .add_system(TextBox::spawn.in_schedule(OnEnter(StartupState::InGame)));
    }
}

const TEXTBOX_TILE_SIZE: Vec2 = Vec2::new(255., 48.);

#[derive(Debug, Component)]
pub struct TextBoxValue;

#[derive(Debug, Resource)]
pub struct TextBox {
    spawn: bool,
    index: usize,
    texts: Vec<String>,
}

impl TextBox {
    fn new(spawn: bool, texts: Vec<String>) -> TextBox {
        TextBox {
            spawn,
            index: 0,
            texts,
        }
    }

    fn bundle(
        asset_server: &Res<AssetServer>,
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

    fn spawn(
        textbox_res: Res<TextBox>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas_res: ResMut<Assets<TextureAtlas>>,
        window_query: Query<&Window>,
    ) {
        if textbox_res.spawn {
            let window = window_query.single();

            let textbox_name = Name::new("Textbox");
            let textbox = TextBox::bundle(&asset_server, texture_atlas_res, window);
            let value = Self::textbox_value(&asset_server, &textbox_res.texts[0], window);

            commands.spawn((textbox_name, textbox));
            // let a = commands.spawn(value).insert(TextBoxValue).id();
            // println!("{:?}", a);
        }
    }

    fn textbox_value(
        asset_server: &Res<AssetServer>,
        text_value: &str,
        window: &Window,
    ) -> TextBundle {
        let font: Handle<Font> = asset_server.load("fonts/april-easter.ttf");

        let style = Style {
            margin: UiRect {
                top: Val::Px(window.height() * 0.75),
                left: Val::Px(window.width() / 5.),
                ..Default::default()
            },
            ..Default::default()
        };
        TextBundle::from_section(
            text_value,
            TextStyle {
                font: font,
                font_size: 50.0,
                color: Color::BLACK,
            },
        )
        .with_style(style)
    }

    fn detect_change(
        keyboard: Res<Input<KeyCode>>,
        textbox_res: Res<TextBox>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas_res: ResMut<Assets<TextureAtlas>>,
        window_query: Query<&Window>,
        // textbox_value: Query<Entity, With<TextBox>>,
    ) {
        let pressed_z_or_x = keyboard.pressed(KeyCode::Z) | keyboard.pressed(KeyCode::X);
        if pressed_z_or_x & textbox_res.spawn {
            // let window = window_query.single();

            // let textbox_name = Name::new("Textbox");
            // let textbox = TextBox::bundle(&asset_server, texture_atlas_res, window);
            // let value = Self::textbox_value(&asset_server, &textbox_res.texts[0], window);

            // commands.spawn((textbox_name, textbox));
            // commands.spawn((value, TextBoxValue));
            // let text_val = textbox_value.single();
            // commands.get_entity(text_val).unwrap().despawn();
        }
    }
}
