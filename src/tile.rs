
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Tile {
    size: Vec2,
    position: Vec2,
}

impl Tile {
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) {
        let player_sprites: Handle<Image> = asset_server.load("player_sprites.png");
        let texture_atlas = TextureAtlas::from_grid(
            player_sprites,
            Vec2::new(16.0, 16.0),
            4,
            1,
            None,
            Some(Vec2::new(6.0, 64.0)),
        );
        let texture_atlas_handle = texture_atlas_res.add(texture_atlas);

        commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform::from_scale(Vec3::new(2.0, 2.0, 0.0)),
                    ..default()
                },
            ));
    }

}