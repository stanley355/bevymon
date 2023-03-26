use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

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
        Vec2::new(14.0, 21.0),
        7,
        1,
        None,
        Some(Vec2::new(25.0, 35.0)),
    );
    let texture_atlas_handle = texture_atlas_res.add(texture_atlas);

    let animation_indices = AnimationIndices { first: 0, last: 6 };

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                // transform: Transform::from_scale(Vec3::splat(6.0)),
                transform: Transform::from_scale(Vec3::new(3.0, 3.0, 0.0)),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(Player);

    // commands
    //     .spawn(SpriteSheetBundle {
    //         texture_atlas: texture_atlas_handle,
    //         sprite: TextureAtlasSprite::new(animation_indices.first),
    //         transform: Transform::from_scale(Vec3::new(3.0, 3.0, 0.0)),
    //         ..default()
    //     })
    //     .insert(Player);
    // commands.spawn(animation_indices);
}

pub fn move_player(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    // println!("hihiih");
    for (indices, mut timer, mut sprite) in &mut query  {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}
