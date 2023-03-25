use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Player;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
) {
    let player_sprites: Handle<Image> = asset_server.load("player_sprites.png");
    let texture_atlas =
        TextureAtlas::from_grid(player_sprites, Vec2::new(25.0, 25.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlas_res.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        ..default()
    });
}
