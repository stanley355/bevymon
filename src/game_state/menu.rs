use super::GameState;
use bevy::prelude::*;

#[derive(Debug)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(MenuScreen::view.in_schedule(OnEnter(GameState::MenuScreen)));
    }
}

#[derive(Debug, Component)]
pub struct MenuScreen;

impl MenuScreen {
    fn view(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        query: Query<&Window>,
        game_state: Res<State<GameState>>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        println!("{:?}", game_state.0);
        if game_state.0 == GameState::MenuScreen {
            let window = query.single();

            let background = asset_server.load("images/war_bg.png");
            let size = Size::new(Val::Px(window.width()), Val::Px(window.height()));

            let bundle = ImageBundle {
                style: Style { size, ..default() },
                image: UiImage::new(background),
                ..default()
            };

            let name = Name::new("menu_bg");

            commands.spawn((bundle, name));
            next_state.set(GameState::Menu);
        }
    }
}
