use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct OpeningSceneSprite;

impl OpeningSceneSprite {
    pub fn new(asset_server: &Res<AssetServer>, image_path: &str) -> ImageBundle {
        let background = asset_server.load(image_path);

        let bundle = ImageBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Percent(-25.),
                    bottom: Val::Percent(5.),
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(0.5, 0.5, 0.),
                ..Default::default()
            },
            image: UiImage::new(background),
            ..default()
        };

        return bundle;
    }

    pub fn world_map(asset_server: &Res<AssetServer>) -> ImageBundle {
        let background = asset_server.load("images/world_map.png");

        let bundle = ImageBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Percent(-80.),
                    bottom: Val::Percent(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(0.5, 0.75, 0.),
                ..Default::default()
            },
            image: UiImage::new(background),
            ..default()
        };

        return bundle;
    }
}
