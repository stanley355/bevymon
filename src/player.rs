use bevy::prelude::*;

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

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, 0.0)),
            ..default()
        })
        .insert(Player);
}

pub fn move_player(
    mut player_entity: Query<(&Player, &mut Handle<TextureAtlas>, &mut TextureAtlasSprite,)>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
) {


    for (_player, mut text_atlas_handle, mut sprite) in player_entity.iter_mut(){
        println!("{:?}", _player)
    }
}
