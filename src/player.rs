use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

const BASE_X: f32 = 20.0;
const BASE_Y: f32 = 27.0;

#[derive(Debug, Component)]
pub struct Player;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
) {
    let player_sprites: Handle<Image> = asset_server.load("player_sprites.png");
    let texture_atlas = TextureAtlas::from_grid(
        player_sprites,
        Vec2::new(BASE_X, BASE_Y),
        3,
        4,
        None,
        Some(Vec2::new(5.0, 4.0)),
    );
    let texture_atlas_handle = texture_atlas_res.add(texture_atlas);

    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_scale(Vec3::new(3.0, 3.0, 0.0)),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(Player);
}

pub fn move_player(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
) {
    // println!("hihiih");
    for (indices, mut timer, mut sprite, atlas) in &mut query {
        // sprite.custom_size = Some(Vec2::new(10.0, 21.0));
        // sprite.index = 18;
        // println!("{:?}", atlas);
        // timer.tick(time.delta());
        // if timer.just_finished() {
        //     sprite.index = if sprite.index == indices.last {
        //         indices.first
        //     } else {
        sprite.index = 11;
        //     };
        // }
    }
}
