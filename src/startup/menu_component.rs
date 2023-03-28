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

    pub fn logo_wrap(window: &Window) -> NodeBundle {
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::left(Val::Px(window.width() / 4.)),
                ..default()
            },
            ..default()
        }
    }

    pub fn logo(asset_server: &Res<AssetServer>) -> TextBundle {
        let font: Handle<Font> = asset_server.load("fonts/poke-solid.ttf");
        TextBundle::from_section(
            "Pokemon",
            TextStyle {
                font: font.clone(),
                font_size: 175.0,
                color: Color::YELLOW,
            },
        )
    }

    pub fn title(asset_server: &Res<AssetServer>) -> TextBundle {
        let font: Handle<Font> = asset_server.load("fonts/poke-solid.ttf");
        TextBundle::from_section(
            "War of Origin",
            TextStyle {
                font: font.clone(),
                font_size: 120.0,
                color: Color::WHITE,
            },
        )
    }
}
