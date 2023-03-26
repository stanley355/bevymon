use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerSpriteIndices {
    pub first: usize,
    pub last: usize,
}

const PLAYER_SPRITE_WIDTH: f32 = 20.0;
const PLAYER_SPRITE_HEIGHT: f32 = 27.0;
const PLAYER_TILE_SIZE: Vec2 = Vec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT);
const PLAYER_SPRITE_OFFSET: Option<Vec2> = Some(Vec2::new(5.0, 4.0));

#[derive(Debug, Component)]
pub struct Player;

impl Player {
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) {
        let player_sprites: Handle<Image> = asset_server.load("player_sprites.png");
        let texture_atlas = TextureAtlas::from_grid(
            player_sprites,
            PLAYER_TILE_SIZE,
            3,
            4,
            None,
            PLAYER_SPRITE_OFFSET,
        );
        let texture_atlas_handle = texture_atlas_res.add(texture_atlas);
        let sprite_indices = PlayerSpriteIndices { first: 1, last: 3 };

        commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(sprite_indices.first),
                    transform: Transform::from_scale(Vec3::new(3.0, 3.0, 0.0)),
                    ..default()
                },
                sprite_indices,
            ))
            .insert(Player);
    }

    pub fn move_player(
        mut query: Query<(
            &mut Player,
            &mut PlayerSpriteIndices,
            &mut TextureAtlasSprite,
        )>,
    ) {
        for (_player, mut _sprite_indices, mut sprite) in &mut query {
            if sprite.index < 11 {
                sprite.index += 1;
            } else {
                sprite.index = 0;
            }
        }
    }
}
