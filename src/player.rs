use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

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
        1,
        1,
        None,
        Some(Vec2::new(25.0, 35.0)),
    );
    let texture_atlas_handle = texture_atlas_res.add(texture_atlas);

    let animation_indices = AnimationIndices { first: 0, last: 6 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, 0.0)),
            ..default()
        },
        animation_indices,
    ));

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
    mut player_entity: Query<(&Player, &AnimationIndices, &mut TextureAtlasSprite)>,
) {
    println!("hihiih");
    for (indices, mut timer, mut sprite) in player_entity.iter_mut() {
        sprite.index += 1;
        println!("{:?}", sprite.index);
        // timer.tick(time.delta());
        // if timer.just_finished() {
        //     sprite.index = if sprite.index == indices.last {
        //         indices.first
        //     } else {
        //         sprite.index + 1
        //     };
        // }
    }
    // for (_player, mut text_atlas_handle, mut sprite) in player_entity.iter_mut() {
    //     println!("{:?}", _player)
    // }
}
