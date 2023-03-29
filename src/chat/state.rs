use bevy::prelude::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ChatState {
    #[default]
    OffChat, //Splash screen and Menu
    OnChat,
}