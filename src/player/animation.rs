use super::component::Player;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct PlayerAnimationTimer(Timer);

impl PlayerAnimationTimer {
    pub fn new() -> Self {
        PlayerAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

#[derive(Debug)]
pub struct PlayerAnimation {}

impl PlayerAnimation {
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
