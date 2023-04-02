use bevy::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::components::Tile;

#[derive(Debug)]
pub struct TileMap;

impl TileMap {
    pub fn new(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) {
        let tile_atlas = Tile::texture_atlas(asset_server, texture_atlas_res);

        let file = File::open("assets/maps/dis.txt").expect("No map file found");

        for (y, line) in BufReader::new(file).lines().enumerate() {
            println!("{:?}", line.unwrap().split(" ").collect::<Vec<_>>())
            // if let Ok(line) = line {
            //     for (x, char) in line.chars().enumerate() {
            //         println!("{:?}", char);
            //         let transform = Transform::from_xyz(x as f32 * 20., y as f32 * 20., 1.0);

            //         let bundle = Tile::bundle(tile_atlas.clone(), 3, transform);

            //         commands.spawn(bundle);
            //     }
            // }
        }
    }
}
