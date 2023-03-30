use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct BackgroundSceneSprite;

impl BackgroundSceneSprite {
    pub fn poke_image(asset_server: &Res<AssetServer>, image_path: &str) -> ImageBundle {
        let background = asset_server.load(image_path);

        let bundle = ImageBundle {
            style: Style {
                margin: UiRect::left(Val::Percent(-25.)),
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
}
