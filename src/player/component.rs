use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct PlayerAnimationTimer(Timer);

const PLAYER_SPRITE_WIDTH: f32 = 20.0;
const PLAYER_SPRITE_HEIGHT: f32 = 28.0;
const PLAYER_TILE_SIZE: Vec2 = Vec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT);
const PLAYER_SPRITE_OFFSET: Option<Vec2> = Some(Vec2::new(5.0, 4.0));
const MOVEMENT_SPEED: f32 = 0.5;

#[derive(Debug, Component)]
pub struct Player;

impl Player {
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) {
        let player_sprites: Handle<Image> = asset_server.load("player_sprites.png");
        let texture_atlas = TextureAtlas::from_grid(
            player_sprites,
            PLAYER_TILE_SIZE,
            3,
            4,
            None,
            PLAYER_SPRITE_OFFSET,
        );
        let texture_atlas_handle = texture_atlas_res.add(texture_atlas);
        let animation_timer = PlayerAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating));

        commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(1),
                    transform: Transform::from_scale(Vec3::new(2.0, 2.0, 0.0)),
                    ..default()
                },
                animation_timer,
            ))
            .insert(Player);
    }

    pub fn player_movement(
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

    pub fn animate_player_movement(
        time: Res<Time>,
        keyboard: Res<Input<KeyCode>>,
        mut query: Query<(
            &mut Player,
            &mut PlayerAnimationTimer,
            &mut TextureAtlasSprite,
        )>,
    ) {
        for (_player, mut timer, mut sprite) in &mut query {
            if keyboard.pressed(KeyCode::A) {
                timer.tick(time.delta());
                if timer.just_finished() {
                    if sprite.index < 5 {
                        sprite.index += 1;
                    } else {
                        sprite.index = 3;
                    }
                }
            }
            if keyboard.just_released(KeyCode::A) {
                sprite.index = 4
            }

            // Right
            if keyboard.pressed(KeyCode::D) {
                timer.tick(time.delta());
                if timer.just_finished() {
                    if sprite.index < 11 {
                        sprite.index += 1;
                    } else {
                        sprite.index = 9;
                    }
                }
            }
            if keyboard.just_released(KeyCode::D) {
                sprite.index = 10
            }

            // Down
            if keyboard.pressed(KeyCode::S) {
                timer.tick(time.delta());
                if timer.just_finished() {
                    if sprite.index < 2 {
                        sprite.index += 1;
                    } else {
                        sprite.index = 0;
                    }
                }
            }
            if keyboard.just_released(KeyCode::S) {
                sprite.index = 1;
            }

            // Up
            if keyboard.pressed(KeyCode::W) {
                timer.tick(time.delta());
                if timer.just_finished() {
                    if sprite.index < 8 {
                        sprite.index += 1;
                    } else {
                        sprite.index = 6
                    }
                }
            }
            if keyboard.just_released(KeyCode::W) {
                sprite.index = 7;
            }
        }
    }
}
