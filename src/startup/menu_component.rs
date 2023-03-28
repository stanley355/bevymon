use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct MenuComponent;

impl MenuComponent {
    pub fn background(asset_server: &Res<AssetServer>, window: &Window) -> ImageBundle {
        let background = asset_server.load("images/war_bg.png");
        let size = Size::new(Val::Px(window.width()), Val::Px(window.height()));

        let bundle = ImageBundle {
            style: Style { size, ..default() },
            image: UiImage::new(background),
            ..default()
        };

        return bundle;
    }
}
