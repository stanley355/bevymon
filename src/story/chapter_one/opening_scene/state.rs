use bevy::prelude::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum OpeningSceneState {
    #[default]
    NoImg,
    EggImg,
    MewImg,
    ArceusImg,
    WorldMapImg,
    CleanupImg,
}

