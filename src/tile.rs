use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Tile {
    size: Vec2,
    position: Vec2,
}

impl Tile {
    fn get_tile_atlas_handle(
        asset_server: Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) -> Handle<TextureAtlas> {
        let player_sprites: Handle<Image> = asset_server.load("tiles.png");
        let texture_atlas = TextureAtlas::from_grid(
            player_sprites,
            Vec2::new(16.0, 16.0),
            4,
            1,
            None,
            Some(Vec2::new(6.0, 64.0)),
        );
        return texture_atlas_res.add(texture_atlas);
    }

    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) {
        let texture_atlas = Self::get_tile_atlas_handle(asset_server, texture_atlas_res);

        for m in -10..2 {
            for n in -5..5 {
                let transform = Transform {
                    translation: Vec3::new(m as f32 * 30.0, n as f32 * 30.0, 10.0),
                    scale: Vec3::new(2.0, 2.0, 0.0),
                    ..default()
                };
                commands.spawn((SpriteSheetBundle {
                    texture_atlas: texture_atlas.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: transform,
                    ..default()
                },));
            }
        }
    }
}
