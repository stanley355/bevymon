use super::animation::PlayerAnimationTimer;
use bevy::prelude::*;

const PLAYER_SPRITE_WIDTH: f32 = 20.0;
const PLAYER_SPRITE_HEIGHT: f32 = 28.0;
const PLAYER_TILE_SIZE: Vec2 = Vec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT);
const PLAYER_SPRITE_OFFSET: Option<Vec2> = Some(Vec2::new(5.0, 4.0));
const PLAYER_Z_INDEX: f32 = 2.0;
const MOVEMENT_SPEED: f32 = 0.5;

#[derive(Debug, Component, Reflect)]
pub struct Player;

impl Player {
    fn get_player_transform() -> Transform {
        Transform {
            translation: Vec3::new(0.0, 0.0, PLAYER_Z_INDEX),
            scale: Vec3::new(2.0, 2.0, 0.0),
            ..Default::default()
        }
    }

    fn get_player_texture_atlas(
        asset_server: Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) -> Handle<TextureAtlas> {
        let player_sprites: Handle<Image> = asset_server.load("player_sprites.png");
        let texture_atlas = TextureAtlas::from_grid(
            player_sprites,
            PLAYER_TILE_SIZE,
            3,
            4,
            None,
            PLAYER_SPRITE_OFFSET,
        );

        return texture_atlas_res.add(texture_atlas);
    }

    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) {
        let texture_atlas_handle = Self::get_player_texture_atlas(asset_server, texture_atlas_res);
        let animation_timer = PlayerAnimationTimer::new();
        let planet = Name::new("Player");
        commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(1),
                    transform: Self::get_player_transform(),
                    ..default()
                },
                animation_timer,
                planet,
            ))
            .insert(Player);

            
    }

    pub fn move_player(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(&mut Player, &mut Transform)>,
    ) {
        for (_player, mut transform) in &mut query {
            if keyboard_input.pressed(KeyCode::A) {
                transform.translation.x -= MOVEMENT_SPEED;
            }
            if keyboard_input.pressed(KeyCode::D) {
                transform.translation.x += MOVEMENT_SPEED;
            }
            if keyboard_input.pressed(KeyCode::S) {
                transform.translation.y -= MOVEMENT_SPEED;
            }
            if keyboard_input.pressed(KeyCode::W) {
                transform.translation.y += MOVEMENT_SPEED;
            }
        }
    }
}
