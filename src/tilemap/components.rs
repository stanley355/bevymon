use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Tile;

impl Tile {
    pub fn texture_atlas_handle(
        asset_server: Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) -> Handle<TextureAtlas> {
        let tile_image: Handle<Image> = asset_server.load("images/tiles/dis_world_tiles.png");

        let texture_atlas = TextureAtlas::from_grid(
            tile_image,
            Vec2::new(20.0, 20.0),
            9,
            7,
            None,
            None,
        );
        return texture_atlas_res.add(texture_atlas);
    }
}